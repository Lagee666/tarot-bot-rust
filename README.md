# Tarot Bot Rust

A lightweight Tarot Card Reading bot built with Rust and Axum, specifically designed for integration with the LINE Messaging API.

## Features

- **Tarot Readings**: Draw random cards or request specific cards by index.
- **Rich Responses**: Sends both text descriptions and card images (Upright/Reversed).
- **Command Based**: Supports traditional commands like "抽卡" (Draw Card) and "幫助" (Help).
- **Data Driven**: All card information is stored in local JSON files in the `data/` directory.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- A LINE Channel Access Token

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/tarot-bot-rust.git
   cd tarot-bot-rust
   ```

2. **Configure environment variables**:
   Create a `.env` file in the root directory:
   ```bash
   LINE_CHANNEL_ACCESS_TOKEN=your_token_here
   ```

3. **Prepare Data**:
   Ensure the `data/` and `images/` directories contain the necessary JSON and JPG files.

## Running the Server

To start the server locally:

```bash
cargo run
```

The server will start at `http://localhost:5000`. You can use a tool like [ngrok](https://ngrok.com/) to expose it for LINE's webhook.

## Usage

Once connected to LINE, you can use the following commands:

- `抽卡` (or `抽一張`): Draws a random tarot card.
- `幫助` (or `說明`): Shows help information.
- `所有卡片`: Lists all card titles.
- `[0-77]`: Input a number to get the corresponding card's information.

## Project Structure

- `src/main.rs`: Entry point and server configuration.
- `src/handler.rs`: Core logic for handling LINE webhooks.
- `src/tarot_info.rs`: Management of tarot card data and logic.
- `data/`: Contains JSON files for each tarot card.
- `images/`: Contains JPG images for cards in both upright and reversed orientations.

## License

[MIT](LICENSE)
