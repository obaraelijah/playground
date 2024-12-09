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
  use std::ffi::OsString;
  use std::fs;

  use super::{
    create_project, delete_project, greet, list_projects, ProjectsDir,
  };

  #[test]
  fn test_list_projects() {
    dotenv::dotenv().unwrap();

    let pd = ProjectsDir::from_env().unwrap();

    assert_eq!(list_projects(&pd).unwrap(), Vec::<OsString>::new(),);

    // add a project

    create_project("test 1".into(), &pd).unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec![OsString::from("test 1.db")],
    );

    // add random file, make sure it is excluded from test

    fs::File::create(format!(
      "{}/not a db.txt",
      pd.0.to_str().unwrap()
    ))
    .unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec![OsString::from("test 1.db")],
    );

    // remove random file again

    fs::remove_file(format!(
      "{}/not a db.txt",
      &pd.0.to_str().unwrap()
    ))
    .unwrap();

    // delete project created above

    delete_project("test 1".into(), &pd).unwrap();

    assert_eq!(list_projects(&pd).unwrap(), Vec::<OsString>::new(),);
  }

  #[test]
  fn test_greet() {
    assert_eq!(greet("World"), "Hello World!".to_owned());
  }
}