use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use proc_marco::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: String,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(["build".to_owned()].to_vec())
        .current_dir("./".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}