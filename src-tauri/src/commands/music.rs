use serde_json::Value;
use tauri::State;

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
            let info =
                netease::song_url(song_id, q, &cookies.netease_cookie(), false).await?;
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
pub async fn music_qr_check(
    key: String,
    cookies: State<'_, CookieStore>,
) -> Result<Value, String> {
    let resp = netease::qr_check(&key, &cookies.netease_cookie()).await?;
    let code = resp["code"].as_i64().unwrap_or(0);
    if code == 803 {
        // Success: try to extract cookie from response
        if let Some(cookie_val) = extract_cookie_from_response(&resp) {
            cookies.set_netease_cookie(&cookie_val).ok();
        }
    }
    Ok(resp)
}

fn extract_cookie_from_response(resp: &Value) -> Option<String> {
    // Try various locations where cookie might appear
    for path in &["/cookie", "/body/cookie", "/body/data/cookie", "/body/data/cookies"] {
        if let Some(val) = resp.pointer(path) {
            if let Some(s) = val.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            }
            // If it's an array of strings
            if let Some(arr) = val.as_array() {
                let joined: Vec<&str> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect();
                if !joined.is_empty() {
                    return Some(joined.join("; "));
                }
            }
        }
    }
    None
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
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
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
