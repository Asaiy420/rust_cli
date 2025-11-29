# 🚀 GemCLI

A fast, lightweight command-line interface for interacting with Google's Gemini AI directly from your terminal.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

## ✨ Features

- 🔥 **Fast & Lightweight** - Built with Rust for optimal performance
- 🤖 **Gemini 2.5 Flash Lite** - Uses Google's latest and fastest AI model
- 💬 **Natural Language** - Ask questions directly from your terminal
- 🎯 **Concise Responses** - Optimized for terminal-friendly output
- 🔐 **Secure** - API key management via environment variables
- ⚡ **Async** - Non-blocking requests with Tokio

## 📋 Prerequisites

- Rust 1.70+ (or latest stable)
- A Google Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))

## 🔧 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Asaiy420/rust_cli.git
cd rust_cli/gemcli

# Build the project
cargo build --release

# The binary will be at target/release/gemcli
```

### Add to PATH (Optional)

**Windows:**

```powershell
# Copy to a directory in your PATH
cp target\release\gemcli.exe C:\Users\YourUsername\.cargo\bin\
```

**Linux/macOS:**

```bash
# Copy to a directory in your PATH
sudo cp target/release/gemcli /usr/local/bin/
```

## ⚙️ Configuration

Create a `.env` file in the project root with your Gemini API key:

```env
GEMINI_API_KEY=your_api_key_here
```

Alternatively, set it as a system environment variable:

**Windows (PowerShell):**

```powershell
$env:GEMINI_API_KEY="your_api_key_here"
```

**Linux/macOS:**

```bash
export GEMINI_API_KEY="your_api_key_here"
```

## 🚀 Usage

### Basic Usage

```bash
gemcli "What is the capital of France?"
```

### Multi-word Prompts

```bash
gemcli explain recursion in simple terms
```

### Code Examples

```bash
gemcli "Write a Python function to reverse a string"
```

### Terminal Commands

```bash
gemcli "How do I list all files in a directory on Linux?"
```

### Quick Help

```bash
gemcli --help
```

## 📚 Examples

```bash
# General questions
gemcli "What is Rust?"

# Programming help
gemcli "Show me a for loop in Rust"

# System administration
gemcli "How to check disk space on Windows?"

# Quick calculations
gemcli "Convert 1GB to MB"

# Debugging help
gemcli "What causes a segmentation fault?"
```

## 🏗️ Project Structure

```
gemcli/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Project dependencies
├── .env                 # API key configuration (create this)
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [Serde](https://serde.rs/) - Serialization framework
- [Dotenvy](https://github.com/allan2/dotenvy) - .env file support

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- Google for providing the Gemini API
- The Rust community for excellent tooling and libraries

## 📞 Support

If you encounter any issues or have questions:

- Open an issue on [GitHub](https://github.com/Asaiy420/rust_cli/issues)
- Check the [Gemini API Documentation](https://ai.google.dev/docs)

---

**Made with ❤️ and Rust**
