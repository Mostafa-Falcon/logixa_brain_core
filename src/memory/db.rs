use crate::{config::BrainConfig, error::AppResult};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecord {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub mode: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub scope: String,
    pub project_id: Option<String>,
    pub kind: String,
    pub content: String,
    pub tags: Option<String>,
    pub importance: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecord {
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub kind: Option<String>,
    pub rules: Option<String>,
    pub status: String,
    pub active_model_profile: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct BrainDb {
    conn: Mutex<Connection>,
}

impl BrainDb {
    pub fn open(path: &str) -> AppResult<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn with_conn<T>(&self, f: impl FnOnce(&Connection) -> AppResult<T>) -> AppResult<T> {
        let conn = self.conn.lock().expect("database lock poisoned");
        f(&conn)
    }

    pub fn ping(&self) -> AppResult<bool> {
        self.with_conn(|conn| {
            let _: i64 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
            Ok(true)
        })
    }

    pub fn migrate(&self) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS conversations (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    project_id TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS messages (
                    id TEXT PRIMARY KEY,
                    conversation_id TEXT NOT NULL,
                    role TEXT NOT NULL,
                    content TEXT NOT NULL,
                    mode TEXT,
                    created_at TEXT NOT NULL,
                    FOREIGN KEY(conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS memories (
                    id TEXT PRIMARY KEY,
                    scope TEXT NOT NULL,
                    project_id TEXT,
                    kind TEXT NOT NULL,
                    content TEXT NOT NULL,
                    tags TEXT,
                    importance INTEGER NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
                    id UNINDEXED,
                    content,
                    scope,
                    project_id,
                    kind,
                    tags
                );

                CREATE TABLE IF NOT EXISTS projects (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    path TEXT,
                    kind TEXT,
                    rules TEXT,
                    status TEXT NOT NULL DEFAULT 'active',
                    active_model_profile TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS active_project (
                    singleton_id INTEGER PRIMARY KEY CHECK(singleton_id = 1),
                    project_id TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS model_profiles (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    model_path TEXT NOT NULL,
                    config_json TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS tool_runs (
                    id TEXT PRIMARY KEY,
                    tool_name TEXT NOT NULL,
                    input_json TEXT NOT NULL,
                    output_json TEXT,
                    success INTEGER NOT NULL,
                    created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS system_events (
                    id TEXT PRIMARY KEY,
                    level TEXT NOT NULL,
                    event TEXT NOT NULL,
                    details_json TEXT,
                    created_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_messages_conversation_created ON messages(conversation_id, created_at);
                CREATE INDEX IF NOT EXISTS idx_memories_project ON memories(project_id);
                CREATE INDEX IF NOT EXISTS idx_events_created ON system_events(created_at);
            "#)?;
            Ok(())
        })
    }

    pub fn seed_defaults(&self, config: &BrainConfig) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO projects (id, name, path, kind, rules, status, created_at, updated_at) VALUES (?1, ?2, NULL, ?3, ?4, 'active', ?5, ?5)",
                params![config.brain.default_project_id, "Logixa Brain", "brain-core", "Headless local AI brain core. UI is always an external client." , now],
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO active_project (singleton_id, project_id, updated_at) VALUES (1, ?1, ?2)",
                params![config.brain.default_project_id, now],
            )?;
            conn.execute(
                "INSERT OR REPLACE INTO model_profiles (id, name, model_path, config_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                params![config.model.model_id, config.model.name, config.model.model_path, serde_json::to_string(&config.model).unwrap_or_default(), now],
            )?;
            Ok(())
        })
    }

    pub fn create_conversation(&self, title: &str, project_id: Option<&str>) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO conversations (id, title, project_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
                params![id, title, project_id, now],
            )?;
            Ok(id)
        })
    }

    pub fn save_message(
        &self,
        conversation_id: &str,
        role: &str,
        content: &str,
        mode: Option<&str>,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO messages (id, conversation_id, role, content, mode, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, conversation_id, role, content, mode, now],
            )?;
            conn.execute("UPDATE conversations SET updated_at = ?1 WHERE id = ?2", params![now, conversation_id])?;
            Ok(id)
        })
    }

    pub fn recent_messages(
        &self,
        conversation_id: &str,
        limit: usize,
    ) -> AppResult<Vec<MessageRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, conversation_id, role, content, mode, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at DESC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![conversation_id, limit as i64], |row| {
                Ok(MessageRecord {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    mode: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?;
            let mut items: Vec<MessageRecord> = rows.collect::<Result<Vec<_>, _>>()?;
            items.reverse();
            Ok(items)
        })
    }

    pub fn insert_memory(&self, memory: &MemoryRecord) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO memories (id, scope, project_id, kind, content, tags, importance, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![memory.id, memory.scope, memory.project_id, memory.kind, memory.content, memory.tags, memory.importance, memory.created_at, memory.updated_at],
            )?;
            conn.execute(
                "INSERT INTO memories_fts (id, content, scope, project_id, kind, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![memory.id, memory.content, memory.scope, memory.project_id, memory.kind, memory.tags],
            )?;
            Ok(())
        })
    }

    pub fn search_memories(
        &self,
        query: &str,
        project_id: Option<&str>,
        limit: usize,
    ) -> AppResult<Vec<MemoryRecord>> {
        self.with_conn(|conn| {
            let safe_query = query.replace('"', " ");
            let sql = if project_id.is_some() {
                "SELECT m.id, m.scope, m.project_id, m.kind, m.content, m.tags, m.importance, m.created_at, m.updated_at
                 FROM memories_fts f JOIN memories m ON m.id = f.id
                 WHERE memories_fts MATCH ?1 AND (m.project_id = ?2 OR m.project_id IS NULL)
                 ORDER BY m.importance DESC, m.updated_at DESC LIMIT ?3"
            } else {
                "SELECT m.id, m.scope, m.project_id, m.kind, m.content, m.tags, m.importance, m.created_at, m.updated_at
                 FROM memories_fts f JOIN memories m ON m.id = f.id
                 WHERE memories_fts MATCH ?1
                 ORDER BY m.importance DESC, m.updated_at DESC LIMIT ?3"
            };
            let mut stmt = conn.prepare(sql)?;
            let map_row = |row: &rusqlite::Row| -> rusqlite::Result<MemoryRecord> {
                Ok(MemoryRecord {
                    id: row.get(0)?,
                    scope: row.get(1)?,
                    project_id: row.get(2)?,
                    kind: row.get(3)?,
                    content: row.get(4)?,
                    tags: row.get(5)?,
                    importance: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            };
            if let Some(pid) = project_id {
                let rows = stmt.query_map(params![safe_query, pid, limit as i64], map_row)?;
                Ok(rows.collect::<Result<Vec<_>, _>>()?)
            } else {
                let rows = stmt.query_map(params![safe_query, "", limit as i64], map_row)?;
                Ok(rows.collect::<Result<Vec<_>, _>>()?)
            }
        })
    }

    pub fn list_projects(&self) -> AppResult<Vec<ProjectRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, path, kind, rules, status, active_model_profile, created_at, updated_at FROM projects ORDER BY updated_at DESC")?;
            let rows = stmt.query_map([], |row| {
                Ok(ProjectRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    kind: row.get(3)?,
                    rules: row.get(4)?,
                    status: row.get(5)?,
                    active_model_profile: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        })
    }

    pub fn upsert_project(&self, project: &ProjectRecord) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO projects (id, name, path, kind, rules, status, active_model_profile, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                 ON CONFLICT(id) DO UPDATE SET name=excluded.name, path=excluded.path, kind=excluded.kind, rules=excluded.rules, status=excluded.status, updated_at=excluded.updated_at",
                params![project.id, project.name, project.path, project.kind, project.rules, project.status, project.active_model_profile, project.created_at, project.updated_at],
            )?;
            Ok(())
        })
    }

    pub fn activate_project(&self, id: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO active_project (singleton_id, project_id, updated_at) VALUES (1, ?1, ?2)
                 ON CONFLICT(singleton_id) DO UPDATE SET project_id=excluded.project_id, updated_at=excluded.updated_at",
                params![id, now],
            )?;
            Ok(())
        })
    }

    pub fn active_project_id(&self) -> AppResult<String> {
        self.with_conn(|conn| {
            let id: String = conn.query_row(
                "SELECT project_id FROM active_project WHERE singleton_id = 1",
                [],
                |row| row.get(0),
            )?;
            Ok(id)
        })
    }

    pub fn insert_event(
        &self,
        level: &str,
        event: &str,
        details_json: Option<&str>,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO system_events (id, level, event, details_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, level, event, details_json, now],
            )?;
            Ok(id)
        })
    }

    pub fn recent_events(&self, limit: usize) -> AppResult<Vec<serde_json::Value>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, level, event, details_json, created_at FROM system_events ORDER BY created_at DESC LIMIT ?1")?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, String>(0)?,
                    "level": row.get::<_, String>(1)?,
                    "event": row.get::<_, String>(2)?,
                    "details_json": row.get::<_, Option<String>>(3)?,
                    "created_at": row.get::<_, String>(4)?,
                }))
            })?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        })
    }

    pub fn insert_tool_run(
        &self,
        tool_name: &str,
        input_json: &str,
        output_json: Option<&str>,
        success: bool,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO tool_runs (id, tool_name, input_json, output_json, success, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, tool_name, input_json, output_json, if success { 1 } else { 0 }, now],
            )?;
            Ok(id)
        })
    }
}
