# Leptos Pomodoro Timer 🍅

A beautiful and functional pomodoro timer built with Rust and Leptos, compiled to WebAssembly for client-side rendering.

## Features

- ⏱️ **Complete Pomodoro Technique**: 25-minute work sessions, 5-minute short breaks, and 15-minute long breaks
- 🔄 **Automatic Cycling**: Automatically switches between work and break periods
- 📊 **Session Tracking**: Keeps track of completed work sessions
- 🎨 **Modern UI**: Beautiful, responsive design with smooth animations
- 🔔 **Browser Notifications**: Alerts when sessions complete
- 📱 **Responsive**: Works perfectly on desktop and mobile devices
- 🚀 **Fast Loading**: Compiled to WebAssembly for optimal performance

## Demo

Visit the live demo: [https://cosmoswafer.github.io/leptos-pomodoro/](https://cosmoswafer.github.io/leptos-pomodoro/)

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/): `cargo install wasm-bindgen-cli`

### Building

1. Clone the repository:
```bash
git clone https://github.com/cosmoswafer/leptos-pomodoro.git
cd leptos-pomodoro
```

2. Build the project:
```bash
./build.sh
```

3. Serve locally:
```bash
cd dist
python3 -m http.server 8080
```

4. Open your browser and visit `http://localhost:8080`

### Manual Build Steps

If you prefer to build manually:

```bash
# Build the WASM target
cargo build --target wasm32-unknown-unknown --release

# Generate WASM bindings
wasm-bindgen --out-dir pkg --target web --no-typescript target/wasm32-unknown-unknown/release/leptos_pomodoro.wasm

# Prepare distribution files
mkdir -p dist
cp index.html dist/
cp -r pkg dist/
```

## Technology Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Leptos](https://leptos.dev/)** - Modern web framework for Rust
- **[WebAssembly](https://webassembly.org/)** - Binary instruction format for web
- **[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)** - Rust and WebAssembly integration

## Deployment

The project includes a GitHub Actions workflow that automatically:

1. Builds the Rust/WASM application
2. Generates the necessary bindings
3. Deploys to GitHub Pages

The workflow is triggered on every push to the main branch.

## Project Structure

```
leptos-pomodoro/
├── src/
│   ├── lib.rs                    # Main application entry point
│   └── components/
│       ├── mod.rs               # Components module
│       └── pomodoro.rs          # Pomodoro timer component
├── .github/
│   └── workflows/
│       └── deploy.yml           # GitHub Actions workflow
├── index.html                   # HTML template
├── Cargo.toml                   # Rust dependencies
├── build.sh                     # Build script
└── README.md                    # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
