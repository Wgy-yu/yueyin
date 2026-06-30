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
}
