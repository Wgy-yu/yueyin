use rusqlite::{Connection, OptionalExtension, params};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

const MIGRATIONS: &[&str] = &[
    // v1: app settings
    "CREATE TABLE IF NOT EXISTS app_settings (
        key   TEXT PRIMARY KEY,
        value TEXT NOT NULL,
        updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );",
    // v2: play queue persistence
    "CREATE TABLE IF NOT EXISTS play_queue (
        position    INTEGER PRIMARY KEY,
        track_id    TEXT NOT NULL,
        track_name  TEXT NOT NULL,
        artist      TEXT NOT NULL,
        album       TEXT,
        cover_url   TEXT,
        duration    INTEGER,
        source      TEXT NOT NULL,
        extra       TEXT
    );",
];

impl Database {
    pub fn open(app_data_dir: &PathBuf) -> Result<Self, String> {
        fs::create_dir_all(app_data_dir)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
        let db_path = app_data_dir.join("yueyin.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("打开数据库失败: {}", e))?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| format!("设置 PRAGMA 失败: {}", e))?;

        let db = Database { conn: Mutex::new(conn) };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
        conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS _migrations (
                    id INTEGER PRIMARY KEY,
                    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
                );",
            )
            .map_err(|e| format!("创建迁移表失败: {}", e))?;

        for (i, sql) in MIGRATIONS.iter().enumerate() {
            let id = (i + 1) as i64;
            let already: bool = conn
                .query_row(
                    "SELECT COUNT(*) > 0 FROM _migrations WHERE id = ?1",
                    params![id],
                    |row| row.get(0),
                )
                .map_err(|e| format!("查询迁移状态失败: {}", e))?;

            if !already {
                conn.execute_batch(sql)
                    .map_err(|e| format!("执行迁移 {} 失败: {}", id, e))?;
                conn.execute(
                        "INSERT INTO _migrations (id) VALUES (?1)",
                        params![id],
                    )
                    .map_err(|e| format!("记录迁移 {} 失败: {}", id, e))?;
            }
        }
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, String> {
        let conn = self.conn.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
        conn.query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| format!("读取设置失败: {}", e))
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
        conn.execute(
                "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
                 ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now')",
                params![key, value],
            )
            .map_err(|e| format!("写入设置失败: {}", e))?;
        Ok(())
    }

    // ponytail: queue is small (<500 items), single transaction replace-all is fine
    pub fn save_queue(&self, tracks: &[QueueTrack]) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
        conn.execute_batch("BEGIN; DELETE FROM play_queue;")
            .map_err(|e| format!("清空队列失败: {}", e))?;
        {
            let mut stmt = conn
                .prepare("INSERT INTO play_queue (position,track_id,track_name,artist,album,cover_url,duration,source,extra) VALUES (?,?,?,?,?,?,?,?,?)")
                .map_err(|e| format!("准备队列插入失败: {}", e))?;
            for (i, t) in tracks.iter().enumerate() {
                stmt.execute(params![
                    i as i64, t.track_id, t.track_name, t.artist,
                    t.album, t.cover_url, t.duration, t.source, t.extra
                ])
                .map_err(|e| format!("插入队列项 {} 失败: {}", i, e))?;
            }
        }
        conn.execute_batch("COMMIT;")
            .map_err(|e| format!("提交队列失败: {}", e))?;
        Ok(())
    }

    pub fn load_queue(&self) -> Result<Vec<QueueTrack>, String> {
        let conn = self.conn.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
        let mut stmt = conn
            .prepare("SELECT track_id,track_name,artist,album,cover_url,duration,source,extra FROM play_queue ORDER BY position")
            .map_err(|e| format!("准备队列查询失败: {}", e))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(QueueTrack {
                    track_id: row.get(0)?,
                    track_name: row.get(1)?,
                    artist: row.get(2)?,
                    album: row.get(3)?,
                    cover_url: row.get(4)?,
                    duration: row.get(5)?,
                    source: row.get(6)?,
                    extra: row.get(7)?,
                })
            })
            .map_err(|e| format!("查询队列失败: {}", e))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| format!("读取队列行失败: {}", e))?);
        }
        Ok(result)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct QueueTrack {
    pub track_id: String,
    pub track_name: String,
    pub artist: String,
    pub album: Option<String>,
    pub cover_url: Option<String>,
    pub duration: Option<i64>,
    pub source: String,
    pub extra: Option<String>,
}
