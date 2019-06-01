use std::error::Error;
use std::{env, fmt, fs, io, process};

use toml::{de, Value};

#[derive(Debug)]
enum CommandErr {
    IncorrectArgs,
    TomlError(de::Error),
    IoError(io::Error),
}

impl fmt::Display for CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandErr::IncorrectArgs => write!(f, "incorrect number of arguments supplied"),
            CommandErr::TomlError(_) => write!(f, "could not parse manifest"),
            CommandErr::IoError(_) => write!(f, "could not read manifest"),
        }
    }
}

impl Error for CommandErr {}

impl From<de::Error> for CommandErr {
    fn from(e: de::Error) -> Self {
        CommandErr::TomlError(e)
    }
}

impl From<io::Error> for CommandErr {
    fn from(e: io::Error) -> Self {
        CommandErr::IoError(e)
    }
}

fn parse_manifest() -> Result<Value, CommandErr> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(CommandErr::IncorrectArgs);
    }

    let manifest = fs::read_to_string(&args[1])?;
    let v = manifest.parse::<Value>()?;
    Ok(v)
}

fn check_workspace_by_key(workspace: &Value, key: &str) -> bool {
    if let Some(v) = workspace.get(key) {
        let values = v
            .as_array()
            .unwrap()
            .into_iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<&str>>();
        return values.windows(2).all(|w| w[0] <= w[1]);
    }
    true
}

fn check_workspace(manifest: &Value) -> bool {
    if let Some(v) = manifest.get("workspace") {
        return check_workspace_by_key(&v, "exclude") && check_workspace_by_key(&v, "members");
    }
    true
}

fn check_deps_by_key(manifest: &Value, key: &str) -> bool {
    if let Some(v) = manifest.get(key) {
        let values = v
            .as_table()
            .unwrap()
            .values()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<&str>>();
        return values.windows(2).all(|w| w[0] <= w[1]);
    }
    true
}

fn is_sorted(manifest: &Value) -> bool {
    check_workspace(manifest)
        && check_deps_by_key(manifest, "build-dependencies")
        && check_deps_by_key(manifest, "dependencies")
        && check_deps_by_key(manifest, "dev-dependencies")
}

fn main() {
    match parse_manifest() {
        Ok(m) => {
            if !is_sorted(&m) {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
