use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut env_file = File::open(".env")?;
    let mut discord_env_var = String::new();
    env_file.read_to_string(&mut discord_env_var)?;
    println!("{}", format!("cargo:rustc-env={}", discord_env_var));
    Ok(())
}