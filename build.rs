use std::env;
fn main()
{
  env_logger::init();
  env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();
}
