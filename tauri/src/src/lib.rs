use anyhow::anyhow;

use std::env;
use std::ffi::OsString;
use std::fs;

#[derive(Debug)]
struct ProjectsDir(pub OsString);

use home::home_dir;
use std::path::PathBuf;

impl ProjectsDir {
  fn from_env() -> anyhow::Result<Self> {
    let projects_dir = env::var("PROJECTS_DIR")?;
    
    let full_path = if projects_dir.starts_with("~/") {
      home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(&projects_dir[2..])
    } else {
      PathBuf::from(projects_dir)
    };

    std::fs::create_dir_all(&full_path)?;

    Ok(Self(full_path.as_os_str().to_os_string()))
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

fn create_project(
  name: OsString,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let mut file_name = dir.0.clone();
  file_name.push(name);
  file_name.push(".db");

  fs::File::options()
    .read(true)
    .write(true)
    .create_new(true)
    .open(file_name)?;

  Ok(())
}

fn delete_project(
  name: OsString,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let mut file_name = dir.0.clone();
  file_name.push(name);
  file_name.push(".db");

  Ok(fs::remove_file(file_name)?)
}

fn list_projects(dir: &ProjectsDir) -> anyhow::Result<Vec<OsString>> {
  let mut projects = Vec::new();

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
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs;

    #[test]
    fn test_list_projects() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let projects_path = temp_dir.path();

        fs::create_dir_all(projects_path).expect("Failed to create projects directory");

        let pd = ProjectsDir(projects_path.as_os_str().to_os_string());

        println!("Projects Directory: {:?}", projects_path);

        assert!(projects_path.exists(), "Projects directory does not exist");
        assert!(projects_path.is_dir(), "Projects path is not a directory");

        let initial_projects = list_projects(&pd)
            .expect("Failed to list initial projects");
        assert!(initial_projects.is_empty(), "Initial projects list should be empty");

        create_project("test 1".into(), &pd).expect("Failed to create project");

        let projects_after_create = list_projects(&pd)
            .expect("Failed to list projects after creation");
        assert_eq!(
            projects_after_create,
            vec![OsString::from("test 1.db")],
            "Project list after creation is incorrect"
        );

        fs::File::create(projects_path.join("not a db.txt"))
            .expect("Failed to create test file");

        let projects_with_extra_file = list_projects(&pd)
            .expect("Failed to list projects with extra file");
        assert_eq!(
            projects_with_extra_file,
            vec![OsString::from("test 1.db")],
            "Project list should only include .db files"
        );

        delete_project("test 1".into(), &pd)
            .expect("Failed to delete project");

        let final_projects = list_projects(&pd)
            .expect("Failed to list final projects");
        assert!(final_projects.is_empty(), "Final projects list should be empty");
    }
}