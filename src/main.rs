use std::path::PathBuf;

use anyhow::Context;
use toml::{Table, Value};

// Represents a parsed manifest with a field maintaining the file contents.
struct Manifest {
    table: Table,
    parsed_file: String,
}

#[derive(Debug, argh::FromArgs)]
/// Check if your Cargo manifest is sorted.
struct Opt {
    /// the manifest to check
    #[argh(positional)]
    manifest: PathBuf,
}

fn parse_manifest(options: &Opt) -> anyhow::Result<Manifest> {
    let manifest = std::fs::read_to_string(&options.manifest).with_context(|| {
        format!(
            "failed to read manifest at path {}",
            options.manifest.display()
        )
    })?;
    let table = manifest
        .parse::<Table>()
        .context("failed to parse manifest")?;
    Ok(Manifest {
        table,
        parsed_file: manifest,
    })
}

fn check_workspace_by_key(workspace: &Value, key: &str) -> bool {
    workspace.get(key).is_none_or(|v| {
        let arrays = v.as_array().unwrap();
        let values = arrays
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>();
        values.windows(2).all(|w| w[0] <= w[1])
    })
}

fn check_workspace(manifest: &Table) -> bool {
    manifest.get("workspace").is_none_or(|v| {
        check_workspace_by_key(v, "exclude") && check_workspace_by_key(v, "members")
    })
}

fn check_deps_by_key(manifest: &Manifest, key: &str) -> bool {
    manifest.table.get(key).is_none_or(|v| {
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
    check_workspace(&manifest.table)
        && check_deps_by_key(manifest, "build-dependencies")
        && check_deps_by_key(manifest, "dependencies")
        && check_deps_by_key(manifest, "dev-dependencies")
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = argh::from_env();
    parse_manifest(&opt).and_then(|m| {
        if !is_sorted(&m) {
            return Err(anyhow::anyhow!("manifest is not sorted"));
        }
        Ok(())
    })
}
