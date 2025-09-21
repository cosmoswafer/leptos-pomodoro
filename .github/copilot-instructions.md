# Leptos Pomodoro Timer - GitHub Copilot Instructions

A modern pomodoro timer built with Rust, Leptos framework, and WebAssembly. Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Setup
- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Add WASM target: `rustup target add wasm32-unknown-unknown`
- Install wasm-bindgen-cli: `cargo install wasm-bindgen-cli --version 0.2.103`
- Install Trunk (optional, faster builds): `cargo install trunk`

### Building the Project
**Use the build script method (recommended and fully tested):**

- `./build.sh` -- takes 75-80 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- **CRITICAL**: Build may appear to hang during WASM compilation - this is NORMAL. Wait for completion.
- Output goes to `dist/` directory with correct file structure

**Alternative: Trunk build system (advanced users):**
- `cargo install trunk` -- takes 7-8 minutes. NEVER CANCEL. Set timeout to 15+ minutes.
- `trunk build --release` -- takes 2-4 seconds but may have path issues for local development
- Use for CI/CD or if you understand WASM module path configuration

### Running the Application
**Recommended method (works reliably):**
1. Build using `./build.sh`
2. Serve: `cd dist && python3 -m http.server 8080`
3. Open browser to `http://localhost:8080`
4. **VERIFY**: Console should show "Starting Leptos Pomodoro Timer app"

### Testing and Validation
- Run tests: `cargo test` -- currently 0 tests, completes in seconds
- Check code: `cargo check` -- takes 40-50 seconds for initial run. NEVER CANCEL. Set timeout to 90+ seconds.
- Format code: `cargo fmt` -- run this before committing
- Lint code: `cargo clippy` -- check for warnings and issues

## Manual Validation Requirements

**ALWAYS run through complete user scenarios after making changes:**

### Required Testing Scenarios
1. **Timer Functionality**:
   - Start timer, verify it counts down from 25:00
   - Pause/resume timer, verify state changes correctly
   - Reset timer, verify it returns to initial state

2. **Mode Switching**:
   - Switch to "Short Break (5m)", verify timer shows 5:00
   - Switch to "Long Break (15m)", verify timer shows 15:00
   - Switch back to "Work (25m)", verify timer shows 25:00
   - Verify active mode highlighting works correctly

3. **Complete Workflow**:
   - Start work session, let timer run for at least 10 seconds
   - Test pause and resume functionality
   - Switch modes while timer is running
   - Verify session counter updates (currently shows "Work Sessions Completed: 0")

### Browser Testing
- Test in at least one modern browser (Chrome/Firefox/Safari)
- Verify JavaScript console shows: "Starting Leptos Pomodoro Timer app"
- Check for any console errors or warnings
- Verify responsive design and UI interactions

## Critical Timing Information

**NEVER CANCEL these long-running operations:**
- Initial `cargo check`: 40-50 seconds - Set timeout to 90+ seconds
- `./build.sh` script: 75-80 seconds - Set timeout to 120+ seconds  
- `cargo install wasm-bindgen-cli`: 5-7 minutes - Set timeout to 10+ minutes
- `cargo install trunk`: 7-8 minutes - Set timeout to 15+ minutes

**Fast operations (< 10 seconds):**
- `cargo test`: < 1 second
- `cargo clippy`: < 1 second (after initial check)
- `cargo fmt`: < 1 second
- `trunk build --release`: 2-4 seconds (but may have serving issues locally)

## Common Tasks and Commands

### Development Workflow
1. Make code changes in `src/`
2. Format: `cargo fmt`
3. Check: `cargo clippy`  
4. Build: `./build.sh` (NEVER CANCEL - takes 75-80 seconds)
5. Test manually: `cd dist && python3 -m http.server 8080`
6. Validate complete user scenarios (see above)
7. Access application at `http://localhost:8080`

### CI/CD Pipeline
The project includes `.github/workflows/deploy.yml` that:
- Builds on Ubuntu with Rust stable
- Uses same build commands as local development
- Deploys to GitHub Pages on main branch pushes
- **Always run `cargo fmt` before committing** or CI will fail due to formatting

### Repository Structure Reference
```
leptos-pomodoro/
├── src/
│   ├── lib.rs                    # Main application entry with App component
│   └── components/
│       ├── mod.rs               # Components module export
│       └── pomodoro.rs          # Main PomodoroTimer component with timer logic
├── .github/
│   └── workflows/
│       └── deploy.yml           # GitHub Actions CI/CD pipeline
├── dist/                        # Build output (generated, in .gitignore)
├── pkg/                         # WASM bindings (generated, in .gitignore)
├── target/                      # Cargo build cache (generated, in .gitignore)
├── index.html                   # HTML template with embedded CSS
├── Cargo.toml                   # Rust dependencies and project config
├── Trunk.toml                   # Trunk build tool configuration
├── build.sh                     # Manual build script
└── README.md                    # Project documentation
```

### Key Files to Understand
- `src/lib.rs`: Application root with routing and main App component
- `src/components/pomodoro.rs`: Timer logic, state management, and UI components
- `index.html`: Contains all CSS styling and WASM module loading script
- `Cargo.toml`: Leptos 0.6 dependencies, WASM-specific configuration
- `.github/workflows/deploy.yml`: Automated build and deployment to GitHub Pages

### Known Issues and Warnings
- Two unused imports in `src/lib.rs` lines 5 and 7 - these generate warnings but don't break builds
- No unit tests currently exist - the project relies on manual validation
- Build output is always in `dist/` directory regardless of build method
- Python 3 HTTP server is required for local testing (no Node.js/npm involved)

### Technology Stack
- **Rust**: Systems programming language with WASM target support
- **Leptos 0.6**: Modern Rust web framework with reactive components
- **WebAssembly**: Client-side execution, no server required  
- **wasm-bindgen**: Rust/JavaScript interop layer
- **gloo-timers**: Timer functionality for WASM environment
- **HTML/CSS**: Embedded styling, no external CSS frameworks

## Troubleshooting
- If build fails, ensure all prerequisites are installed correctly
- If WASM module fails to load, check browser console for specific errors  
- If timer doesn't work, verify JavaScript is enabled and no console errors exist
- For slow builds, this is normal - refer to timing information above
- Always use absolute paths when working with files: `/home/runner/work/leptos-pomodoro/leptos-pomodoro/`