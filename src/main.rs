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

// Represents a parsed manifest with a field maintaining the file contents.
struct Manifest {
    value: Value,
    parsed_file: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "toml-sorted", about = "Check if Cargo.toml is sorted.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    manifest: PathBuf,
}

fn parse_manifest(options: &Opt) -> Result<Manifest, CommandErr> {
    let manifest = fs::read_to_string(&options.manifest)?;
    let v = manifest.parse::<Value>()?;
    Ok(Manifest {
        value: v,
        parsed_file: manifest,
    })
}

fn check_workspace_by_key(workspace: &Value, key: &str) -> bool {
    workspace.get(key).map_or(true, |v| {
        let arrays = v.as_array().unwrap();
        let values = arrays
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>();
        values.windows(2).all(|w| w[0] <= w[1])
    })
}

fn check_workspace(manifest: &Value) -> bool {
    manifest.get("workspace").map_or(true, |v| {
        check_workspace_by_key(&v, "exclude") && check_workspace_by_key(&v, "members")
    })
}

fn check_deps_by_key(manifest: &Manifest, key: &str) -> bool {
    manifest.value.get(key).map_or(true, |v| {
        let values = v.as_table().unwrap().keys().collect::<Vec<_>>();
        // Do another check to see if we have a special case of [key.value] inside of the file.
        if !values.windows(2).all(|w| w[0] <= w[1]) {
            let checks = values
                .windows(2)
                .filter(|w| w[1] < w[0])
                .map(|w| w[1])
                .collect::<Vec<_>>();
            for c in checks {
                if !manifest.parsed_file.contains(&format!("[{}.{}]", key, c)) {
                    return false;
                }
            }
        }
        true
    })
}

fn is_sorted(manifest: &Manifest) -> bool {
    check_workspace(&manifest.value)
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
