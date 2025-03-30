# discord_updater

A simple CLI tool for Arch Linux users to update discord version through `build_info.json`



## Installation

```bash
# Clone the repo
git clone https://github.com/VulnX/discord_updater.git
cd discord_updater
# Compile it yourself
cargo build --release
# Move to `bin` directory
sudo mv ./target/release/discord_updater /usr/local/bin
# Enable setuid bit
sudo chown -R root:$(whoami) /usr/local/bin/discord_updater
sudo chmod +s /usr/local/bin/discord_updater
```

## Usage

```bash
$ discord_updater -h
A simple CLI tool for Arch Linux users to update discord version through `build_info.json`

Usage: discord_updater [OPTIONS]

Options:
  -p, --path <PATH>      Path to `build_info.json` [default: /opt/discord/resources/build_info.json]
  -u, --update <UPDATE>  (Optional) Manually specify the new version like "0.0.89"
  -h, --help             Print help
  -V, --version          Print version
```


