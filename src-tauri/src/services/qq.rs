use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::http_client::client;

const MUSICU_URL: &str = "https://u.y.qq.com/cgi-bin/musicu.fcg";
const SMARTBOX_URL: &str = "https://c.y.qq.com/splcloud/fcgi-bin/smartbox_new.fcg";

const QQ_HEADERS_REFERER: &str = "https://y.qq.com/";

// ---------- Types ----------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QQSong {
    pub provider: String,
    pub source: String,
    #[serde(rename = "type")]
    pub song_type: String,
    pub id: String,
    #[serde(rename = "qqId", skip_serializing_if = "Option::is_none")]
    pub qq_id: Option<String>,
    pub mid: String,
    #[serde(rename = "songmid")]
    pub songmid: String,
    #[serde(rename = "mediaMid", skip_serializing_if = "Option::is_none")]
    pub media_mid: Option<String>,
    pub name: String,
    pub artist: String,
    pub artists: Vec<QQArtist>,
    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    #[serde(rename = "artistMid", skip_serializing_if = "Option::is_none")]
    pub artist_mid: Option<String>,
    pub album: String,
    #[serde(rename = "albumMid", skip_serializing_if = "Option::is_none")]
    pub album_mid: Option<String>,
    pub cover: String,
    pub duration: u64,
    pub fee: i64,
    pub playable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QQArtist {
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QQSongUrlInfo {
    pub provider: String,
    pub url: String,
    pub trial: bool,
    pub playable: bool,
    pub level: String,
    pub quality: String,
    pub filename: String,
    #[serde(rename = "requestedQuality")]
    pub requested_quality: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "loggedIn")]
    pub logged_in: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QQLyricResult {
    pub provider: String,
    pub mid: String,
    pub lyric: String,
    pub tlyric: String,
    pub yrc: String,
    pub qrc: String,
    pub roma: String,
    pub source: String,
}

// ---------- Quality templates ----------

struct QQualityTemplate {
    prefix: &'static str,
    ext: &'static str,
    level: &'static str,
    label: &'static str,
}

const QQ_QUALITIES: &[QQualityTemplate] = &[
    QQualityTemplate { prefix: "RS01", ext: ".flac", level: "hires", label: "Hi-Res FLAC" },
    QQualityTemplate { prefix: "F000", ext: ".flac", level: "lossless", label: "无损 FLAC" },
    QQualityTemplate { prefix: "M800", ext: ".mp3",  level: "exhigh", label: "320k MP3" },
    QQualityTemplate { prefix: "M500", ext: ".mp3",  level: "standard", label: "128k MP3" },
    QQualityTemplate { prefix: "C400", ext: ".m4a",  level: "aac", label: "AAC/M4A" },
];

fn qq_quality_from(pref: &str) -> usize {
    match pref {
        "hires" | "hi-res" => 0,
        "lossless" | "flac" | "sq" => 1,
        "exhigh" | "high" | "320" | "320k" => 2,
        "standard" | "normal" | "128" | "128k" => 3,
        "aac" | "m4a" => 4,
        _ => 0,
    }
}

// ---------- Helpers ----------

fn map_qq_artists(raw: &Value) -> Vec<QQArtist> {
    let arr = match raw.as_array() {
        Some(a) => a,
        None => return vec![],
    };
    arr.iter()
        .filter_map(|a| {
            let name = a["name"]
                .as_str()
                .or(a["title"].as_str())
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                None
            } else {
                Some(QQArtist {
                    id: a["id"].as_u64(),
                    mid: a["mid"].as_str().map(|s| s.to_string()),
                    name,
                })
            }
        })
        .collect()
}

fn qq_album_cover(album_mid: &str, size: u32) -> String {
    if album_mid.is_empty() {
        return String::new();
    }
    format!(
        "https://y.qq.com/music/photo_new/T002R{}x{}M000{}.jpg?max_age=2592000",
        size, size, album_mid
    )
}

fn map_qq_smart_song(item: &Value) -> QQSong {
    let mid = item["mid"]
        .as_str()
        .or(item["songmid"].as_str())
        .or(item["id"].as_str())
        .unwrap_or("")
        .to_string();
    let singer = item["singer"].as_str().unwrap_or("");
    QQSong {
        provider: "qq".into(),
        source: "qq".into(),
        song_type: "qq".into(),
        id: mid.clone(),
        qq_id: item["id"].as_str().map(|s| s.to_string()),
        mid: mid.clone(),
        songmid: mid,
        media_mid: None,
        name: item["name"]
            .as_str()
            .or(item["title"].as_str())
            .unwrap_or("")
            .to_string(),
        artist: singer.to_string(),
        artists: if singer.is_empty() {
            vec![]
        } else {
            vec![QQArtist {
                id: None,
                mid: None,
                name: singer.to_string(),
            }]
        },
        artist_id: None,
        artist_mid: None,
        album: String::new(),
        album_mid: None,
        cover: String::new(),
        duration: 0,
        fee: 0,
        playable: false,
    }
}

fn map_qq_track(track: &Value, fallback: &Value) -> QQSong {
    let album = &track["album"];
    let artists = map_qq_artists(&track["singer"]);
    let mid = track["mid"]
        .as_str()
        .or(fallback["mid"].as_str())
        .unwrap_or("")
        .to_string();
    let album_mid = album["mid"]
        .as_str()
        .or(track["albummid"].as_str())
        .unwrap_or("")
        .to_string();
    QQSong {
        provider: "qq".into(),
        source: "qq".into(),
        song_type: "qq".into(),
        id: mid.clone(),
        qq_id: track["id"]
            .as_u64()
            .or(fallback["qqId"].as_u64())
            .map(|id| id.to_string()),
        mid: mid.clone(),
        songmid: mid,
        media_mid: track["file"]["media_mid"]
            .as_str()
            .or(track["strMediaMid"].as_str())
            .map(|s| s.to_string()),
        name: track["name"]
            .as_str()
            .or(track["title"].as_str())
            .or(fallback["name"].as_str())
            .unwrap_or("")
            .to_string(),
        artist: artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(" / "),
        artists: if artists.is_empty() {
            fallback["artists"]
                .as_array()
                .map(|a| map_qq_artists(&Value::Array(a.clone())))
                .unwrap_or_default()
        } else {
            artists
        },
        artist_id: track["singer"]
            .as_array()
            .and_then(|a| a.first())
            .and_then(|a| a["id"].as_u64().map(|id| id.to_string())),
        artist_mid: track["singer"]
            .as_array()
            .and_then(|a| a.first())
            .and_then(|a| a["mid"].as_str().map(|s| s.to_string())),
        album: album["name"]
            .as_str()
            .or(album["title"].as_str())
            .or(track["albumname"].as_str())
            .unwrap_or("")
            .to_string(),
        album_mid: Some(album_mid.clone()),
        cover: qq_album_cover(&album_mid, 300),
        duration: (track["interval"].as_u64().unwrap_or(0)) * 1000,
        fee: if track["pay"]["pay_play"].as_u64().unwrap_or(0) > 0 {
            1
        } else {
            0
        },
        playable: false,
    }
}

async fn qq_music_request(payload: &Value, cookie: &str) -> Result<Value, String> {
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let mut req = client()
        .post(MUSICU_URL)
        .header("Referer", QQ_HEADERS_REFERER)
        .header("Content-Type", "application/json;charset=UTF-8")
        .body(body);
    if !cookie.is_empty() {
        req = req.header("Cookie", cookie);
    }
    let resp = req.send().await.map_err(|e| format!("QQ 音乐请求失败: {}", e))?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    // Handle JSONP callback wrapping
    let json_str = text
        .trim()
        .strip_prefix("callback(")
        .and_then(|s| s.strip_suffix(");"))
        .unwrap_or(text.trim());
    serde_json::from_str(json_str).map_err(|e| format!("解析 QQ 响应失败: {}", e))
}

async fn qq_get_json(
    url: &str,
    params: &[(&str, &str)],
    cookie: &str,
) -> Result<Value, String> {
    let mut req = client()
        .get(url)
        .header("Referer", QQ_HEADERS_REFERER)
        .query(params);
    if !cookie.is_empty() {
        req = req.header("Cookie", cookie);
    }
    let resp = req.send().await.map_err(|e| format!("QQ 请求失败: {}", e))?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let json_str = text
        .trim()
        .strip_prefix("callback(")
        .and_then(|s| s.strip_suffix(");"))
        .unwrap_or(text.trim());
    serde_json::from_str(json_str).map_err(|e| format!("解析 QQ 响应失败: {}", e))
}

// ---------- Public API ----------

/// Search songs via QQ Music smartbox.
pub async fn search(keywords: &str, limit: u32, _cookie: &str) -> Result<Vec<QQSong>, String> {
    let limit = limit.max(4).min(12);
    let items = qq_get_json(
        SMARTBOX_URL,
        &[
            ("format", "json"),
            ("key", keywords),
            ("g_tk", "5381"),
            ("loginUin", "0"),
            ("hostUin", "0"),
            ("inCharset", "utf8"),
            ("outCharset", "utf-8"),
            ("notice", "0"),
            ("platform", "yqq.json"),
            ("needNewCode", "0"),
        ],
        "",
    )
    .await?;

    let itemlist = items
        .pointer("/data/song/itemlist")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let base_songs: Vec<QQSong> = itemlist
        .iter()
        .take(limit as usize)
        .map(map_qq_smart_song)
        .collect();

    // Enrich with song detail
    let mut detailed = Vec::new();
    for song in base_songs {
        match qq_song_detail(&song.mid, &song).await {
            Ok(d) => detailed.push(d),
            Err(_) => detailed.push(song),
        }
    }

    // Deduplicate
    let mut seen = std::collections::HashSet::new();
    let result: Vec<QQSong> = detailed
        .into_iter()
        .filter(|s| {
            let key = if !s.mid.is_empty() {
                s.mid.clone()
            } else {
                format!("{}|{}", s.name, s.artist)
            };
            !key.is_empty() && !s.name.is_empty() && seen.insert(key)
        })
        .collect();

    Ok(result)
}

async fn qq_song_detail(mid: &str, fallback: &QQSong) -> Result<QQSong, String> {
    if mid.is_empty() {
        return Ok(fallback.clone());
    }
    let resp = qq_music_request(
        &json!({
            "comm": {"ct": 24, "cv": 0},
            "songinfo": {
                "module": "music.pf_song_detail_svr",
                "method": "get_song_detail_yqq",
                "param": {"song_mid": mid},
            },
        }),
        "",
    )
    .await?;

    let track_info = resp.pointer("/songinfo/data/track_info").unwrap_or(&Value::Null);
    if track_info.is_null() {
        return Ok(fallback.clone());
    }
    Ok(map_qq_track(track_info, &serde_json::to_value(fallback).unwrap_or(Value::Null)))
}

/// Get QQ song playback URL.
pub async fn song_url(
    mid: &str,
    media_mid: &str,
    quality_pref: &str,
    cookie: &str,
) -> Result<QQSongUrlInfo, String> {
    if mid.is_empty() {
        return Ok(QQSongUrlInfo {
            provider: "qq".into(),
            url: String::new(),
            trial: false,
            playable: false,
            level: String::new(),
            quality: String::new(),
            filename: String::new(),
            requested_quality: quality_pref.to_string(),
            restriction: None,
            error: Some("MISSING_MID".into()),
            message: Some("Missing QQ song mid".into()),
            logged_in: Some(!cookie.is_empty()),
        });
    }

    let guid = format!("{}", 10000000 + rand::random::<u32>() % 90000000);
    let start = qq_quality_from(quality_pref);
    let file_media_mid = if !media_mid.is_empty() {
        media_mid
    } else {
        mid
    };

    let mut filenames = Vec::new();
    let mut file_meta = Vec::new();
    for q in QQ_QUALITIES.iter().skip(start) {
        let fname = format!("{}{}{}", q.prefix, file_media_mid, q.ext);
        filenames.push(fname.clone());
        file_meta.push((q.level, q.label, fname));
    }
    // Also try with mid as media
    if file_media_mid != mid {
        for q in QQ_QUALITIES.iter().skip(start) {
            let fname = format!("{}{}{}", q.prefix, mid, q.ext);
            filenames.push(fname.clone());
            file_meta.push((q.level, q.label, fname));
        }
    }

    let songmid_arr: Vec<&str> = filenames.iter().map(|_| mid).collect();
    let songtype_arr: Vec<i64> = filenames.iter().map(|_| 0).collect();

    let resp = qq_music_request(
        &json!({
            "comm": {"uin": "0", "format": "json", "ct": 24, "cv": 0},
            "req_0": {
                "module": "vkey.GetVkeyServer",
                "method": "CgiGetVkey",
                "param": {
                    "guid": guid,
                    "songmid": songmid_arr,
                    "songtype": songtype_arr,
                    "uin": "0",
                    "loginflag": 1,
                    "platform": "20",
                    "filename": filenames,
                },
            },
        }),
        cookie,
    )
    .await?;

    let data = resp.pointer("/req_0/data").unwrap_or(&Value::Null);
    let midurlinfo = data["midurlinfo"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let info = midurlinfo
        .iter()
        .find(|item| item["purl"].as_str().map(|s| !s.is_empty()).unwrap_or(false))
        .or(midurlinfo.first());

    if let Some(info) = info {
        if let Some(purl) = info["purl"].as_str() {
            if !purl.is_empty() {
                let sip = data["sip"]
                    .as_array()
                    .and_then(|a| a.first())
                    .and_then(|v| v.as_str())
                    .unwrap_or("https://ws.stream.qqmusic.qq.com/");
                let filename = info["filename"].as_str().unwrap_or("");
                let meta = file_meta.iter().find(|(_, _, f)| f == filename);
                let (level, quality) = meta
                    .map(|(l, q, _)| (l.to_string(), q.to_string()))
                    .unwrap_or_default();

                return Ok(QQSongUrlInfo {
                    provider: "qq".into(),
                    url: format!("{}{}", sip, purl),
                    trial: false,
                    playable: true,
                    level,
                    quality,
                    filename: filename.to_string(),
                    requested_quality: quality_pref.to_string(),
                    restriction: None,
                    error: None,
                    message: None,
                    logged_in: Some(!cookie.is_empty()),
                });
            }
        }
    }

    Ok(QQSongUrlInfo {
        provider: "qq".into(),
        url: String::new(),
        trial: false,
        playable: false,
        level: String::new(),
        quality: String::new(),
        filename: String::new(),
        requested_quality: quality_pref.to_string(),
        restriction: Some(json!({
            "provider": "qq",
            "category": "url_unavailable",
            "message": "QQ 音乐没有返回播放地址，可能受版权、会员或官方客户端限制",
        })),
        error: Some("QQ_URL_UNAVAILABLE".into()),
        message: Some("QQ 音乐没有返回播放地址".into()),
        logged_in: Some(!cookie.is_empty()),
    })
}

/// Fetch lyrics for a QQ song.
pub async fn lyric(mid: &str, id: &str, cookie: &str) -> Result<QQLyricResult, String> {
    if mid.is_empty() && id.is_empty() {
        return Err("Missing QQ song mid or id".into());
    }

    let mut lyric_text = String::new();
    let mut trans_text = String::new();
    let mut qrc_text = String::new();
    let mut roma_text = String::new();
    let mut source = "qq-musicu";

    // Try musicu lyric
    let mut param = serde_json::Map::new();
    if !mid.is_empty() {
        param.insert("songMID".into(), json!(mid));
    }
    if !id.is_empty() {
        if let Ok(n) = id.parse::<u64>() {
            param.insert("songID".into(), json!(n));
        }
    }

    match qq_music_request(
        &json!({
            "comm": {"ct": 24, "cv": 0},
            "lyric": {
                "module": "music.musichallSong.PlayLyricInfo",
                "method": "GetPlayLyricInfo",
                "param": param,
            },
        }),
        cookie,
    )
    .await
    {
        Ok(resp) => {
            let data = resp.pointer("/lyric/data").unwrap_or(&Value::Null);
            lyric_text = decode_qq_lyric(data["lyric"].as_str().unwrap_or(""));
            trans_text = decode_qq_lyric(data["trans"].as_str().unwrap_or(""));
            qrc_text = decode_qq_lyric(data["qrc"].as_str().unwrap_or(""));
            roma_text = decode_qq_lyric(data["roma"].as_str().unwrap_or(""));
        }
        Err(e) => {
            eprintln!("[QQLyric] musicu failed: {}", e);
        }
    }

    // Fallback to legacy lyric endpoint
    if lyric_text.is_empty() && !mid.is_empty() {
        match qq_get_json(
            "https://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg",
            &[
                ("songmid", mid),
                ("songtype", "0"),
                ("format", "json"),
                ("nobase64", "1"),
                ("g_tk", "5381"),
                ("loginUin", "0"),
                ("hostUin", "0"),
                ("inCharset", "utf8"),
                ("outCharset", "utf-8"),
                ("notice", "0"),
                ("platform", "yqq.json"),
                ("needNewCode", "0"),
            ],
            cookie,
        )
        .await
        {
            Ok(body) => {
                lyric_text = decode_qq_lyric(body["lyric"].as_str().unwrap_or(""));
                let trans_legacy = decode_qq_lyric(
                    body["trans"]
                        .as_str()
                        .or(body["tlyric"].as_str())
                        .unwrap_or(""),
                );
                if !trans_legacy.is_empty() {
                    trans_text = trans_legacy;
                }
                source = "qq-legacy";
            }
            Err(e) => {
                eprintln!("[QQLyric] legacy failed: {}", e);
            }
        }
    }

    let is_empty = lyric_text.is_empty();
    Ok(QQLyricResult {
        provider: "qq".into(),
        mid: mid.to_string(),
        lyric: lyric_text,
        tlyric: trans_text,
        yrc: String::new(),
        qrc: qrc_text,
        roma: roma_text,
        source: if is_empty { "qq-empty".into() } else { source.into() },
    })
}

fn decode_qq_lyric(text: &str) -> String {
    let raw = decode_html_entities(text.trim());
    if raw.is_empty() {
        return String::new();
    }
    let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    let looks_base64 = compact.len() >= 8
        && compact.len() % 4 == 0
        && compact
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');
    if looks_base64 && !raw.starts_with('[') {
        if let Ok(decoded) = BASE64.decode(&compact) {
            if let Ok(text) = String::from_utf8(decoded) {
                if text.contains('[') || text.chars().any(|c| ('\u{4e00}'..='\u{9fa5}').contains(&c)) {
                    return decode_html_entities(&text.replace('\u{feff}', "").replace("\r\n", "\n"));
                }
            }
        }
    }
    decode_html_entities(&raw.replace("\r\n", "\n"))
}

fn decode_html_entities(text: &str) -> String {
    text.replace("&#x27;", "'")
        .replace("&#x2F;", "/")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&nbsp;", " ")
}
