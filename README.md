🧩 **Level 0 – Absolute Rust Beginner (Language Foundations)**

> Goal: Get comfortable with the Rust syntax, memory model (ownership/borrowing), basic tooling (cargo), and standard library.
> 

---

### 📚 **Topic: Variables, Functions, and Control Flow**

1. **Basic Calculator**
    - Take two numbers and an operator (+, -, *, /) from user input.
    - Perform the operation and print the result.
    - Practice `match`, functions, and error handling.
2. **FizzBuzz with a Twist**
    - Classic FizzBuzz from 1 to 100.
    - Add a twist: Ask user for custom divisors and labels.

  **3.   Temperature Converter**

- Convert Celsius ↔ Fahrenheit.
- Ask user input in terminal and print the converted value.
1.  **Number Guessing Game**
    - Generate a random number.
    - Take user input and give feedback: "Too high", "Too low", or "Correct".
    - Practice `loop`, `match`, and `Result`.
2.  **Tip Calculator**
    - Ask for bill amount and tip %.
    - Print total payable.
    - Format output to 2 decimal places using `format!`.

---

### 📚 **Topic: Ownership, Borrowing, and References**

1. **String Analyzer**
    - Take a sentence from the user.
    - Print total words, longest word, and the word frequency map.
    - Practice references and borrowing — e.g., pass `&str` around.
2. **Manual Memory Tracker**
    - Create a struct to simulate memory blocks.
    - Practice `Drop` trait, and show when Rust frees memory.
3.  **Palindrome Checker**
    - Function takes a `String`, returns `bool`.
    - Practice moving and borrowing ownership.
4. **Anagram Detector**
    - Function takes two `&str` and checks if they are anagrams.
    - Use sorting and comparison.

  **5.   Simple Encryption**

- Shift each letter by 3 positions (Caesar cipher).
- Practice character manipulation and cloning strings.

---

### 📚 **Topic: Structs, Enums, and Pattern Matching**

1. **Banking System**
    - Create a `BankAccount` struct with deposit, withdraw, and balance methods.
    - Use `enum` for transaction type and `match` to process it.
2. **Shape Area Calculator**
    - Use enums like `Shape::Circle`, `Shape::Rectangle`, etc.
    - Implement trait `Area` with a method `area()` for each shape.
3. **Dice Roller**
    - Randomly generate a number 1–6.
    - Match against number to show dice emoji 🎲.
4. **Basic Login Flow** 

          Store usernames & passwords in a `HashMap`.

          Validate login with proper `Result`.

**5.   Student Grade Tracker**

- Struct: `Student { name, marks: Vec<u32> }`
- Calculate average and grade (`A`, `B`, `C`).
- Use `enum Grade`.

 **6   Ticket Booking Simulator**

- Enum: `SeatType { Economy, Business, First }`
- Struct: `Ticket { name, seat: SeatType }`
- Print ticket with pricing based on type.

 **7     Dice Roller**

- Randomly generate a number 1–6.
- Match against number to show dice emoji 🎲.
1.  **Basic Login Flow**

          Store usernames & passwords in a `HashMap`.

         Validate login with proper `Result`.

---

### 📚 **Topic: Collections (Vec, HashMap, Option, Result)**

1. **Mini Shopping Cart**
    - Store items in a `Vec` or `HashMap<String, u32>`.
    - Let user add/remove/list items via terminal.
    - Practice `Option` for safe retrieval.
2. **Phonebook**
    - Map names to numbers using `HashMap`.
    - Allow search, insert, update, delete.
    - Use `Result` to handle user errors cleanly.

---

### 📚 **Topic: Collections & Loops**

1.  **Frequency Counter**

           Count how often each character appears in a string.

          Use `HashMap<char, u32>`.

1.  **Running Average Calculator**

          Keep taking numbers until user enters `exit`.

          Print average after every input.

1.   **Prime Number Generator**

          Ask for a range (e.g., 1 to 100) and print all prime numbers.

          Optimize with Sieve of Eratosthenes.

---

### 📚 **Topic: Basic File I/O**

1. **Notes Saver**

          Ask the user to enter notes line by line.

          Save them into a `.txt` file using `std::fs`.

1.   **Config Reader**

            Load a simple config file (`key=value`) and parse it into a `HashMap`.

---

### 📚 **Topic: Modules and Project Structure**

1. **Math Toolkit as a Library**
    - Create a custom library crate with modules:
        - `arithmetic`, `geometry`, `statistics`.
    - Write unit tests using `#[cfg(test)]`.
2. **CLI Flashcards App**
- User creates decks, adds Q&A, then quizzes themselves.
- Learn how to break code into reusable modules (`mod`, `use`).

## ⚙️ **Level 1 – Core Rust (Structs, Traits, File I/O, Serde)**

> Goal: Learn how to build reusable abstractions and work with external crates (serde, fs, json, etc.)
> 

---

### 📚 **Topic: Structs, Traits, and File I/O**

1. **CLI To-Do App**
    - Create a `Task` struct.
    - Store tasks in a vector and save/load to a JSON file.
    - Commands: `add`, `remove`, `list`, `done`.
    - Use file handling (`std::fs`) + JSON (`serde`).
2. **User Authentication Mock**
    - Build a CLI tool where users register/login.
    - Store user data (hashed password) in a file.
    - Practice struct design and traits for reusable logic.

---

### 📚 **Topic: Serde + JSON/CSV Parsing**

1. **CSV to JSON Converter**
    - Read CSV file from disk and convert it into JSON format.
    - Use `serde` and `csv` crate.
    - Bonus: Validate CSV structure using custom types.
2. **Contact Book with Export Options**
    - CRUD for contacts (name, phone, email).
    - Export to both CSV and JSON.
    - Use enums to choose export format.

---

### 📚 **Topic: Traits & Error Handling**

1. **Currency Converter**
    - Structs: `Currency`, `ExchangeRate`.
    - Trait: `Convert` with `convert_to(&self, target: &Currency)`.
    - Return `Result<f64, ConversionError>`.
2. **Unit Conversion Toolkit**
    - Length, weight, temperature conversion using traits.
    - `impl Trait` pattern.
    - Build a generic CLI around trait implementations.

---

## 🚦 **Level 2 – Concurrency & Async Basics (Tokio, Threads, Channels)**

> Goal: Understand multithreading, async/await, and communication between threads.
> 

---

### 📚 **Topic: Threads & Channels**

1. **Multithreaded File Hasher**
    - Spawn threads to hash multiple files in parallel.
    - Use `std::thread` + `mpsc::channel` to return results.
    - Bonus: Add progress bar using `indicatif`.
2. **Parallel Sorting**
    - Divide a vector into chunks and sort each in a separate thread.
    - Merge them at the end.
    - Practice `Arc`, `Mutex`, and channel coordination.

---

### 📚 **Topic: Async/Await and Futures**

1. **Async HTTP Pinger**
    - Ping multiple websites using `reqwest` + `tokio`.
    - Print status codes and response times.
    - Use `futures::join_all` to await all.
2. **Download Manager**
    - Accept multiple URLs.
    - Use `tokio::fs` and `reqwest` to download concurrently.
    - Bonus: Show download speed.

---

### 📚 **Topic: Time, Intervals, and Scheduling**

1. **Task Scheduler**
    - Repeatedly run tasks at intervals using `tokio::time::interval`.
    - Schedule: log printer, API poller, file cleaner.
    - Load config from `TOML`.

---

## 🌐 **Level 3 – HTTP Servers & Web Skills (Actix, Axum, Sockets)**

> Goal: Build real APIs, learn low-level HTTP servers, and work with DB + web libraries.
> 

---

### 📚 **Topic: HTTP Server (From Scratch)**

1. **Mini HTTP Server**
    - Use `TcpListener`, `TcpStream`.
    - Handle only `GET /` and `POST /data`.
    - Return simple HTML or echo posted JSON.
2. **Serve Static Files**
    - Extend the above server to read and serve static `.html`, `.css`, `.js` files.
    - Learn how MIME types work.

---

### 📚 **Topic: REST API with Axum or Actix**

1. **CRUD Blog API**
    - Endpoints: `/posts`, `/posts/:id`
    - Use `Axum`, `serde`, `sqlx`, `PostgreSQL`.
    - Async functions for handlers.
2. **JWT-based Auth System**
    - Register/Login endpoints.
    - Issue and validate JWT tokens.
    - Use middleware to protect routes.

---

### 📚 **Topic: WebSocket Communication**

1. **Real-Time Chat Server**
    - Use `tokio-tungstenite` or `warp` for WebSocket handling.
    - Broadcast messages to all connected clients.
    - Handle connect/disconnect gracefully.

---

### 📚 **Topic: Middleware, Error Handling, and Logging**

1. **Rate-Limited API**
    - Add middleware for logging, rate-limiting.
    - Use `tower` or `actix-web-middleware`.
    - Return proper JSON error messages on limits.

## 🧠 **Level 4 – Advanced Rust Concepts (Lifetimes, Macros, Traits, Async Mastery)**

> Goal: Master advanced topics like custom macros, trait systems, lifetimes, and async architecture.
> 

---

### 📚 **Topic: Lifetimes & Advanced Borrowing**

1. **Reference Tracker**
    - Track multiple string references with lifetimes.
    - Create a system where you return a reference to the longest string.
2. **Custom Cache**
    - Use a generic cache with a reference-based API.
    - Practice lifetime annotations in methods and structs.

---

### 📚 **Topic: Traits, Generics & Associated Types**

1. **Plugin System**
    - Create a `Plugin` trait with `run(&self, input: String) -> String`.
    - Implement multiple plugins that transform input.
    - Use trait objects (`Box<dyn Plugin>`) and generics.
2. **Custom Iterator System**
    - Create a custom iterable struct (e.g. Fibonacci, SlidingWindow).
    - Implement `Iterator`, `IntoIterator` with generics and trait bounds.

---

### 📚 **Topic: Procedural Macros & Derives**

1. **Mini Validator Derive**
    - Write a `#[derive(Validate)]` macro.
    - Add `#[validate(length(min = 5))]`style attributes.
    - Use `proc_macro` crate.
2. **Auto Logger Macro**
    - Create a macro that logs entry and exit of functions.
    - Optional: support logging levels (`info`, `warn`, etc.).

---

### 📚 **Topic: Advanced Async Design**

1. **Concurrent Task Orchestrator**
    - Launch multiple async tasks and monitor their state.
    - Cancel long-running tasks after timeout using `tokio::select!`.
2. **Custom Runtime Builder**
    - Build a runtime setup similar to `tokio::main`, but manually:
        - `Runtime::new().block_on(main_fut())`
    - Understand how executors work.

---

## 🚀 **Level 5 – Real-World Rust Projects (End-to-End Apps)**

> Goal: Build full-stack systems or tools that could pass as real portfolio projects.
> 

---

### 📚 **Project: Real-Time Chat App**

1. **Backend: WebSocket Server**
    - Use `tokio`, `tungstenite`, `warp` or `axum`.
    - Handle rooms, user sessions, and message routing.
2. **Database & Auth**
    - Use `sqlx` + Postgres to store users and messages.
    - Add user registration, JWT-based login.
3. **Frontend (Optional)**
    - Simple HTML/JS WebSocket client.
    - Or build with React + WebSocket if you know JS.

---

### 📚 **Project: GitHub Clone (Repo + Commit Viewer)**

1. **Features**
    - Users can create "repos", upload files, commit changes.
    - Store versioned metadata in DB.
2. **Tech Stack**
    - `axum` or `actix-web`
    - `sqlx` + PostgreSQL
    - `tower-http` for middleware
3. **Bonus**
    - S3-compatible object storage (e.g., MinIO)
    - Token-based repo sharing

---

### 📚 **Project: Distributed Key-Value Store**

1. **Core Engine**
    - Store key-value pairs on disk.
    - Log-structured or LSM-tree style (like Bitcask).
2. **Networking**
    - Handle basic TCP requests from clients.
    - Encode/decode messages using `bincode` or `serde`.
3. **Cluster**
    - Add replication between nodes.
    - Use heartbeats, leader election (Raft optional).

---

### 📚 **Project: Rust-Based CLI Dev Tools**

1. **TOML Config Inspector**
    - Load and validate `.toml` configs for Rust projects.
    - Flag missing fields, types, etc.
2. **Task Runner (Make Alternative)**
    - Define tasks in YAML or TOML.
    - Run them like `cargo xtask build`, `deploy`, etc.

---

### 📚 **Project: AI/ML Pipeline in Rust**

(Stretch goal for high-performers)

1. **Dataset Processor**
    - Stream large `.csv` or `.jsonl` datasets using iterators and async I/O.
2. **Model Server**
    - Host ONNX models with a simple REST endpoint.
    - Use `tract`, `tch-rs`, or `burn`.

---

This roadmap ensures:

- **Breadth** of topics (system, web, CLI, tooling, async).
- **Depth** with industry-level patterns.
- **Portfolio-ready** projects for showcasing