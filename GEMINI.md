# Gemini Context: hybrid-racing-simulator

This document provides foundational mandates and context for Gemini CLI when working on the `hybrid-racing-simulator` project.

## Project Overview
A hybrid racing simulator built with Rust and WebAssembly, targeting web platforms.

## Tech Stack
- **Language:** Rust (2024 Edition)
- **Target:** WebAssembly (Wasm)
- **Key Libraries:** `wasm-bindgen`, `web-sys`, `console_error_panic_hook`
- **Frontend:** HTML/JS integration via `index.html`
- **Build/Serve Tool:** [Trunk](https://trunkrs.dev/) (`trunk-rs`)

## Development Workflow
- **Serve Locally:** `trunk serve`
- **Build for Production:** `trunk build --release`
- **Testing:** `cargo test`
- **Linting:** `cargo clippy`
- **Formatting:** `cargo fmt`

## Coding Standards
- Follow idiomatic Rust patterns.
- Ensure all Wasm-exposed functions are properly documented with `#[wasm_bindgen]`.
- Maintain safety and minimize `unsafe` blocks.
- Use `console_error_panic_hook` for better debugging in the browser console.

## Project Structure
- `src/main.rs`: Entry point for the Rust logic.
- `index.html`: Main web entry point (managed by Trunk).
- `Cargo.toml`: Project configuration and dependencies.
