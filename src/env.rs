use std::{collections::HashMap, env::{self}};

// Return the env var value or empty string
fn get_env_var(name: String) -> String {
    match env::var(name) {
        Ok(env_value) => {
            env_value
        }
        Err(_) => {
            "".to_string()
        }
    }
}

/**
 * Given String::"{\"version\": \"1.0.0\",\"os\": \"${ENV_OS_NAME}\",\"os_version\": \"${ENV_OS_VERSION}\"}"
 * Returns HashMap::{${ENV_OS_NAME}: <ENV_OS_NAME value>, ${ENV_OS_VERSION}: <ENV_OS_VERSION value>}
 */
pub fn collect_env_vars(content: &String) -> Result<HashMap<String, String>, ()> {
    let mut chars = content.chars();
    let mut result: HashMap<String, String> = HashMap::new();

    let mut should_collect = false;
    let mut collected_var_name = String::new();

    while let Some(c) = chars.next() {
        match c {
            // Identify the start of the env variable
            '$' => {
                let next_char = chars.next().unwrap();

                if next_char == '{' {
                    should_collect = true;
                }
            }
            // Get the collected var name
            // Ge the value from os and save in the hashmap
            '}' => {
                if should_collect {
                    result.insert(
                        format!(
                            "{}{}{}",
                            "${".to_string(),
                            collected_var_name.to_string(),
                            "}".to_string()
                        ),
                        // Get the value for var name
                        get_env_var(collected_var_name.to_string()),
                    );
                    
                    should_collect = false;
                    collected_var_name = String::new();
                }
            }
            // Collect the var name
            _ => {
                if should_collect {
                    collected_var_name += &c.to_string();
                }
            }
        }
    }

    Ok(result)
}

// Get the replacement content to save into the file
pub fn replace_env_vars(content: &String, env_vars: HashMap<String, String>) -> Result<String, ()> {
    let mut new_content = content.clone();

    for (env_var, env_value) in env_vars.into_iter() {
        new_content = new_content.replace(&env_var, &env_value);
    }

    Ok(new_content)
}