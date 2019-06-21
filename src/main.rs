use std::error::Error;
use std::path::PathBuf;
use std::{fmt, fs, io, process};

use structopt::StructOpt;
use toml::{de, Value};

#[derive(Debug)]
enum CommandErr {
    TomlError(de::Error),
    IoError(io::Error),
}

impl fmt::Display for CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

#[derive(Debug, StructOpt)]
#[structopt(name = "toml-sorted", about = "Check if Cargo.toml is sorted.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    manifest: PathBuf,
}

fn parse_manifest(options: &Opt) -> Result<Value, CommandErr> {
    let manifest = fs::read_to_string(&options.manifest)?;
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
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<String>>();
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
    let opt = Opt::from_args();
    match parse_manifest(&opt) {
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
