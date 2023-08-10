# Rori Discord Bot

This is a Discord bot written in Rust that demonstrates basic command handling using both prefix and slash commands. The bot in future will also integrates with GPT-2 for generating text responses.

## 🛠️ Development

Make sure you have cargo installed

```sh
# Clone the repository and enter the directory
git clone https://github.com/UnknownRori/rori-discord-bot
cd rori-discord-bot

# Build the project
cargo build

# Test the project
cargo test
```

## 🚀 Deployment for Shuttle

Make sure you installed shuttle

```sh
cargo install cargo-shuttle
```

```sh
# Enter directory of the cloned repository
cd rori-discord-bot

# Copy the example secret file
cp ./Secrets.toml.example ./Secrets.toml

# Enter credentials and other stuff
vim ./Secrets.toml

# Deploy the app
cargo shuttle deploy
```

## 🌟 Contribution

Feel free to contribute, send pull request or issue and i will take a look

## 📑 License

[BSD-3 Clause License](./LICENSE)
