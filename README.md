# ⚙️ Physics Engine

A simple 2D physics engine built in Rust using Macroquad.

Created as a learning project to understand how real physics engines work under the hood.

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Macroquad](https://img.shields.io/badge/Framework-Macroquad-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![crates.io](https://crates.io/crates/ball_engine)

---

## ✨ Features

- 🌍 Gravity simulation
- ⚡ Velocity & acceleration
- 🟢 Circle rigid bodies
- 💥 Ball-to-ball collision detection
- 🧱 Wall & floor collisions
- 🏀 Restitution (bounciness)
- 🛞 Friction
- 😴 Sleeping bodies
- 🎨 Random colors
- ⚖️ Random mass support
- 📏 Random radius support
- 🖱️ Click-to-spawn balls

---

## 📸 Preview

Add a screenshot or GIF here.

Example:

![Preview](images/demo.gif)

---

## 🎮 Controls

| Key | Action |
|------|---------|
| Left Click | Spawn Ball |
| ESC | Exit |

---

## 🧠 What I Learned

This project helped me learn:

- Rust ownership & borrowing
- Structs and modules
- Physics simulation
- Collision detection
- Collision resolution
- Game loops
- Engine architecture
- Performance optimization

---

## 📂 Project Structure

```text
src/
├── main.rs
└── engine/
    ├── body.rs
    ├── collision.rs
    ├── physics.rs
    ├── world.rs
    └── mod.rs
```

---

## 🚀 Installation

### Clone

```bash
git clone https://github.com/parth2152012/parth-engine.git
cd parth-engine
```

### Run

```bash
cargo run --release
```

---

## 🔨 Built With

- Rust
- Macroquad

---

## 🗺️ Roadmap

Future ideas:

- [ ] Rectangle bodies
- [ ] Springs
- [ ] Rope constraints
- [ ] Spatial hashing
- [ ] Pool game
- [ ] Carrom game
- [ ] WebAssembly build

---

## 🤝 Contributing

Pull requests are welcome.

Feel free to open an issue if you find a bug or have an idea.

---

## 📜 License

MIT License

---

## 👨‍💻 Author

Built by Parth as a Rust learning project.

GitHub: https://github.com/parth2152012
