**ArcDB** is a lightweight, multi-threaded key-value store written in Rust. It is designed as a deep dive into systems programming, focusing on building a Redis-compatible server from the ground up.

## 🎯 Project Goals
Unlike a typical tutorial clone, ArcDB aims to strictly adhere to low-level engineering principles:
- **Zero-Abstraction Networking:** Handling raw TCP streams and byte buffers.
- **Protocol Implementation:** A custom parser for the Redis Serialization Protocol (RESP).
- **Concurrency Models:** Exploring thread safety, atomic reference counting (`Arc`), and interior mutability without relying on high-level framework magic.

## 🛠 Status
Currently under active development. The goal is to reach feature parity with basic Redis commands (`SET`, `GET`, `PING`, `ECHO`) while maintaining strict memory safety.
