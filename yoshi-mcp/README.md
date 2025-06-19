# Yoshi MCP - Local LLM VS Code Extension

A powerful VS Code extension that brings local LLM inference capabilities directly to your development environment using the Yoshi error handling framework, Candle ML framework, and WebAssembly for high performance.

## 🚀 Features

- **Local LLM Inference**: Run QWEN, Llama, and other GGUF models locally without external API calls
- **High Performance**: WebAssembly-powered inference with GPU acceleration support (CUDA, Metal, MKL)
- **Yoshi Error Handling**: Complete integration with the Yoshi framework for robust error management
- **MCP Server**: Model Context Protocol server for seamless VS Code integration
- **Security First**: Sandboxed execution, input validation, and resource management
- **Streaming Generation**: Real-time text generation with progress indicators
- **Intelligent Caching**: LRU cache for improved performance and reduced inference time
- **Model Management**: Easy model download, verification, and switching

## 📋 Requirements

- VS Code 1.85.0 or higher
- Rust 1.87+ (for building WASM components)
- Node.js 18+ (for TypeScript compilation)
- wasm-pack (for WebAssembly builds)

### Optional for GPU Acceleration

- NVIDIA GPU with CUDA support
- macOS with Metal support
- Intel CPU with MKL support

## 🛠️ Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/arcmoonstudios/yoshi
cd yoshi/yoshi-mcp
```

2. Install dependencies:
```bash
npm install
```

3. Build WebAssembly components:
```bash
# On Unix/Linux/macOS
chmod +x scripts/build-wasm.sh
./scripts/build-wasm.sh

# On Windows
scripts/build-wasm.bat
```

4. Compile TypeScript:
```bash
npm run compile
```

5. Package the extension:
```bash
npm run package
```

6. Install in VS Code:
```bash
code --install-extension yoshi-mcp-0.1.0.vsix
```

## 🎯 Quick Start

1. **Install the Extension**: Follow the installation steps above

2. **Download a Model**: Use the command palette (`Ctrl+Shift+P`) and run:
   ```
   Yoshi MCP: Download Model
   ```

3. **Select a Model**: Choose from recommended models or specify your own GGUF file:
   ```
   Yoshi MCP: Select Model
   ```

4. **Generate Text**: Select some text in your editor and press `Ctrl+Shift+G` or use:
   ```
   Yoshi MCP: Generate Text with Local LLM
   ```

## 🔧 Configuration

Configure the extension through VS Code settings:

```json
{
  "yoshiMcp.modelPath": "/path/to/your/model.gguf",
  "yoshiMcp.maxTokens": 200,
  "yoshiMcp.temperature": 0.7,
  "yoshiMcp.enableGPU": true,
  "yoshiMcp.cacheSize": 100,
  "yoshiMcp.autoDownloadModels": false
}
```

### Configuration Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `modelPath` | string | `""` | Path to local GGUF model file |
| `maxTokens` | number | `200` | Maximum tokens to generate |
| `temperature` | number | `0.7` | Sampling temperature (0.0-2.0) |
| `enableGPU` | boolean | `true` | Enable GPU acceleration |
| `cacheSize` | number | `100` | Number of cached inference results |
| `autoDownloadModels` | boolean | `false` | Auto-download recommended models |

## 🎮 Commands

| Command | Keybinding | Description |
|---------|------------|-------------|
| `Yoshi MCP: Generate Text` | `Ctrl+Shift+G` | Generate text from selected prompt |
| `Yoshi MCP: Select Model` | - | Choose and load a model |
| `Yoshi MCP: Download Model` | - | Download recommended models |
| `Yoshi MCP: Show Status` | - | Display extension status and metrics |

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                          Yoshi-MCP                          │
│  ┌─────────────────┐    ┌──────────────────────────────┐    │
│  │  TypeScript     │    │       MCP Server             │    │
│  │     Shell       │◄──►│    (Context Protocol)        │    │
│  │                 │    │                              │    │
│  │  ┌─────────────┐│    │  ┌─────────────────────────┐ │    │
│  │  │ WebAssembly ││    │  │    Candle Inference     │ │    │
│  │  │   Bridge    ││    │  │      Engine (Rust)      │ │    │
│  │  │             ││    │  │                         │ │    │
│  │  │  ┌─────────┐││    │  │   ┌─────────────────────┤ │    │
│  │  │  │ GGUF    │││    │  │   │ QWEN/Llama Models   │ │    │
│  │  │  │ Models  │││    │  │   │   (Quantized)       │ │    │
│  │  │  └─────────┘││    │  │   └─────────────────────┤ │    │
│  │  └─────────────┘│    │  └─────────────────────────┘ │    │
│  └─────────────────┘    └──────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

- **TypeScript Shell**: VS Code extension interface and user interaction
- **WebAssembly Bridge**: High-performance bindings between JS and Rust
- **Candle Inference Engine**: Rust-based ML inference with GPU acceleration
- **MCP Server**: Model Context Protocol for VS Code integration
- **Yoshi Error Handling**: Comprehensive error management and recovery
- **Security Manager**: Input validation and sandboxed execution

## 🔒 Security Features

- **Input Validation**: Comprehensive validation of user inputs and prompts
- **Sandboxed Execution**: Isolated model execution with resource limits
- **Model Verification**: Cryptographic verification of model integrity
- **Resource Management**: Memory and time limits to prevent system impact
- **Path Sanitization**: Protection against directory traversal attacks

## 📊 Performance

- **WebAssembly**: Near-native performance for inference operations
- **GPU Acceleration**: CUDA, Metal, and MKL support for faster inference
- **Intelligent Caching**: LRU cache reduces repeated inference calls
- **Streaming**: Real-time token generation with progress updates
- **Memory Optimization**: Efficient memory usage with configurable limits

## 🧪 Supported Models

### Recommended Models

| Model | Size | Description |
|-------|------|-------------|
| Qwen2.5 7B Instruct Q4_0 | 4.2 GB | High-quality instruction following |
| Llama 3.2 3B Instruct Q4_0 | 1.9 GB | Fast, smaller model for quick responses |
| Mistral 7B Instruct Q4_0 | 4.1 GB | Excellent code generation capabilities |

### Model Format Support

- **GGUF**: Primary format with quantization support
- **Quantization Levels**: Q4_0, Q5_1, Q8_0 for different size/quality tradeoffs
- **Architectures**: Llama, Qwen, Mistral, and other transformer models

## 🛠️ Development

### Building from Source

```bash
# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Clone and build
git clone https://github.com/arcmoonstudios/yoshi
cd yoshi/yoshi-mcp
npm install
./scripts/build-wasm.sh
npm run compile
```

### Running Tests

```bash
# Rust tests
cargo test

# TypeScript tests
npm test

# WASM tests
wasm-pack test --node
```

### Project Structure

```
yoshi-mcp/
├── src/
│   ├── lib.rs              # Main Rust library
│   ├── error.rs            # Yoshi error handling
│   ├── inference.rs        # Candle inference engine
│   ├── model.rs            # Model management
│   ├── mcp.rs              # MCP server implementation
│   ├── security.rs         # Security and validation
│   ├── wasm.rs             # WebAssembly bindings
│   ├── extension.ts        # Main VS Code extension
│   ├── config/             # Configuration management
│   ├── model/              # Model management (TS)
│   ├── security/           # Security management (TS)
│   └── mcp/                # MCP server (TS)
├── scripts/                # Build scripts
├── docs/                   # Documentation
└── dist/                   # Built artifacts
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT OR Apache-2.0 license.

## 🙏 Acknowledgments

- [Candle](https://github.com/huggingface/candle) - Rust ML framework
- [Yoshi Framework](../yoshi/) - Error handling and auto-correction
- [VS Code Extension API](https://code.visualstudio.com/api) - Extension development
- [WebAssembly](https://webassembly.org/) - High-performance web execution

## 📞 Support

- 📧 Email: LordXyn@proton.me
- 🐛 Issues: [GitHub Issues](https://github.com/arcmoonstudios/yoshi/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/arcmoonstudios/yoshi/discussions)

---

**Built with ❤️ by ArcMoon Studios**
