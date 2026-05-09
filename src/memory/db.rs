use crate::{config::BrainConfig, error::AppResult, memory::search::normalize_fts_query};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStateRecord {
    pub project_id: String,
    pub current_step: Option<String>,
    pub last_commit: Option<String>,
    pub last_tag: Option<String>,
    pub summary: Option<String>,
    pub next_step: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingToolCallRecord {
    pub id: String,
    pub tool_name: String,
    pub project_id: Option<String>,
    pub input_json: String,
    pub status: String,
    pub danger_level: String,
    pub created_at: String,
    pub updated_at: String,
    pub decided_at: Option<String>,
    pub decision_note: Option<String>,
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

                CREATE TABLE IF NOT EXISTS project_states (
                    project_id TEXT PRIMARY KEY,
                    current_step TEXT,
                    last_commit TEXT,
                    last_tag TEXT,
                    summary TEXT,
                    next_step TEXT,
                    status TEXT NOT NULL DEFAULT 'active',
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
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

                CREATE TABLE IF NOT EXISTS pending_tool_calls (
                    id TEXT PRIMARY KEY,
                    tool_name TEXT NOT NULL,
                    project_id TEXT,
                    input_json TEXT NOT NULL,
                    status TEXT NOT NULL DEFAULT 'pending',
                    danger_level TEXT NOT NULL DEFAULT 'low',
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    decided_at TEXT,
                    decision_note TEXT
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
                CREATE INDEX IF NOT EXISTS idx_pending_tool_calls_status ON pending_tool_calls(status, created_at);
                CREATE INDEX IF NOT EXISTS idx_tool_runs_created ON tool_runs(created_at);
            "#)?;
            Ok(())
        })
    }

    pub fn seed_defaults(&self, config: &BrainConfig) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO projects (id, name, path, kind, rules, status, created_at, updated_at) VALUES (?1, ?2, NULL, ?3, ?4, 'active', ?5, ?5)",
                params![config.brain.default_project_id, "Logixa Brain", "brain-core", "Headless local AI brain core. UI is always an external client.", now],
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO project_states (project_id, current_step, last_commit, last_tag, summary, next_step, status, created_at, updated_at) VALUES (?1, ?2, NULL, ?3, ?4, ?5, 'active', ?6, ?6)",
                params![
                    config.brain.default_project_id,
                    "Step 5 — Project State Memory",
                    "brain-step4-long-term-memory",
                    "Logixa Brain Core is a headless Rust service with Qwen runtime, Gigi prompt policy, conversation history, and long-term memory.",
                    "Add project state memory so Gigi can know the active project status.",
                    now
                ],
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

    pub fn conversation_project_id(
        &self,
        conversation_id: &str,
    ) -> AppResult<Option<Option<String>>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT project_id FROM conversations WHERE id = ?1")?;
            let mut rows = stmt.query(params![conversation_id])?;
            if let Some(row) = rows.next()? {
                Ok(Some(row.get::<_, Option<String>>(0)?))
            } else {
                Ok(None)
            }
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
        self.search_memories_filtered(query, project_id, None, None, limit)
    }

    pub fn search_memories_filtered(
        &self,
        query: &str,
        project_id: Option<&str>,
        scope: Option<&str>,
        kind: Option<&str>,
        limit: usize,
    ) -> AppResult<Vec<MemoryRecord>> {
        self.with_conn(|conn| {
            let bounded_limit = limit.clamp(1, 24) as i64;
            let trimmed_query = query.trim();

            if trimmed_query.is_empty() {
                return Self::list_memories_filtered_conn(
                    conn,
                    project_id,
                    scope,
                    kind,
                    bounded_limit,
                );
            }

            let fts_query = normalize_fts_query(trimmed_query);
            if !fts_query.is_empty() {
                let fts_result = Self::search_memories_fts_conn(
                    conn,
                    &fts_query,
                    project_id,
                    scope,
                    kind,
                    bounded_limit,
                );
                if let Ok(items) = fts_result {
                    if !items.is_empty() {
                        return Ok(items);
                    }
                }
            }

            let like_items = Self::search_memories_like_conn(
                conn,
                trimmed_query,
                project_id,
                scope,
                kind,
                bounded_limit,
            )?;
            if !like_items.is_empty() {
                return Ok(like_items);
            }

            Self::list_memories_filtered_conn(conn, project_id, scope, kind, bounded_limit)
        })
    }

    fn map_memory_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryRecord> {
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
    }

    fn search_memories_fts_conn(
        conn: &Connection,
        fts_query: &str,
        project_id: Option<&str>,
        scope: Option<&str>,
        kind: Option<&str>,
        limit: i64,
    ) -> rusqlite::Result<Vec<MemoryRecord>> {
        let mut stmt = conn.prepare(
            "SELECT m.id, m.scope, m.project_id, m.kind, m.content, m.tags, m.importance, m.created_at, m.updated_at
             FROM memories_fts f JOIN memories m ON m.id = f.id
             WHERE memories_fts MATCH ?1
               AND (?2 IS NULL OR m.project_id = ?2 OR m.project_id IS NULL)
               AND (?3 IS NULL OR m.scope = ?3)
               AND (?4 IS NULL OR m.kind = ?4)
             ORDER BY m.importance DESC, m.updated_at DESC
             LIMIT ?5",
        )?;
        let rows = stmt.query_map(
            params![fts_query, project_id, scope, kind, limit],
            Self::map_memory_row,
        )?;
        rows.collect::<Result<Vec<_>, _>>()
    }

    fn search_memories_like_conn(
        conn: &Connection,
        query: &str,
        project_id: Option<&str>,
        scope: Option<&str>,
        kind: Option<&str>,
        limit: i64,
    ) -> AppResult<Vec<MemoryRecord>> {
        let like_query = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT m.id, m.scope, m.project_id, m.kind, m.content, m.tags, m.importance, m.created_at, m.updated_at
             FROM memories m
             WHERE (m.content LIKE ?1 OR IFNULL(m.tags, '') LIKE ?1 OR m.kind LIKE ?1 OR m.scope LIKE ?1)
               AND (?2 IS NULL OR m.project_id = ?2 OR m.project_id IS NULL)
               AND (?3 IS NULL OR m.scope = ?3)
               AND (?4 IS NULL OR m.kind = ?4)
             ORDER BY m.importance DESC, m.updated_at DESC
             LIMIT ?5",
        )?;
        let rows = stmt.query_map(
            params![like_query, project_id, scope, kind, limit],
            Self::map_memory_row,
        )?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn list_memories_filtered_conn(
        conn: &Connection,
        project_id: Option<&str>,
        scope: Option<&str>,
        kind: Option<&str>,
        limit: i64,
    ) -> AppResult<Vec<MemoryRecord>> {
        let mut stmt = conn.prepare(
            "SELECT m.id, m.scope, m.project_id, m.kind, m.content, m.tags, m.importance, m.created_at, m.updated_at
             FROM memories m
             WHERE (?1 IS NULL OR m.project_id = ?1 OR m.project_id IS NULL)
               AND (?2 IS NULL OR m.scope = ?2)
               AND (?3 IS NULL OR m.kind = ?3)
             ORDER BY m.importance DESC, m.updated_at DESC
             LIMIT ?4",
        )?;
        let rows = stmt.query_map(
            params![project_id, scope, kind, limit],
            Self::map_memory_row,
        )?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn list_projects(&self) -> AppResult<Vec<ProjectRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, path, kind, rules, status, active_model_profile, created_at, updated_at FROM projects ORDER BY updated_at DESC")?;
            let rows = stmt.query_map([], |row| Self::map_project_row(row))?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        })
    }

    pub fn get_project(&self, id: &str) -> AppResult<Option<ProjectRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, path, kind, rules, status, active_model_profile, created_at, updated_at FROM projects WHERE id = ?1")?;
            let mut rows = stmt.query_map(params![id], |row| Self::map_project_row(row))?;
            if let Some(row) = rows.next() {
                Ok(Some(row?))
            } else {
                Ok(None)
            }
        })
    }

    fn map_project_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectRecord> {
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

    pub fn get_project_state(&self, project_id: &str) -> AppResult<Option<ProjectStateRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT project_id, current_step, last_commit, last_tag, summary, next_step, status, created_at, updated_at
                 FROM project_states WHERE project_id = ?1",
            )?;
            let mut rows = stmt.query_map(params![project_id], |row| Self::map_project_state_row(row))?;
            if let Some(row) = rows.next() {
                Ok(Some(row?))
            } else {
                Ok(None)
            }
        })
    }

    pub fn upsert_project_state(&self, state: &ProjectStateRecord) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO project_states (project_id, current_step, last_commit, last_tag, summary, next_step, status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                 ON CONFLICT(project_id) DO UPDATE SET
                    current_step=excluded.current_step,
                    last_commit=excluded.last_commit,
                    last_tag=excluded.last_tag,
                    summary=excluded.summary,
                    next_step=excluded.next_step,
                    status=excluded.status,
                    updated_at=excluded.updated_at",
                params![
                    state.project_id,
                    state.current_step,
                    state.last_commit,
                    state.last_tag,
                    state.summary,
                    state.next_step,
                    state.status,
                    state.created_at,
                    state.updated_at
                ],
            )?;
            Ok(())
        })
    }

    fn map_project_state_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectStateRecord> {
        Ok(ProjectStateRecord {
            project_id: row.get(0)?,
            current_step: row.get(1)?,
            last_commit: row.get(2)?,
            last_tag: row.get(3)?,
            summary: row.get(4)?,
            next_step: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
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

    pub fn insert_pending_tool_call(&self, pending: &PendingToolCallRecord) -> AppResult<()> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO pending_tool_calls (id, tool_name, project_id, input_json, status, danger_level, created_at, updated_at, decided_at, decision_note)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    pending.id,
                    pending.tool_name,
                    pending.project_id,
                    pending.input_json,
                    pending.status,
                    pending.danger_level,
                    pending.created_at,
                    pending.updated_at,
                    pending.decided_at,
                    pending.decision_note
                ],
            )?;
            Ok(())
        })
    }

    pub fn get_pending_tool_call(&self, id: &str) -> AppResult<Option<PendingToolCallRecord>> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, tool_name, project_id, input_json, status, danger_level, created_at, updated_at, decided_at, decision_note
                 FROM pending_tool_calls WHERE id = ?1",
            )?;
            let mut rows = stmt.query_map(params![id], Self::map_pending_tool_call_row)?;
            if let Some(row) = rows.next() {
                Ok(Some(row?))
            } else {
                Ok(None)
            }
        })
    }

    pub fn list_pending_tool_calls(&self, limit: usize) -> AppResult<Vec<PendingToolCallRecord>> {
        self.with_conn(|conn| {
            let bounded_limit = limit.clamp(1, 100) as i64;
            let mut stmt = conn.prepare(
                "SELECT id, tool_name, project_id, input_json, status, danger_level, created_at, updated_at, decided_at, decision_note
                 FROM pending_tool_calls
                 WHERE status = 'pending'
                 ORDER BY created_at DESC
                 LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![bounded_limit], Self::map_pending_tool_call_row)?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        })
    }

    pub fn update_pending_tool_status(
        &self,
        id: &str,
        status: &str,
        note: Option<&str>,
    ) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE pending_tool_calls
                 SET status = ?1, updated_at = ?2, decided_at = ?2, decision_note = COALESCE(?3, decision_note)
                 WHERE id = ?4",
                params![status, now, note, id],
            )?;
            Ok(())
        })
    }

    fn map_pending_tool_call_row(
        row: &rusqlite::Row<'_>,
    ) -> rusqlite::Result<PendingToolCallRecord> {
        Ok(PendingToolCallRecord {
            id: row.get(0)?,
            tool_name: row.get(1)?,
            project_id: row.get(2)?,
            input_json: row.get(3)?,
            status: row.get(4)?,
            danger_level: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            decided_at: row.get(8)?,
            decision_note: row.get(9)?,
        })
    }

    pub fn recent_tool_runs(&self, limit: usize) -> AppResult<Vec<serde_json::Value>> {
        self.with_conn(|conn| {
            let bounded_limit = limit.clamp(1, 100) as i64;
            let mut stmt = conn.prepare(
                "SELECT id, tool_name, input_json, output_json, success, created_at
                 FROM tool_runs
                 ORDER BY created_at DESC
                 LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![bounded_limit], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, String>(0)?,
                    "tool_name": row.get::<_, String>(1)?,
                    "input_json": row.get::<_, String>(2)?,
                    "output_json": row.get::<_, Option<String>>(3)?,
                    "success": row.get::<_, i64>(4)? == 1,
                    "created_at": row.get::<_, String>(5)?,
                }))
            })?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        })
    }
}
