use tauri::App;
use std::path::PathBuf;
use tauri::Config;
use rusqlite::{params, Connection};
use std::sync::Mutex;
use tauri::State;
use thiserror::Error;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
  setup: Option<SetupHook>,
}

impl AppBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn setup<F>(mut self, setup: F) -> Self
  where
    F: FnOnce(&mut App) -> Result<(), 
    Box<dyn std::error::Error>> + Send + 'static,
  {
    self.setup.replace(Box::new(setup));
    self

  }

  pub fn run(self) {
    let setup = self.setup;
    eprintln!("\nStarting Application:\n");
    tauri::Builder::default()
      .manage(Database(Mutex::new(create_database_connection())))
      .invoke_handler(tauri::generate_handler![delete_session, save_session, load_n_sessions, unarchive_project, save_project, archive_project, delete_project, get_archived_projects, complete_session, get_active_projects, create_project, start_session, get_project_sessions_after_date, get_sessions_after_date, get_sessions_in_range])
      .setup(move |app| {
        #[cfg(target_os = "android")] {
          use tauri::Manager;
          let main_window = app.get_window("main").unwrap();
          main_window.with_webview(|webview| {
              eprintln!("\nWebview:\n\n");
              use jni::objects::JValue;
              webview.jni_handle().exec(|env, _, webview| {
                env.call_method(webview, "setBackgroundColor", "(I)V", &[tauri::wry::application::platform::android::ndk_glue::jni::objects::JValue::Int(0)]).unwrap();
              });
          });
        }
        if let Some(setup) = setup {
          (setup)(app)?;
        }
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
}


#[cfg(target_os = "android")]
pub fn get_app_data_dir() -> String {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let env = vm.attach_current_thread().unwrap();

    let context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };

    let dir = env
        .call_method(context, "getFilesDir", "()Ljava/io/File;", &[]).unwrap()
        .l().unwrap();

    let dir_str = env.get_string(
        env
            .call_method(dir, "getAbsolutePath", "()Ljava/lang/String;", &[]).unwrap()
            .l().unwrap()
            .into(),
    ).unwrap();

    dir_str.to_str().unwrap().to_string()
}

// Database connection

use serde_json::Value;
use serde;
use serde::{Deserialize, Serialize};

struct Database(Mutex<Connection>);

#[derive(Serialize, Deserialize)]
struct Project {
  id:         i32,
  name:       String,
  color:      String,
  target:     i32,
  archived:   i32,
  active:     i32
}

#[derive(Serialize, Deserialize)]
struct Session {
  id:         i32,
  project_id: i32,
  start:      String,
  end:        Option<String>,
  target:     i32
}

#[derive(Serialize, Deserialize)]
struct ProjectSession {
  id:         i32,
  start:      String,
  end:        Option<String>,
  target:     i32,
  project_id: i32,
  name:       String,
  color:      String
}

fn create_database_connection() -> Connection {
  let database_location = get_app_data_dir() + "/database.db";
  let mut connection = Connection::open(database_location).unwrap();

  // connection.execute(
  //   "DROP TABLE IF EXISTS session",
  //   params![],
  // ).unwrap();

  // eprintln!("dropped sessions");

  // connection.execute(
  //   "DROP TABLE IF EXISTS project",
  //   params![],
  // ).unwrap(); 

  // eprintln!("dropped projects");

  connection.execute(
      "CREATE TABLE IF NOT EXISTS project (
                id            INTEGER   PRIMARY KEY,
                name          TEXT      NOT NULL,
                color         TEXT      NOT NULL,
                target        INTEGER   NOT NULL,
                archived      BOOLEAN   DEFAULT FALSE   NOT NULL,
                active        BOOLEAN   DEFAULT FALSE   NOT NULL
                )",
      params![],
  ).unwrap();

  eprintln!("created projects");

  connection.execute(
    "CREATE TABLE IF NOT EXISTS session (
              id              INTEGER   PRIMARY KEY,
              project_id      INTEGER   NOT NULL,
              start           TIMESTAMP DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
              end             TIMESTAMP,
              target          INTEGER   NOT NULL,
              CONSTRAINT fk_project
                FOREIGN KEY (project_id)
                REFERENCES project(id)
              )",
    params![],
  ).unwrap();

  // Seeding

  // connection.execute(
  //   "DELETE FROM session",
  //   params![],
  // ).unwrap();

  // connection.execute(
  //   "DELETE FROM project",
  //   params![],
  // ).unwrap();

  // connection.execute(
  //   "INSERT INTO project (name, color, target) VALUES (?1, ?2, ?3)",
  //   params!["Learn German".to_string(), "9c6cf0".to_string(), 0],
  // ).unwrap();
  // connection.execute(
  //   "INSERT INTO project (name, color, target) VALUES (?1, ?2, ?3)",
  //   params!["Exercise".to_string(), "92d65a".to_string(), 1],
  // ).unwrap();
  // connection.execute(
  //   "INSERT INTO project (name, color, target) VALUES (?1, ?2, ?3)",
  //   params!["PSA Development".to_string(), "f56f49".to_string(), 0],
  // ).unwrap();

  return connection;
}

// PROJECTS

#[tauri::command]
async fn get_active_projects(database: tauri::State<'_, Database>) -> Result<String, Error> {
  let connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare("SELECT * FROM project WHERE archived = 0")?;
  
  let project_iter = stmt.query_map(params![], |row| {
      Ok(Project {
          id: row.get(0)?,
          name: row.get(1)?,
          color: row.get(2)?,
          target: row.get(3)?,
          archived: row.get(4)?,
          active: row.get(5)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}

#[tauri::command]
async fn get_archived_projects(database: tauri::State<'_, Database>) -> Result<String, Error> {
  let connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare("SELECT * FROM project WHERE archived = 1")?;
  
  let project_iter = stmt.query_map(params![], |row| {
      Ok(Project {
          id: row.get(0)?,
          name: row.get(1)?,
          color: row.get(2)?,
          target: row.get(3)?,
          archived: row.get(4)?,
          active: row.get(5)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}

#[tauri::command]
async fn create_project(database: tauri::State<'_, Database>, name: String, color: String, target: i32) -> Result<String, Error> {
  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  connection.execute(
    "INSERT INTO project (name, color, target) VALUES (?1, ?2, ?3)",
    params![name, color, target],
  )?;

  Ok("Success".into())
}

#[tauri::command]
async fn save_project(database: tauri::State<'_, Database>, project_id: i32, name: String, color: String, target: i32) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE project
    SET
      name = ?1,
      color = ?2,
      target = ?3
    WHERE
      id = ?4",
    params![name, color, target, project_id],
  )?;

  transaction.execute(
    "UPDATE session
    SET 
      target = ?1
    WHERE
      end IS NULL AND project_id = ?2",
    params![target, project_id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn save_session(database: tauri::State<'_, Database>, id: i32, start: String, end: Option<String>) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE session
    SET
      start = ?1,
      end = ?2
    WHERE
      id = ?3",
    params![start, end, id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn archive_project(database: tauri::State<'_, Database>, project_id: i32) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE project
    SET
      archived = 1,
      active = 0
    WHERE
      id = ?1",
    params![project_id],
  )?;
  
  transaction.execute(
    "UPDATE session
    SET 
      end = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
    WHERE
      end IS NULL AND project_id = ?1",
    params![project_id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn unarchive_project(database: tauri::State<'_, Database>, project_id: i32) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE project
    SET
      archived = 0
    WHERE
      id = ?1",
    params![project_id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn delete_project(database: tauri::State<'_, Database>, project_id: i32) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "DELETE FROM session
    WHERE
      project_id = ?1",
    params![project_id],
  )?;

  transaction.execute(
    "DELETE FROM project
    WHERE
      id = ?1",
    params![project_id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn delete_session(database: tauri::State<'_, Database>, id: i32) -> Result<String, Error> {

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "DELETE FROM session
    WHERE
      id = ?1",
    params![id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn start_session(database: tauri::State<'_, Database>, project_id: i32, target: i32) -> Result<String, Error> {
  println!("starting session");

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE project
    SET active = 0
    WHERE active = 1",
    params![],
  )?;

  transaction.execute(
    "UPDATE session
    SET end = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
    WHERE end IS NULL",
    params![],
  )?;

  transaction.execute(
    "INSERT INTO session (project_id, target) VALUES (?1, ?2)",
    params![project_id, target],
  )?;

  transaction.execute(
    "UPDATE project
    SET 
      active = 1
    WHERE
      id = ?1",
    params![project_id],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn complete_session(database: tauri::State<'_, Database>) -> Result<String, Error> {
  println!("completing session: ");

  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut transaction = connection.transaction()?;

  transaction.execute(
    "UPDATE session 
    SET
      end = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
    WHERE
      end IS NULL",
    params![],
  )?;

  transaction.execute(
    "UPDATE project
    SET 
      active = 0
    WHERE
      active = 1",
    params![],
  )?;

  transaction.commit()?;

  Ok("Success".into())
}

#[tauri::command]
async fn get_sessions_after_date(database: tauri::State<'_, Database>, date : String) -> Result<String, Error> {
  
  println!("getting sessions");
  
  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare("SELECT * FROM session WHERE end > ?1 OR end IS NULL")?;

  let project_iter = stmt.query_map(params![date], |row| {
      Ok(Session {
          id: row.get(0)?,
          project_id: row.get(1)?,
          start: row.get(2)?,
          end: row.get(3).unwrap(),
          target: row.get(4)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}

#[tauri::command]
async fn get_sessions_in_range(database: tauri::State<'_, Database>, start : String, end : String) -> Result<String, Error> {
  
  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare("SELECT * FROM session WHERE (end > ?1 OR end IS NULL) AND start < ?2")?;

  let project_iter = stmt.query_map(params![start, end], |row| {
      Ok(Session {
          id: row.get(0)?,
          project_id: row.get(1)?,
          start: row.get(2)?,
          end: row.get(3).unwrap(),
          target: row.get(4)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}

#[tauri::command]
async fn get_project_sessions_after_date(database: tauri::State<'_, Database>, date : String, project_id: i32) -> Result<String, Error> {
  
  println!("getting sessions");
  
  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare("
  SELECT 
    * 
  FROM 
    session 
  WHERE 
    end > ?1 
  AND 
    project_id = ?2")?;

  let project_iter = stmt.query_map(params![date, project_id], |row| {
      Ok(Session {
          id: row.get(0)?,
          project_id: row.get(1)?,
          start: row.get(2)?,
          end: row.get(3).unwrap(),
          target: row.get(4)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}

#[tauri::command]
async fn load_n_sessions(database: tauri::State<'_, Database>, offset: i32, limit: i32) -> Result<String, Error> {
  
  let mut connection = match database.0.lock() {
    Err(_) => return Err(Error::ConnectionFailed),
    Ok(v) => v,
  };

  let mut stmt = connection.prepare(
    "SELECT 
      session.id,
      session.start,
      session.end,
      session.target,
      project.id,
      project.name,
      project.color
    FROM 
      session
    INNER JOIN project ON
      project.id = session.project_id
    WHERE
      session.end IS NOT NULL
    ORDER BY 
      session.start DESC
    LIMIT 
      ?1 
    OFFSET 
      ?2"
  )?;

  let project_iter = stmt.query_map(params![limit, offset], |row| {
      Ok(ProjectSession {
          id: row.get(0)?,
          start: row.get(1)?,
          end: row.get(2).unwrap(),
          target: row.get(3)?,
          project_id: row.get(4)?,
          name: row.get(5)?,
          color: row.get(6)?
      })
  })?;

  let mut projects = Vec::new();
  for project in project_iter {
    projects.push(project?);
  }
  
  Ok(serde_json::to_string(&projects)?)
}


// Errors

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("Unable to establish database connection")]
  ConnectionFailed,
  #[error(transparent)]
  SQLiteError(#[from] rusqlite::Error),
  #[error(transparent)]
  SerdeJsonString(#[from] serde_json::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}