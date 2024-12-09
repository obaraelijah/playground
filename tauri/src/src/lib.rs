use std::env;

struct ProjectsDir(pub String);

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

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
  Ok(
    tauri::Builder::default()
      .manage(ProjectsDir(env::var("PROJECTS_DIR")?))
      .invoke_handler(tauri::generate_handler![greet])
      .build(tauri::generate_context!())?,
  )
}

#[cfg(test)]
mod tests {
  use super::greet;

  #[test]
  fn test_greet() {
    assert_eq!(greet("World"), "Hello World!".to_owned());
  }
}