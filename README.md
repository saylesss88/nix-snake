# nix-snake ❄️🐍

A lightweight, terminal-based Snake game and screensaver written in Rust.

![snowflake-bounce demo](https://raw.githubusercontent.com/saylesss88/nix-snake/main/demo.gif)

Watch a NixOS Lambda (λ) navigate your terminal, consuming snowflakes and
packages in an infinite loop, or take control and play yourself.

**Status: Active Development** 🚧

---

## ✨ Features

- 🖥️ Screensaver Mode (Autopilot): The snake plays itself using a greedy
  pathfinding algorithm. Perfect for a terminal background.

- 🎮 Seamless Override: Press any arrow key to instantly switch from
  "Screensaver" to "Manual" mode. Press a to switch back.

- ❄️ NixOS Themed: The snake head is a Lambda (λ), eating snowflakes (❄) and
  packages (📦).

- 🚀 Performance: Built with pure crossterm for low-latency rendering and
  minimal resource usage.

- 🔄 Infinity Walls: The world wraps around the edges of your terminal.

---

## 📦 Installation From Source

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/saylesss88/nix-snake cd nix-snake
cargo install --path .
```

crates.io

```bash
cargo install nix-snake
```

---

## 🕹️ Controls

- `a`: Switch to Autopilot (Screensaver) Mode

- `q`/ `Esc`: Quit
