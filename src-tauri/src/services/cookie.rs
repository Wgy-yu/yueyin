use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct CookieStore {
    data_dir: PathBuf,
    netease: Mutex<String>,
    qq: Mutex<String>,
}

impl CookieStore {
    pub fn open(data_dir: &PathBuf) -> Self {
        fs::create_dir_all(data_dir).ok();
        let netease_path = data_dir.join(".netease-cookie");
        let qq_path = data_dir.join(".qq-cookie");
        let netease = fs::read_to_string(&netease_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        let qq = fs::read_to_string(&qq_path)
            .unwrap_or_default()
            .trim()
            .to_string();
        CookieStore {
            data_dir: data_dir.clone(),
            netease: Mutex::new(netease),
            qq: Mutex::new(qq),
        }
    }

    pub fn netease_cookie(&self) -> String {
        self.netease.lock().unwrap().clone()
    }

    pub fn set_netease_cookie(&self, cookie: &str) -> Result<(), String> {
        let trimmed = cookie.trim().to_string();
        let path = self.data_dir.join(".netease-cookie");
        fs::write(&path, &trimmed)
            .map_err(|e| format!("写入网易云 cookie 失败: {}", e))?;
        *self.netease.lock().unwrap() = trimmed;
        Ok(())
    }

    pub fn qq_cookie(&self) -> String {
        self.qq.lock().unwrap().clone()
    }

    pub fn set_qq_cookie(&self, cookie: &str) -> Result<(), String> {
        let trimmed = cookie.trim().to_string();
        let path = self.data_dir.join(".qq-cookie");
        fs::write(&path, &trimmed)
            .map_err(|e| format!("写入 QQ cookie 失败: {}", e))?;
        *self.qq.lock().unwrap() = trimmed;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn csrf_token(&self) -> String {
        parse_cookie_field(&self.netease_cookie(), "__csrf")
    }
}

#[allow(dead_code)]
fn parse_cookie_field(cookie: &str, field: &str) -> String {
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix(field).and_then(|s| s.strip_prefix('=')) {
            return val.trim().to_string();
        }
    }
    String::new()
}
