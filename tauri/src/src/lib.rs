use anyhow::anyhow;

use std::env;
use std::ffi::OsString;
use std::fs;

#[derive(Debug)]
struct ProjectsDir(pub OsString);

impl ProjectsDir {
  fn from_env() -> anyhow::Result<Self> {
    Ok(Self(OsString::from(env::var("PROJECTS_DIR")?)))
  }
}

struct Entry {
  id: u32,
  title: String,
  body: String,
  published: bool,
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello {}!", name)
}

fn create_project(name: OsString) -> anyhow::Result<()> {
  todo!()
}

fn list_projects(dir: &ProjectsDir) -> anyhow::Result<Vec<OsString>> {
  let mut projects = Vec::new();

  println!("{:?}", dir);

  for entry in fs::read_dir(&dir.0)? {
    let file_name = entry?.file_name();

    let file_ext = file_name
      .to_str()
      .ok_or(anyhow!("error reading file"))?
      .split(".")
      .last()
      .ok_or(anyhow!("error reading file"))?;

    if file_ext == "db" {
      projects.push(file_name);
    }
  }

  Ok(projects)
}

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
  Ok(
    tauri::Builder::default()
      .manage(ProjectsDir::from_env()?)
      .invoke_handler(tauri::generate_handler![greet])
      .build(tauri::generate_context!())?,
  )
}

#[cfg(test)]
mod tests {
  use std::ffi::OsString;

  use super::{greet, list_projects, ProjectsDir};

  #[test]
  fn test_list_projects() {
    dotenv::dotenv().unwrap();

    assert_eq!(
      list_projects(&ProjectsDir::from_env().unwrap()).unwrap(),
      Vec::<OsString>::new(),
    );
  }

  #[test]
  fn test_greet() {
    assert_eq!(greet("World"), "Hello World!".to_owned());
  }
}