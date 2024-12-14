// I received some assistance from a language model to write this document.

# WASM Snake Game

A classic Snake game implemented in Rust and WebAssembly. This guide will help you set up and run the game from scratch.

## Prerequisites Installation

### 1. Install Rust
First, you need to install Rust on your computer:

**For Windows:**
1. Download the Rust installer from https://rustup.rs/
2. Run the downloaded file and follow the installation prompts
3. Restart your computer after installation

**For Mac/Linux:**
1. Open Terminal
2. Copy and paste this command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the prompts and restart your terminal

### 2. Install wasm-pack
After installing Rust, install wasm-pack:

1. Open Command Prompt as Administrator
2. Run:

```bash
cargo install wasm-pack
```


### 3. Install Node.js
1. Download Node.js from https://nodejs.org/
2. Choose the LTS (Long Term Support) version
3. Run the installer and follow the prompts
4. Restart your computer

## Setting Up The Project

1. Clone this repository:

```bash
git clone https://github.com/muhammadzeejah/canvas-snake-game-using-std.git
```
2. Navigate to the project directory:

```bash
cd canvas-snake-game-using-std
```

3. Build the WebAssembly package:

```bash
wasm-pack build --target web
```

## Running The Game

1. Install a simple web server (you only need to do this once):
For Windows:

```bash
npm install -g http-server
```

For Mac/Linux:

```bash
sudo npm install -g http-server
```

2. Start the server:

```bash
http-server
```

3. Open your browser and go to 

```bash
http://127.0.0.1:{port}/
```

## Troubleshooting

### Common Issues and Solutions:

1. **"Command not found" errors:**
   - Make sure you've restarted your terminal/computer after installations
   - Check if the PATH environment variables are set correctly

2. **Build fails:**
   - Run `rustup update` to ensure you have the latest Rust version
   - Try deleting the `target` and `pkg` folders and rebuild

3. **Game doesn't load:**
   - Check if you're using a modern browser (Chrome, Firefox, Edge)
   - Make sure you built the project with `wasm-pack build --target web`
   - Check browser console for errors (Press F12 to open developer tools)