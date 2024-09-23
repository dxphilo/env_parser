use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

#[derive(Debug, Clone)]
pub struct Env {
    pub vars: HashMap<String, String>,
}

impl Env {
    pub fn load() -> Env {
        let mut file = File::open(".env").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let mut env_vars = HashMap::new();

        for line in contents.lines() {
            if line.starts_with("#") || line.is_empty() {
                continue;
            }

            let parts = line.split("=").collect::<Vec<&str>>();
            env_vars.insert(parts[0].to_string(), parts[1].to_string());
        }

        Env { vars: env_vars }
    }
}

fn main() {
    let env = Env::load();

    for (key, value) in env.vars.into_iter() {
        println!("Key:{}, Value: {:?}", key, value)
    }
}
