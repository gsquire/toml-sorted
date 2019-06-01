use std::error::Error;
use std::{env, fmt, fs};

use toml::Value;

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug)]
enum CommandErr {
    IncorrectArgs,
    NotSorted,
}

impl fmt::Display for CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandErr::IncorrectArgs => write!(f, "incorrect number of arguments supplied"),
            CommandErr::NotSorted => write!(f, "manifest contents are not all sorted"),
        }
    }
}

impl Error for CommandErr {}

fn parse_manifest() -> Res<Value> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(Box::new(CommandErr::IncorrectArgs));
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

fn main() -> Res<()> {
    let manifest = parse_manifest()?;
    if !is_sorted(&manifest) {
        return Err(Box::new(CommandErr::NotSorted));
    }

    Ok(())
}
