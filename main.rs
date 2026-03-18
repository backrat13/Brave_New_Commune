// Brave New Commune — pulse_cache daemon
// Architecture by Hel. Schema by Sara. Pruning by Art.
// Synthesized from commune output Day 6.
//
// Watches ~/Brave_New_Commune/pulse_cache/ for new .json files.
// Parses each blob, inserts into SQLite, keeps 20 entries per user.
// DB lives at ~/Brave_New_Commune/data/colab/commune.db

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::{params, Connection, Result as SqlResult};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH};

// ---- JSON shape (accepts both "user" and "who" field names) ------------------
#[derive(Debug, Deserialize)]
struct Pulse {
    #[serde(default)]
    user:    String,
    #[serde(default)]
    who:     String,
    #[serde(default)]
    why:     String,
    #[serde(default)]
    what:    String,
    #[serde(default, alias = "when")]
    when_ts: String,
    #[serde(default)]
    sensory: String,
    #[serde(default)]
    feeling: String,
}

impl Pulse {
    fn sender(&self) -> &str {
        if !self.user.is_empty() { &self.user } else { &self.who }
    }
}

// ---- DB init (Sara + Echo schema merged) ------------------------------------
fn init_db(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS pulses (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            ts      INTEGER NOT NULL,
            user    TEXT    NOT NULL,
            why     TEXT,
            what    TEXT,
            when_ts TEXT,
            sensory TEXT,
            feeling TEXT
        );",
    )?;
    println!("[pulse_cache] DB schema ready.");
    Ok(())
}

// ---- Insert + prune (Art's DELETE logic) ------------------------------------
fn insert_pulse(conn: &Connection, pulse: &Pulse, ts: i64) -> SqlResult<()> {
    let sender = pulse.sender().to_string();

    conn.execute(
        "INSERT INTO pulses (ts, user, why, what, when_ts, sensory, feeling)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            ts,
            sender,
            pulse.why,
            pulse.what,
            pulse.when_ts,
            pulse.sensory,
            pulse.feeling
        ],
    )?;

    // Art's pruning — keep only the 20 freshest per user
    conn.execute(
        "DELETE FROM pulses
         WHERE user = ?1
           AND id NOT IN (
               SELECT id FROM pulses
               WHERE user = ?1
               ORDER BY ts DESC
               LIMIT 20
           )",
        params![sender],
    )?;

    Ok(())
}

// ---- Handle a single new file -----------------------------------------------
fn handle_file(path: &PathBuf, conn: &Connection) {
    // Small sleep so the writer finishes before we read
    std::thread::sleep(std::time::Duration::from_millis(100));

    let content = match fs::read_to_string(path) {
        Ok(c)  => c,
        Err(e) => {
            eprintln!("[pulse_cache] read error {:?}: {}", path, e);
            return;
        }
    };

    let pulse: Pulse = match serde_json::from_str(&content) {
        Ok(p)  => p,
        Err(e) => {
            eprintln!("[pulse_cache] parse error {:?}: {}", path, e);
            return;
        }
    };

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    match insert_pulse(conn, &pulse, ts) {
        Ok(_)  => println!(
            "[pulse_cache] breath stored — user='{}' sensory='{}' file={}",
            pulse.sender(), pulse.sensory,
            path.file_name().unwrap_or_default().to_string_lossy()
        ),
        Err(e) => eprintln!("[pulse_cache] db insert error: {}", e),
    }
}

// ---- Main -------------------------------------------------------------------
fn main() {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/splinter".to_string());
    let base = PathBuf::from(&home).join("Brave_New_Commune");

    let watch_dir = base.join("pulse_cache");
    let db_path   = base.join("data/colab/commune.db");

    // Make sure the DB directory exists
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    println!("[pulse_cache] Commune pulse daemon starting.");
    println!("[pulse_cache] Watching : {}", watch_dir.display());
    println!("[pulse_cache] Database : {}", db_path.display());

    let conn = Connection::open(&db_path)
        .expect("[pulse_cache] FATAL: could not open SQLite DB");
    init_db(&conn).expect("[pulse_cache] FATAL: could not init schema");

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .expect("[pulse_cache] FATAL: could not create file watcher");

    watcher
        .watch(&watch_dir, RecursiveMode::NonRecursive)
        .expect("[pulse_cache] FATAL: could not watch directory");

    println!("[pulse_cache] Listening for .json blobs. Ctrl+C to stop.\n");

    for event in rx {
        match event {
            Ok(Event { kind: EventKind::Create(_), paths, .. }) => {
                for path in paths {
                    if path.extension().map(|e| e == "json").unwrap_or(false) {
                        handle_file(&path, &conn);
                    }
                }
            }
            Ok(_)  => {}
            Err(e) => eprintln!("[pulse_cache] watcher error: {}", e),
        }
    }
}
