# Port Scanner (Rust)

⚠️ Disclaimer
This tool is for educational and legal testing purposes only. Do not use it to scan networks or systems you don’t own or have permission to test.

This is a basic asynchronous port scanner written in **Rust** using [`tokio`](https://crates.io/crates/tokio). This is my first self-project in Rust to better learn the language and was my second attempt at creating a port scanner. This is the 2nd version with added timers and timeouts for no response ports.

It scans a range of ports (default: 1 to 1024) on a specified target and displays which ports are open.

---

## New Features

- Scans TCP ports 1 through 1023 (default range)
- Uses async I/O for efficiency
- Displays open ports
- Displays estimated time remaining
- Ignores closed ports for clean output
- Times out if no response is received within 3 seconds per port

---

## Technologies Used

- Rust – safe and fast systems-level language
- Cargo – Rust’s package manager and build system
- Tokio – asynchronous runtime for handling concurrent tasks
- added **std::net** and **std::time**: Built-in Rust standard libraries
- Command Line Interface (CLI) – interaction through terminal
- TCP Networking – scans network ports for open connections

---


## Dependencies

- [`tokio`](https://crates.io/crates/tokio)

Add to `Cargo.toml`:

```toml
tokio = { version = "1.46.1", features = ["full"] }

## Project Structure

```
port-scanner/
├── src/
│   └── main.rs         # Main source file for the port scanner
├── Cargo.toml          # Rust package configuration file
├── Cargo.lock          # Lock file for reproducible builds
└── README.md           # Project documentation /w updates
```


---

## How to Run

1. Clone the repository

   ```
   git clone https://github.com/kyle-allen2006/port_scanner.git
   cd port_scanner

2. Build and run the scanner

    cargo run


About the Developer:

Hello and welcome, I’m Kyle Allen I am a passionate developer transitioning into software engineering after 16 years of experience in robotics and automation. I’m currently pursuing my Software Engineering degree at WGU while building real-world projects to strengthen my skills in modern web development.

Background in robotics, controls, industrial automation, and maintenance.

Current student at Western Governors University (WGU).

Strong foundation in programming logic and problem solving.

Actively building a full-stack portfolio using Node.js, Angular, MongoDB, and now Rust.

This project is part of my personal growth journey as I transition into full-time software development. I'm eager to bring my technical experience and dedication to a forward-thinking development team.