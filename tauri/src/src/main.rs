#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]
  
use std::env;

struct ProjectsDir(String);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    tauri::Builder::default()
        .manage(ProjectsDir(env::var("PROJECTS_DIR")?))
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())?;
    Ok(())
}

#[cfg(test)]
mod tests {
  use super::greet;

  #[test]
  fn test_greet() {
    assert_eq!(greet("World"), "Hello World!".to_owned());
  }
}
