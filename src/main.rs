mod env;
mod file;

use env::{collect_env_vars, replace_env_vars};
use file::{get_file_path, read_file, save_to_file};
use std::env as os_env;

use crate::file::get_output_path;

fn main() -> Result<(), ()> {
    let args: Vec<String> = os_env::args().collect();

    let file_path = get_file_path(&args)?;
    print!("Path {:?}", file_path);

    let file_output_path = get_output_path(&args)?;
    print!("Output {:?}", file_output_path);

    let file_content = read_file(file_path).unwrap();
    print!("Content {}", file_content);

    let env_vars = collect_env_vars(&file_content)?;
    print!("Vars {:?}", env_vars);

    let new_content = replace_env_vars(&file_content, env_vars)?;
    print!("New content {}", new_content);

    save_to_file(new_content, file_output_path);
    Ok(())
}
