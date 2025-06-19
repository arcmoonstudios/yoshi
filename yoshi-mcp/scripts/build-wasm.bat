@echo off
REM Build script for WASM components of Yoshi MCP (Windows)

echo Building Yoshi MCP WebAssembly components...

REM Ensure we have the WASM target
rustup target add wasm32-unknown-unknown

REM Build with optimizations for WASM
echo Building WASM module...
set RUSTFLAGS=-C target-feature=+simd128
wasm-pack build . --target web --out-dir dist/wasm --release --scope arcmoon-studios

REM Optimize WASM binary if wasm-opt is available
where wasm-opt >nul 2>nul
if %ERRORLEVEL% == 0 (
    echo Optimizing WASM binary...
    wasm-opt --enable-simd --enable-bulk-memory -O3 dist/wasm/yoshi_mcp_bg.wasm -o dist/wasm/yoshi_mcp_bg.wasm
) else (
    echo wasm-opt not found, skipping optimization
)

REM Generate TypeScript definitions for bundler target
echo Generating TypeScript definitions...
wasm-pack build . --target bundler --out-dir dist/wasm-ts --release --scope arcmoon-studios

echo WASM build complete!
echo Web target: dist/wasm/
echo TypeScript target: dist/wasm-ts/
