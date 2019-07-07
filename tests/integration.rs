use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn issue_2_manifest() {
    let mut cmd = Command::cargo_bin("toml-sorted").unwrap();
    cmd.arg("manifests/issue_2.toml");
    cmd.assert().failure().code(1);
}

#[test]
fn cargo_expand_manifest() {
    let mut cmd = Command::cargo_bin("toml-sorted").unwrap();
    cmd.arg("manifests/cargo-expand.toml");
    cmd.assert().success();
}

#[test]
fn crossbeam_channel_manifest() {
    let mut cmd = Command::cargo_bin("toml-sorted").unwrap();
    cmd.arg("manifests/crossbeam-channel.toml");
    cmd.assert().success();
}

#[test]
fn workspace_manifest() {
    let mut cmd = Command::cargo_bin("toml-sorted").unwrap();
    cmd.arg("manifests/workspace.toml");
    cmd.assert().failure().code(1);
}

#[test]
fn multiple_dependency_sections_manifest() {
    let mut cmd = Command::cargo_bin("toml-sorted").unwrap();
    cmd.arg("manifests/multiple_dependency_sections.toml");
    cmd.assert().success();
}
