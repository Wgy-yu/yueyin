use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::crypto::{eapi_encrypt, weapi_encrypt};
use super::http_client::client;

const WEAPI_BASE: &str = "https://music.163.com/weapi";
const EAPI_BASE: &str = "https://interface.music.163.com/eapi";

// ---------- Shared types ----------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NeteaseSong {
    pub provider: String,
    pub source: String,
    #[serde(rename = "type")]
    pub song_type: String,
    pub id: u64,
    pub name: String,
    pub artist: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "artistId")]
    pub artist_id: Option<u64>,
    pub album: String,
    pub cover: String,
    pub duration: u64,
    pub fee: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    pub id: Option<u64>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongUrlInfo {
    pub provider: String,
    pub url: Option<String>,
    pub trial: bool,
    pub playable: bool,
    pub level: String,
    pub quality: String,
    pub br: Option<u64>,
    #[serde(rename = "requestedQuality")]
    pub requested_quality: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LyricResult {
    pub provider: String,
    pub lyric: String,
    pub tlyric: String,
    pub yrc: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    #[serde(rename = "loggedIn")]
    pub logged_in: bool,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub nickname: String,
    pub avatar: String,
    #[serde(rename = "vipType")]
    pub vip_type: i64,
    #[serde(rename = "vipLevel")]
    pub vip_level: String,
    #[serde(rename = "isVip")]
    pub is_vip: bool,
    #[serde(rename = "isSvip")]
    pub is_svip: bool,
    #[serde(rename = "vipLabel")]
    pub vip_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasCookie")]
    pub has_cookie: Option<bool>,
}

// ---------- Quality constants ----------

struct QualityCandidate {
    level: &'static str,
    br: u64,
    label: &'static str,
    svip: bool,
}

const QUALITIES: &[QualityCandidate] = &[
    QualityCandidate {
        level: "jymaster",
        br: 1999000,
        label: "超清母带",
        svip: true,
    },
    QualityCandidate {
        level: "hires",
        br: 1999000,
        label: "高清臻音",
        svip: false,
    },
    QualityCandidate {
        level: "lossless",
        br: 1411000,
        label: "无损",
        svip: false,
    },
    QualityCandidate {
        level: "exhigh",
        br: 999000,
        label: "极高",
        svip: false,
    },
    QualityCandidate {
        level: "standard",
        br: 128000,
        label: "标准",
        svip: false,
    },
];

fn quality_from(pref: &str) -> usize {
    match pref {
        "jymaster" | "master" | "studio" | "svip" => 0,
        "hires" | "hi-res" | "highres" => 1,
        "lossless" | "flac" | "sq" => 2,
        "exhigh" | "high" | "320" | "320k" => 3,
        "standard" | "normal" | "128" | "128k" => 4,
        _ => 1,
    }
}

// ---------- Helpers ----------

fn map_artists(raw: &Value) -> Vec<Artist> {
    let arr = match raw.as_array() {
        Some(a) => a,
        None => return vec![],
    };
    arr.iter()
        .filter_map(|a| {
            let name = a["name"].as_str().unwrap_or("").to_string();
            if name.is_empty() {
                None
            } else {
                Some(Artist {
                    id: a["id"].as_u64(),
                    name,
                })
            }
        })
        .collect()
}

fn map_song(s: &Value) -> NeteaseSong {
    let artists = map_artists(&s["ar"]);
    let album = &s["al"];
    let cover = album["picUrl"]
        .as_str()
        .or_else(|| album["coverUrl"].as_str())
        .unwrap_or("")
        .to_string();
    NeteaseSong {
        provider: "netease".into(),
        source: "netease".into(),
        song_type: "song".into(),
        id: s["id"].as_u64().unwrap_or(0),
        name: s["name"].as_str().unwrap_or("").to_string(),
        artist: artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(" / "),
        artists,
        artist_id: s["ar"]
            .as_array()
            .and_then(|a| a.first())
            .and_then(|a| a["id"].as_u64()),
        album: album["name"].as_str().unwrap_or("").to_string(),
        cover,
        duration: s["dt"].as_u64().or(s["duration"].as_u64()).unwrap_or(0),
        fee: s["fee"].as_i64(),
    }
}

async fn weapi_post(endpoint: &str, body: &Value, cookie: &str) -> Result<Value, String> {
    let csrf = extract_csrf(cookie);
    let mut payload = body.clone();
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("csrf_token".into(), json!(csrf));
    }
    let text = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let (params, enc_sec_key) = weapi_encrypt(&text);
    let form = [
        ("params", params.as_str()),
        ("encSecKey", enc_sec_key.as_str()),
    ];
    let url = format!("{}/{}", WEAPI_BASE, endpoint);
    let resp = client()
        .post(&url)
        .header("Referer", "https://music.163.com/")
        .header("Origin", "https://music.163.com")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", cookie)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("请求网易云 API 失败: {}", e))?;
    let text = resp
        .text()
        .await
        .map_err(|e| format!("读取网易云响应失败: {}", e))?;
    serde_json::from_str::<Value>(&text).map_err(|e| {
        let preview: String = text.chars().take(120).collect();
        format!("解析网易云响应失败: {e}; 响应片段: {preview}")
    })
}

fn extract_csrf(cookie: &str) -> String {
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("__csrf=") {
            return val.trim().to_string();
        }
    }
    String::new()
}

fn cookie_field(cookie: &str, name: &str) -> String {
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix(name).and_then(|s| s.strip_prefix('=')) {
            return val.trim().to_string();
        }
    }
    String::new()
}

fn eapi_header(cookie: &str) -> Value {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());
    let mut header = json!({
        "os": "iPhone OS",
        "appver": "9.0.90",
        "versioncode": "140",
        "osver": "16.2",
        "channel": "distribution",
        "buildver": now,
        "__csrf": extract_csrf(cookie),
    });
    if let Some(obj) = header.as_object_mut() {
        let music_u = cookie_field(cookie, "MUSIC_U");
        let music_a = cookie_field(cookie, "MUSIC_A");
        if !music_u.is_empty() {
            obj.insert("MUSIC_U".into(), json!(music_u));
        }
        if !music_a.is_empty() {
            obj.insert("MUSIC_A".into(), json!(music_a));
        }
    }
    header
}

async fn eapi_post(uri: &str, body: &Value, cookie: &str) -> Result<Value, String> {
    let mut payload = body.clone();
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("header".into(), eapi_header(cookie));
    }
    let text = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let params = eapi_encrypt(uri, &text);
    let path = uri
        .strip_prefix("/api/")
        .ok_or_else(|| format!("无效 eapi 路径: {uri}"))?;
    let url = format!("{EAPI_BASE}/{path}");
    let resp = client()
        .post(&url)
        .header(
            "User-Agent",
            "NeteaseMusic/9.0.90.240122161425(9009000);Dalvik/2.1.0",
        )
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", cookie)
        .form(&[("params", params.as_str())])
        .send()
        .await
        .map_err(|e| format!("请求网易云 eapi 失败: {}", e))?;
    let text = resp
        .text()
        .await
        .map_err(|e| format!("读取网易云 eapi 响应失败: {}", e))?;
    serde_json::from_str::<Value>(&text).map_err(|e| {
        let preview: String = text.chars().take(120).collect();
        format!("解析网易云 eapi 响应失败: {e}; 响应片段: {preview}")
    })
}

async fn song_detail(ids: &[u64], cookie: &str) -> Result<Value, String> {
    let c = format!(
        "[{}]",
        ids.iter()
            .map(|id| format!("{{\"id\":{id}}}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    weapi_post("v3/song/detail", &json!({ "c": c }), cookie).await
}

// ---------- Public API ----------

/// Search songs via Netease cloudsearch.
pub async fn search(keywords: &str, limit: u32, cookie: &str) -> Result<Vec<NeteaseSong>, String> {
    let body = json!({
        "s": keywords,
        "type": 1,
        "limit": limit,
        "offset": 0,
        "total": true,
    });
    let resp = eapi_post("/api/cloudsearch/pc", &body, cookie).await?;
    let songs_val = resp
        .pointer("/result/songs")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut mapped: Vec<NeteaseSong> = songs_val.iter().map(map_song).collect();

    // Backfill missing covers via song_detail
    let missing: Vec<u64> = mapped
        .iter()
        .filter(|s| s.cover.is_empty())
        .map(|s| s.id)
        .collect();
    if !missing.is_empty() {
        if let Ok(detail_resp) = song_detail(&missing, cookie).await {
            if let Some(songs) = detail_resp["songs"].as_array() {
                let id_to_pic: std::collections::HashMap<u64, String> = songs
                    .iter()
                    .filter_map(|s| {
                        let id = s["id"].as_u64()?;
                        let pic = s["al"]["picUrl"]
                            .as_str()
                            .or(s["album"]["picUrl"].as_str())?;
                        if pic.is_empty() {
                            None
                        } else {
                            Some((id, pic.to_string()))
                        }
                    })
                    .collect();
                for song in &mut mapped {
                    if song.cover.is_empty() {
                        if let Some(pic) = id_to_pic.get(&song.id) {
                            song.cover = pic.clone();
                        }
                    }
                }
            }
        }
    }

    Ok(mapped)
}

/// Get song playback URL. Tries highest quality first, falls back.
pub async fn song_url(
    id: u64,
    quality_pref: &str,
    cookie: &str,
    is_svip: bool,
) -> Result<SongUrlInfo, String> {
    let start = quality_from(quality_pref);
    let mut trial_fallback: Option<SongUrlInfo> = None;

    for q in QUALITIES.iter().skip(start) {
        if q.svip && !is_svip {
            continue;
        }
        // Try v1 endpoint first
        let result = try_song_url_v1(id, q.level, cookie)
            .await
            .or(try_song_url(id, q.br, cookie).await);
        match result {
            Ok(d) => {
                let url = d["url"].as_str().map(|s| s.to_string());
                let has_trial = d["freeTrialInfo"].is_object();
                if let Some(ref u) = url {
                    if !u.is_empty() && !has_trial {
                        return Ok(SongUrlInfo {
                            provider: "netease".into(),
                            url: Some(u.clone()),
                            trial: false,
                            playable: true,
                            level: q.level.to_string(),
                            quality: q.label.to_string(),
                            br: d["br"].as_u64(),
                            requested_quality: quality_pref.to_string(),
                            restriction: None,
                            error: None,
                            message: None,
                        });
                    }
                    if !u.is_empty() && has_trial && trial_fallback.is_none() {
                        trial_fallback = Some(SongUrlInfo {
                            provider: "netease".into(),
                            url: Some(u.clone()),
                            trial: true,
                            playable: true,
                            level: q.level.to_string(),
                            quality: q.label.to_string(),
                            br: d["br"].as_u64(),
                            requested_quality: quality_pref.to_string(),
                            restriction: Some(classify_restriction(&d, cookie)),
                            error: None,
                            message: None,
                        });
                    }
                }
            }
            Err(_) => continue,
        }
    }

    if let Some(trial) = trial_fallback {
        return Ok(trial);
    }

    Ok(SongUrlInfo {
        provider: "netease".into(),
        url: None,
        trial: false,
        playable: false,
        level: String::new(),
        quality: String::new(),
        br: None,
        requested_quality: quality_pref.to_string(),
        restriction: Some(json!({
            "provider": "netease",
            "category": "url_unavailable",
            "message": "网易云没有返回可播放地址，可能是版权、会员或地区限制",
        })),
        error: None,
        message: Some("没有可播放地址".into()),
    })
}

async fn try_song_url_v1(id: u64, level: &str, cookie: &str) -> Result<Value, String> {
    let body = json!({
        "ids": format!("[{id}]"),
        "level": level,
        "encodeType": "flac",
    });
    let resp = eapi_post("/api/song/enhance/player/url/v1", &body, cookie).await?;
    let d = resp.pointer("/data/0").cloned().unwrap_or(Value::Null);
    if d.is_null() || d["url"].as_str().is_none() {
        return Err("no url".into());
    }
    Ok(d)
}

async fn try_song_url(id: u64, br: u64, cookie: &str) -> Result<Value, String> {
    let body = json!({
        "ids": format!("[\"{id}\"]"),
        "br": br,
    });
    let resp = eapi_post("/api/song/enhance/player/url", &body, cookie).await?;
    let d = resp.pointer("/data/0").cloned().unwrap_or(Value::Null);
    if d.is_null() || d["url"].as_str().is_none() {
        return Err("no url".into());
    }
    Ok(d)
}

fn classify_restriction(data: &Value, cookie: &str) -> Value {
    let logged_in = !cookie.is_empty();
    let fee = data["fee"].as_i64().unwrap_or(0);
    let code = data["code"].as_i64().unwrap_or(0);
    let has_trial = data["freeTrialInfo"].is_object();

    if !logged_in {
        return json!({
            "provider": "netease",
            "category": "login_required",
            "message": "网易云需要登录后尝试获取完整播放地址",
            "action": "login",
        });
    }
    if has_trial {
        return json!({
            "provider": "netease",
            "category": "trial_only",
            "message": "网易云仅返回试听片段，完整播放需要会员或购买",
            "action": "upgrade",
        });
    }
    if fee == 1 {
        return json!({
            "provider": "netease",
            "category": "vip_required",
            "message": "网易云歌曲需要 VIP 权限",
            "action": "upgrade",
        });
    }
    if code == 404 || code == 403 {
        return json!({
            "provider": "netease",
            "category": "copyright_unavailable",
            "message": "网易云版权暂不可播",
            "action": "switch_source",
        });
    }
    json!({
        "provider": "netease",
        "category": "url_unavailable",
        "message": "网易云没有返回可播放地址",
        "action": if logged_in { "switch_source" } else { "login" },
    })
}

/// Fetch lyrics for a Netease song.
pub async fn lyric(id: u64, cookie: &str) -> Result<LyricResult, String> {
    // Try lyric_new first
    let body = json!({
        "id": id,
        "lv": -1,
        "tv": -1,
        "kv": -1,
        "rv": -1,
    });
    let resp = eapi_post(
        "/api/song/lyric/v1",
        &json!({
            "id": id,
            "cp": false,
            "tv": 0,
            "lv": 0,
            "rv": 0,
            "kv": 0,
            "yv": 0,
            "ytv": 0,
            "yrv": 0,
        }),
        cookie,
    )
    .await
    .or(eapi_post("/api/song/lyric", &body, cookie).await)?;

    let lrc = resp["lrc"]["lyric"].as_str().unwrap_or("").to_string();
    let tlyric = resp["tlyric"]["lyric"].as_str().unwrap_or("").to_string();
    let yrc = resp["yrc"]["lyric"].as_str().unwrap_or("").to_string();

    Ok(LyricResult {
        provider: "netease".into(),
        lyric: lrc,
        tlyric,
        yrc,
        source: "lyric".into(),
    })
}

/// Get login status by checking the cookie.
pub async fn login_status(cookie: &str) -> LoginInfo {
    let has_music_u = cookie
        .split(';')
        .any(|part| part.trim().starts_with("MUSIC_U="));
    if cookie.is_empty() {
        return LoginInfo {
            logged_in: false,
            user_id: None,
            nickname: String::new(),
            avatar: String::new(),
            vip_type: 0,
            vip_level: "none".into(),
            is_vip: false,
            is_svip: false,
            vip_label: "无VIP".into(),
            has_cookie: Some(false),
        };
    }

    let body = json!({});
    let resp = weapi_post("w/nuser/account/get", &body, cookie)
        .await
        .or(weapi_post("nuser/account/get", &body, cookie).await);
    match resp {
        Ok(resp) => {
            let profile = &resp["profile"];
            let account = &resp["account"];
            let user_id = profile["userId"]
                .as_str()
                .map(|s| s.to_string())
                .or_else(|| profile["userId"].as_u64().map(|id| id.to_string()))
                .or_else(|| account["id"].as_u64().map(|id| id.to_string()));
            let logged_in = user_id.is_some() || has_music_u;
            let nickname = profile["nickname"]
                .as_str()
                .unwrap_or(if has_music_u {
                    "网易云音乐用户"
                } else {
                    "网易云用户"
                })
                .to_string();
            let avatar = profile["avatarUrl"].as_str().unwrap_or("").to_string();
            let vip_type = account["vipType"].as_i64().unwrap_or(0);
            let is_svip = vip_type >= 10;
            let is_vip = is_svip || vip_type > 0;
            LoginInfo {
                logged_in,
                user_id,
                nickname,
                avatar,
                vip_type,
                vip_level: if is_svip {
                    "svip".into()
                } else if is_vip {
                    "vip".into()
                } else {
                    "none".into()
                },
                is_vip,
                is_svip,
                vip_label: if is_svip {
                    "SVIP".into()
                } else if is_vip {
                    "VIP".into()
                } else {
                    "无VIP".into()
                },
                has_cookie: Some(true),
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("[netease-login] account info failed: {e}");
            LoginInfo {
                logged_in: has_music_u,
                user_id: None,
                nickname: if has_music_u {
                    "网易云音乐用户".into()
                } else {
                    String::new()
                },
                avatar: String::new(),
                vip_type: 0,
                vip_level: "none".into(),
                is_vip: false,
                is_svip: false,
                vip_label: "无VIP".into(),
                has_cookie: Some(has_music_u),
            }
        }
    }
}

/// Get QR login key.
pub async fn qr_key() -> Result<String, String> {
    let body = json!({});
    let resp = weapi_post("login/qrcode/unikey", &body, "").await?;
    resp["unikey"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "获取二维码 key 失败".into())
}

/// Create QR code image (base64).
pub async fn qr_create(key: &str) -> Result<(String, String), String> {
    let url = format!("https://music.163.com/login?codekey={}", key);
    // Generate QR via external service
    let qr_url = format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}",
        urlencoding::encode(&url)
    );
    let resp = client()
        .get(&qr_url)
        .send()
        .await
        .map_err(|e| format!("获取二维码图片失败: {}", e))?;
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    let img = format!("data:image/png;base64,{}", BASE64.encode(&bytes));
    Ok((img, url))
}

/// QR check result with extracted cookie.
pub struct QrCheckResult {
    pub body: Value,
    pub cookie: String,
}

/// Poll QR login status, preserving Set-Cookie headers.
pub async fn qr_check_with_cookies(
    key: &str,
    existing_cookie: &str,
) -> Result<QrCheckResult, String> {
    let csrf = extract_csrf(existing_cookie);
    let mut payload = json!({ "key": key, "type": 1 });
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("csrf_token".into(), json!(csrf));
    }
    let text = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let (params, enc_sec_key) = weapi_encrypt(&text);
    let form = [
        ("params", params.as_str()),
        ("encSecKey", enc_sec_key.as_str()),
    ];
    let url = format!("{}/{}", WEAPI_BASE, "login/qrcode/client/login");
    let resp = client()
        .post(&url)
        .header("Referer", "https://music.163.com/")
        .header("Origin", "https://music.163.com")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", existing_cookie)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("二维码登录请求失败: {}", e))?;

    // Extract Set-Cookie headers before consuming body
    let mut cookie_parts: Vec<String> = Vec::new();
    for val in resp.headers().get_all("set-cookie") {
        if let Ok(s) = val.to_str() {
            // Keep only name=value, strip attributes
            let name_value = s.split(';').next().unwrap_or("").trim();
            if !name_value.is_empty() {
                cookie_parts.push(name_value.to_string());
            }
        }
    }

    // Deduplicate by cookie name
    let mut seen = std::collections::HashSet::new();
    cookie_parts.retain(|c| {
        let name = c.split('=').next().unwrap_or("");
        seen.insert(name.to_string())
    });

    let body = resp
        .json::<Value>()
        .await
        .map_err(|e| format!("解析二维码响应失败: {}", e))?;

    // Merge with existing cookie
    let mut merged = existing_cookie.to_string();
    for part in &cookie_parts {
        if !merged.contains(&format!("{}=", part.split('=').next().unwrap_or(""))) {
            if !merged.is_empty() {
                merged.push_str("; ");
            }
            merged.push_str(part);
        }
    }

    Ok(QrCheckResult {
        body,
        cookie: merged,
    })
}

/// Fetch user's playlists.
pub async fn user_playlists(uid: &str, cookie: &str) -> Result<Value, String> {
    let owned_uid;
    let uid = if uid.is_empty() {
        let info = login_status(cookie).await;
        owned_uid = info.user_id.unwrap_or_default();
        owned_uid.as_str()
    } else {
        uid
    };
    if uid.is_empty() {
        return Ok(json!({ "loggedIn": false, "playlists": [] }));
    }
    let body = json!({
        "uid": uid,
        "limit": 100,
        "offset": 0,
    });
    let resp = weapi_post("user/playlist", &body, cookie).await?;
    let playlists = resp["playlist"].as_array().cloned().unwrap_or_default();
    let mapped: Vec<Value> = playlists
        .iter()
        .map(|p| {
            json!({
                "id": p["id"].as_u64().unwrap_or(0).to_string(),
                "name": p["name"].as_str().unwrap_or(""),
                "cover": p["coverImgUrl"].as_str().unwrap_or(""),
                "trackCount": p["trackCount"].as_u64().unwrap_or(0),
                "playCount": p["playCount"].as_u64().unwrap_or(0),
                "creator": p["creator"]["nickname"].as_str().unwrap_or(""),
                "subscribed": p["subscribed"].as_bool().unwrap_or(false),
                "specialType": p["specialType"].as_i64().unwrap_or(0),
                "provider": "netease",
            })
        })
        .collect();
    Ok(json!({ "loggedIn": true, "userId": uid, "playlists": mapped }))
}

/// Fetch tracks in a playlist.
pub async fn playlist_tracks(id: &str, cookie: &str) -> Result<Value, String> {
    let song_id: u64 = id.parse().map_err(|_| "Invalid playlist id")?;
    let body = json!({
        "id": song_id,
        "n": 100000,
        "s": 8,
    });
    let resp = eapi_post("/api/v6/playlist/detail", &body, cookie).await?;
    let playlist = &resp["playlist"];
    let track_ids: Vec<u64> = playlist["trackIds"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|t| t["id"].as_u64()).collect())
        .unwrap_or_default();

    let info = json!({
        "id": playlist["id"].as_u64().unwrap_or(0).to_string(),
        "name": playlist["name"].as_str().unwrap_or(""),
        "cover": playlist["coverImgUrl"].as_str().unwrap_or(""),
        "trackCount": playlist["trackCount"].as_u64().unwrap_or(0),
    });

    // Fetch full track details in batches
    let mut all_tracks: Vec<Value> = Vec::new();
    for chunk in track_ids.chunks(500) {
        if let Ok(detail_resp) = song_detail(chunk, cookie).await {
            if let Some(songs) = detail_resp["songs"].as_array() {
                for s in songs {
                    all_tracks.push(map_song_to_value(s));
                }
            }
        }
    }

    Ok(json!({ "playlist": info, "tracks": all_tracks }))
}

fn map_song_to_value(s: &Value) -> Value {
    let artists = map_artists(&s["ar"]);
    let album = &s["al"];
    let cover = album["picUrl"]
        .as_str()
        .or_else(|| album["coverUrl"].as_str())
        .unwrap_or("")
        .to_string();
    json!({
        "id": s["id"].as_u64().unwrap_or(0).to_string(),
        "name": s["name"].as_str().unwrap_or(""),
        "artist": artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(" / "),
        "album": album["name"].as_str().unwrap_or(""),
        "cover": cover,
        "duration": s["dt"].as_u64().or(s["duration"].as_u64()).unwrap_or(0),
        "source": "netease",
    })
}

/// Batch check liked status.
pub async fn like_check(ids: &[u64], uid: &str, cookie: &str) -> Result<Value, String> {
    // Use likelist endpoint
    let body = json!({ "uid": uid });
    let resp = eapi_post("/api/song/like/get", &body, cookie).await?;
    let liked_ids: std::collections::HashSet<u64> = resp["ids"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
        .unwrap_or_default();
    let result: Value = ids
        .iter()
        .map(|id| (id.to_string(), json!(liked_ids.contains(id))))
        .collect::<serde_json::Map<String, Value>>()
        .into();
    Ok(json!({ "liked": result }))
}

/// Toggle like status for a song.
pub async fn like_toggle(id: u64, like: bool, cookie: &str) -> Result<Value, String> {
    let body = json!({ "trackIds": format!("[{id}]") });
    let _ = eapi_post("/api/song/like/check", &body, cookie).await.ok();
    let body = json!({
        "alg": "itembased",
        "trackId": id,
        "like": like,
        "time": "3",
    });
    let resp = weapi_post("radio/like", &body, cookie).await?;
    Ok(json!({ "ok": resp["code"].as_i64() == Some(200) }))
}
