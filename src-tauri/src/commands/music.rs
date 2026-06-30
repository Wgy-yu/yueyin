use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::services::cookie::CookieStore;
use crate::services::{netease, qq};

// ---------- Netease commands ----------

#[tauri::command]
pub async fn music_search(
    keywords: String,
    limit: Option<u32>,
    source: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let src = source.as_deref().unwrap_or("netease");
    let lim = limit.unwrap_or(20);
    match src {
        "qq" => {
            let songs = qq::search(&keywords, lim, &cookies.qq_cookie()).await?;
            Ok(serde_json::json!({ "provider": "qq", "songs": songs }))
        }
        _ => {
            let songs = netease::search(&keywords, lim, &cookies.netease_cookie()).await?;
            Ok(serde_json::json!({ "songs": songs }))
        }
    }
}

#[tauri::command]
pub async fn music_song_url(
    id: String,
    source: Option<String>,
    quality: Option<String>,
    media_mid: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let src = source.as_deref().unwrap_or("netease");
    let q = quality.as_deref().unwrap_or("hires");
    match src {
        "qq" => {
            let mid = &id;
            let mmid = media_mid.as_deref().unwrap_or("");
            let info = qq::song_url(mid, mmid, q, &cookies.qq_cookie()).await?;
            Ok(serde_json::to_value(info).unwrap())
        }
        _ => {
            let song_id: u64 = id.parse().map_err(|_| "Invalid song id")?;
            let info = netease::song_url(song_id, q, &cookies.netease_cookie(), false).await?;
            Ok(serde_json::to_value(info).unwrap())
        }
    }
}

#[tauri::command]
pub async fn music_lyric(
    id: String,
    source: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let src = source.as_deref().unwrap_or("netease");
    match src {
        "qq" => {
            let result = qq::lyric(&id, "", &cookies.qq_cookie()).await?;
            Ok(serde_json::to_value(result).unwrap())
        }
        _ => {
            let song_id: u64 = id.parse().map_err(|_| "Invalid song id")?;
            let result = netease::lyric(song_id, &cookies.netease_cookie()).await?;
            Ok(serde_json::to_value(result).unwrap())
        }
    }
}

// ---------- Cookie management ----------

#[tauri::command]
pub async fn music_open_web_login(
    provider: String,
    app: tauri::AppHandle,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let is_qq = provider == "qq";
    let label = if is_qq {
        "music-login-qq"
    } else {
        "music-login-netease"
    };
    if let Some(existing) = app.get_webview_window(label) {
        existing.set_focus().map_err(|e| e.to_string())?;
        return Err("登录窗口已打开，请在官方页面完成登录".into());
    }

    let (url, title, width, height) = if is_qq {
        (
            "https://y.qq.com/n/ryqq/profile",
            "QQ 音乐登录",
            900.0,
            720.0,
        )
    } else {
        (
            "https://music.163.com/#/login",
            "网易云音乐登录",
            940.0,
            760.0,
        )
    };
    let window = WebviewWindowBuilder::new(
        &app,
        label,
        WebviewUrl::External(url.parse().map_err(|e| format!("登录地址无效: {e}"))?),
    )
    .title(title)
    .inner_size(width, height)
    .min_inner_size(760.0, 560.0)
    .center()
    .build()
    .map_err(|e| format!("打开官方登录窗口失败: {e}"))?;

    for _ in 0..250 {
        tokio::time::sleep(Duration::from_millis(1200)).await;
        if app.get_webview_window(label).is_none() {
            return Err("LOGIN_CANCELLED: 登录窗口已关闭".into());
        }
        let header = build_webview_cookie_header(&window, is_qq)?;
        if is_qq {
            if qq_cookie_has_login(&header) {
                cookies.set_qq_cookie(&header)?;
                let info = qq_login_info(&header);
                let _ = window.close();
                return Ok(info);
            }
        } else if cookie_value(&header, "MUSIC_U").is_some() {
            cookies.set_netease_cookie(&header)?;
            let info = netease::login_status(&header).await;
            if info.logged_in {
                let _ = window.close();
                return serde_json::to_value(info).map_err(|e| e.to_string());
            }
        }
    }

    let _ = window.close();
    Err("LOGIN_TIMEOUT: 登录超时，请重试".into())
}

#[tauri::command]
pub fn music_cancel_web_login(app: tauri::AppHandle) {
    for label in ["music-login-netease", "music-login-qq"] {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.close();
        }
    }
}

fn build_webview_cookie_header(
    window: &tauri::WebviewWindow,
    is_qq: bool,
) -> Result<String, String> {
    let allowed = |domain: &str| {
        let domain = domain.trim_start_matches('.').to_ascii_lowercase();
        if is_qq {
            domain == "qq.com" || domain.ends_with(".qq.com")
        } else {
            domain == "163.com"
                || domain.ends_with(".163.com")
                || domain == "netease.com"
                || domain.ends_with(".netease.com")
        }
    };
    let mut values = HashMap::new();
    for cookie in window
        .cookies()
        .map_err(|e| format!("读取登录会话失败: {e}"))?
    {
        if cookie.domain().is_none_or(allowed) && !cookie.value().is_empty() {
            values.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }
    let priority: &[&str] = if is_qq {
        &[
            "uin",
            "qqmusic_uin",
            "wxuin",
            "p_uin",
            "qm_keyst",
            "qqmusic_key",
            "music_key",
            "wxskey",
            "p_skey",
            "skey",
        ]
    } else {
        &["MUSIC_U", "__csrf", "NMTID", "MUSIC_A"]
    };
    let mut pairs = Vec::new();
    for name in priority {
        if let Some(value) = values.remove(*name) {
            pairs.push(format!("{name}={value}"));
        }
    }
    let mut rest: Vec<_> = values.into_iter().collect();
    rest.sort_by(|a, b| a.0.cmp(&b.0));
    pairs.extend(
        rest.into_iter()
            .map(|(name, value)| format!("{name}={value}")),
    );
    Ok(pairs.join("; "))
}

fn cookie_value(cookie: &str, name: &str) -> Option<String> {
    cookie.split(';').find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key == name && !value.is_empty()).then(|| value.to_string())
    })
}

fn qq_uin(cookie: &str) -> Option<String> {
    ["uin", "qqmusic_uin", "wxuin", "p_uin"]
        .iter()
        .find_map(|name| cookie_value(cookie, name))
        .map(|value| {
            value
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
        })
        .filter(|value| !value.is_empty())
}

fn qq_cookie_has_login(cookie: &str) -> bool {
    qq_uin(cookie).is_some()
        && [
            "qm_keyst",
            "qqmusic_key",
            "music_key",
            "p_skey",
            "skey",
            "wxskey",
        ]
        .iter()
        .any(|name| cookie_value(cookie, name).is_some())
}

fn qq_login_info(cookie: &str) -> Value {
    let user_id = qq_uin(cookie).unwrap_or_default();
    let nickname = ["nickname", "nick", "qq_nickname", "ptnick"]
        .iter()
        .find_map(|name| cookie_value(cookie, name))
        .unwrap_or_else(|| "QQ 音乐用户".into());
    serde_json::json!({
        "provider": "qq",
        "loggedIn": true,
        "hasCookie": true,
        "userId": user_id,
        "nickname": nickname,
        "avatar": format!("https://q1.qlogo.cn/g?b=qq&nk={user_id}&s=100"),
    })
}

#[cfg(test)]
mod login_tests {
    use super::*;

    #[test]
    fn recognizes_provider_login_cookies() {
        assert_eq!(
            cookie_value("foo=1; MUSIC_U=token", "MUSIC_U").as_deref(),
            Some("token")
        );
        assert!(qq_cookie_has_login("uin=o12345; p_skey=key"));
        assert!(qq_cookie_has_login("uin=o12345; qm_keyst=key"));
    }
}

#[tauri::command]
pub async fn music_login_status(cookies: State<'_, CookieStore>) -> Result<Value, String> {
    let info = netease::login_status(&cookies.netease_cookie()).await;
    Ok(serde_json::to_value(info).unwrap())
}

#[tauri::command]
pub async fn music_login_cookie(
    cookie: String,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    // Validate: must contain MUSIC_U
    if !cookie.contains("MUSIC_U=") {
        return Ok(serde_json::json!({
            "loggedIn": false,
            "error": "INVALID_NETEASE_COOKIE",
            "message": "网易云 cookie 缺少 MUSIC_U",
        }));
    }
    cookies.set_netease_cookie(&cookie)?;
    let info = netease::login_status(&cookies.netease_cookie()).await;
    let mut result = serde_json::to_value(info).unwrap();
    result["saved"] = serde_json::json!(true);
    result["hasCookie"] = serde_json::json!(true);
    Ok(result)
}

#[tauri::command]
pub async fn music_logout(cookies: State<'_, CookieStore>) -> Result<Value, String> {
    cookies.set_netease_cookie("")?;
    Ok(serde_json::json!({ "ok": true }))
}

#[tauri::command]
pub async fn music_qr_key() -> Result<Value, String> {
    let key = netease::qr_key().await?;
    Ok(serde_json::json!({ "key": key }))
}

#[tauri::command]
pub async fn music_qr_create(key: String) -> Result<Value, String> {
    let (img, url) = netease::qr_create(&key).await?;
    Ok(serde_json::json!({ "img": img, "url": url }))
}

#[tauri::command]
pub async fn music_qr_check(key: String, cookies: State<'_, CookieStore>) -> Result<Value, String> {
    let result = netease::qr_check_with_cookies(&key, &cookies.netease_cookie()).await?;
    let code = result.body["code"].as_i64().unwrap_or(0);

    if code == 803 {
        // Login success: validate cookie contains MUSIC_U
        if !result.cookie.contains("MUSIC_U=") {
            return Ok(serde_json::json!({
                "code": code,
                "success": false,
                "error": "MISSING_MUSIC_U",
                "message": "登录成功但未获取到 MUSIC_U",
            }));
        }
        // Save cookie
        cookies.set_netease_cookie(&result.cookie)?;
        // Verify by fetching profile
        let info = netease::login_status(&result.cookie).await;
        return Ok(serde_json::json!({
            "code": code,
            "success": info.logged_in,
            "loggedIn": info.logged_in,
            "profile": {
                "userId": info.user_id,
                "nickname": info.nickname,
                "avatar": info.avatar,
            }
        }));
    }

    // Not success yet (800=expired, 801=waiting, 802=scanned)
    Ok(serde_json::json!({
        "code": code,
        "success": false,
    }))
}

// ---------- QQ Music auth ----------

#[tauri::command]
pub async fn music_qq_login_status(cookies: State<'_, CookieStore>) -> Result<Value, String> {
    let qq_cookie = cookies.qq_cookie();
    if qq_cookie.is_empty() {
        return Ok(serde_json::json!({ "provider": "qq", "loggedIn": false, "hasCookie": false }));
    }
    // Simple validation: check for uin
    let has_uin = qq_cookie.contains("uin=");
    Ok(serde_json::json!({
        "provider": "qq",
        "loggedIn": has_uin,
        "hasCookie": true,
    }))
}

#[tauri::command]
pub async fn music_qq_login_cookie(
    cookie: String,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    cookies.set_qq_cookie(&cookie)?;
    Ok(serde_json::json!({
        "provider": "qq",
        "saved": true,
        "hasCookie": true,
    }))
}

#[tauri::command]
pub async fn music_qq_logout(cookies: State<'_, CookieStore>) -> Result<Value, String> {
    cookies.set_qq_cookie("")?;
    Ok(serde_json::json!({ "provider": "qq", "ok": true, "loggedIn": false }))
}

// ---------- Audio proxy ----------

#[tauri::command]
pub async fn music_audio_proxy(url: String) -> Result<Vec<u8>, String> {
    use super::super::services::http_client::client;

    let resp = client()
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .header("Referer", "https://music.163.com/")
        .send()
        .await
        .map_err(|e| format!("音频代理请求失败: {}", e))?;

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("读取音频数据失败: {}", e))?;

    Ok(bytes.to_vec())
}

// ---------- Playlists ----------

#[tauri::command]
pub async fn music_user_playlists(
    uid: Option<String>,
    source: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let src = source.as_deref().unwrap_or("netease");
    match src {
        "qq" => {
            let qq_cookie = cookies.qq_cookie();
            let uin = uid.as_deref().unwrap_or("");
            qq::user_playlists(uin, &qq_cookie).await
        }
        _ => {
            let netease_cookie = cookies.netease_cookie();
            let uid_str = uid.as_deref().unwrap_or("");
            netease::user_playlists(uid_str, &netease_cookie).await
        }
    }
}

#[tauri::command]
pub async fn music_playlist_tracks(
    id: String,
    source: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let src = source.as_deref().unwrap_or("netease");
    match src {
        "qq" => qq::playlist_tracks(&id, &cookies.qq_cookie()).await,
        _ => netease::playlist_tracks(&id, &cookies.netease_cookie()).await,
    }
}

#[tauri::command]
pub async fn music_like_check(
    ids: Vec<String>,
    uid: Option<String>,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let u64_ids: Vec<u64> = ids.iter().filter_map(|s| s.parse().ok()).collect();
    let uid_str = uid.as_deref().unwrap_or("");
    netease::like_check(&u64_ids, uid_str, &cookies.netease_cookie()).await
}

#[tauri::command]
pub async fn music_like_toggle(
    id: String,
    like: bool,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let song_id: u64 = id.parse().map_err(|_| "Invalid song id")?;
    netease::like_toggle(song_id, like, &cookies.netease_cookie()).await
}
