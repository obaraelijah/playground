use example_app::schema;

fn main() -> anyhow::Result<()> {
  dotenv::dotenv()?;
  
  println!("{}", schema()?.sdl());
  
  Ok(())
}