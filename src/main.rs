use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

/// A simple CLI tool for Arch Linux users to update discord version through `build_info.json`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[group(skip)]
struct Args {
    /// Path to `build_info.json`
    #[arg(
        short,
        long,
        default_value_t = String::from("/opt/discord/resources/build_info.json"),
    )]
    path: String,

    /// (Optional) Manually specify the new version like "0.0.89"
    #[arg(short, long)]
    update: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BuildInfo {
    release_channel: String,
    version: String,
}

fn increment_version(version: &String) -> String {
    let mut parts: Vec<u32> = version
        .split(".")
        .map(|part| part.parse().unwrap())
        .collect();
    if let Some(last_part) = parts.last_mut() {
        *last_part += 1;
    }
    parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join(".")
}

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Read file to buffer
    let build_info_path = PathBuf::from(args.path);
    let mut file = File::open(&build_info_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    // Parse buffer into `BuildInfo` struct
    let mut build_info = serde_json::from_str::<BuildInfo>(&file_contents).unwrap();

    // Update BuildInfo.version to `new_version` if an `update` CLI argument is specified, else
    // simply increment the `patch` version (see https://semver.org/) in the parsed `build_info`
    match args.update {
        Some(new_version) => {
            build_info.version = new_version;
        }
        None => {
            build_info.version = increment_version(&build_info.version);
        }
    }

    // Write the updated `build_info` struct, parsed as json into the dest file
    file_contents = serde_json::to_string(&build_info).unwrap();
    file = File::create(&build_info_path).unwrap();
    file.write_all(file_contents.as_bytes()).unwrap();
}
