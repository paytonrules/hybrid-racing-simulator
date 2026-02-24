# Implementation Plan: WebGPU + egui Hello World

## Goal
Replace the DOM-based "Hello World" with a high-performance WebGPU rendering pipeline and an interactive `egui` interface.

## 1. Dependency Updates (`Cargo.toml`)
Add the following crates:
- `wgpu`: Core WebGPU API.
- `winit`: Windowing and event handling (targets `<canvas>`).
- `egui`, `egui-wgpu`, `egui-winit`: Immediate-mode GUI.
- `wasm-bindgen-futures`: Async support for GPU initialization.
- `log`, `console_log`: Logging for Wasm.

## 2. UI Preparation (`index.html`)
- Add `<canvas id="canvas"></canvas>` to the body.
- Style the canvas to fill the window.

## 3. Core Logic (`src/main.rs`)
- **State Management:** Create a `State` struct to hold `Instance`, `Adapter`, `Device`, `Queue`, `Surface`, and `egui` renderers.
- **Async Initialization:** Use `wasm_bindgen_futures::spawn_local` to initialize the GPU.
- **Event Loop:**
    - Handle window resizing.
    - Pass input events to `egui`.
    - **Render Pass:** Clear the screen and render `egui` primitives.

## 4. Verification
- Run `trunk serve`.
- Confirm a "Hello World" `egui` window appears over a colored background.
