# Rust Learning Playground 🦀

This repository is a **personal Rust learning playground** containing small, focused examples and mini-projects that cover **core Rust concepts**, standard library usage, and concurrency primitives.

The goal of this repo is **hands-on mastery of Rust fundamentals** by writing code from scratch rather than relying on tutorials alone.

---

## 📂 Repository Structure

Each folder focuses on **one Rust concept** or feature:

### 🧠 Core Rust Concepts
- `ownership` – Ownership rules and memory safety
- `borrow` – Borrowing & references
- `lifetimes` – Struct and function lifetimes
- `drop` – Manual drop and destructor behavior
- `patterns` – Pattern matching (`@` binding, etc.)

### 🧱 Data Structures
- `linkedlist` – Linked list implementations
- `queue` – Queue from scratch
- `lists` – List variants
- `vectors` – Insert & remove operations
- `hashmaps` – HashMap usage with input handling

### 🧩 Language Features
- `enums` – Enum usage (traffic light example)
- `structs` – Struct definitions and methods
- `generics` – Generic functions and methods
- `closures` – Closures and capture behavior
- `iterators` – Iterator adapters (filter, etc.)
- `strings` – Deep dive into Rust strings

### ⚙️ Concurrency & Async
- `thread` – Multi-threading & channels
- `mutex` – `Arc<Mutex<T>>` usage
- `rc` – Reference counting (`Rc`)
- `refcell` – Interior mutability
- `hello-async` – Async basics
- `streams` – Streams from *The Rust Programming Language*

### 🧪 Testing
- `adder` – Integration tests

### 📦 Mini Projects
- `project_contact_list` – Contact list CLI project
- `restaurant` – Module system & `use` keyword

### ❗ Error Handling
- `error_handling` – `Result`, `Option`, and custom errors

---

## 🎯 Purpose of This Repository

- Learn Rust **the Rust way** (ownership-first thinking)
- Practice **standard library internals**
- Understand **low-level behavior** (memory, concurrency)
- Build confidence for **backend, systems, and blockchain development**

This repo is intentionally **not tutorial-style**, but rather **experiment-driven**.

---

## 🚀 How to Run Examples

```bash
cd <folder-name>
cargo run
