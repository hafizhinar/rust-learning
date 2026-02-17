# rust-learning
Rust learning journey from Go language

# 🦀 Rust Complete Curriculum

> **Goal:** Go → Rust | Backend, Payment System, Systems Programming  
> **Pace:** < 5 jam/minggu | **Estimasi:** ~9–10 Bulan  
> **Pendekatan:** Konsep → Analogi Go → Implementasi → Pitfalls → Latihan

---

## Introduction

Repo ini adalah workspace untuk belajar Rust secara terstruktur dari nol hingga production-ready. Setiap modul adalah crate tersendiri yang bisa di-run dan di-test secara independen.

### Struktur Repo

```
rust-learning/                    ← root repo
├── Cargo.toml                    ← workspace config
├── README.md                     ← curriculum checklist (file ini)
│
├── module-01-foundations/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
├── module-02-ownership/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
│
└── ... dst per modul
```

### Setup Awal

```bash
# 1. Clone repo
git clone https://github.com/username/rust-learning.git
cd rust-learning

# 2. Install Rust (jika belum)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Verifikasi
rustc --version
cargo --version
```

### Menambah Modul Baru

```bash
# Buat crate baru untuk modul
cargo new module-01-foundations

# Tambahkan ke root Cargo.toml
# [workspace]
# members = [
#     "module-01-foundations",
# ]
```

**Root `Cargo.toml`:**
```toml
[workspace]
members = [
    "module-01-foundations",
    "module-02-ownership",
    "module-03-type-system",
    # tambah setiap masuk modul baru
]
```

### Cara Run Per Modul

```bash
# Run modul tertentu dari root
cargo run -p module-01-foundations

# Test modul tertentu
cargo test -p module-01-foundations

# Run semua sekaligus
cargo build --workspace
```

### Workflow Belajar

```
Masuk modul baru
      ↓
cargo new module-XX-nama
      ↓
Tambahkan ke workspace Cargo.toml
      ↓
Tulis code latihan per topik
      ↓
cargo run -p module-XX-nama
      ↓
cargo test -p module-XX-nama
      ↓
git commit & push
```

### Commit Convention

```
feat(module-01): add variables and mutability examples
feat(module-01): add functions exercise
test(module-01): add unit test for currency converter
feat(module-02): add ownership examples
fix(module-02): fix borrow checker error in transaction log
```

---

## Progress Overview

| Modul | Topik | Status |
|-------|-------|--------|
| 01 | Language Foundations | ⬜ |
| 02 | Ownership, Borrowing & Lifetimes | ⬜ |
| 03 | Type System Lanjutan | ⬜ |
| 04 | Error Handling | ⬜ |
| 05 | Memory Management & Smart Pointers | ⬜ |
| 06 | Functional: Closures & Iterators | ⬜ |
| 07 | Concurrency & Async | ⬜ |
| 08 | Data Structures & Algorithms *(Ongoing)* | ⬜ |
| 09 | Code Organization & Project Structure | ⬜ |
| 10 | Design Patterns in Rust | ⬜ |
| 11 | Testing In Depth | ⬜ |
| 12 | Database Integration | ⬜ |
| 13 | Redis Integration | ⬜ |
| 14 | Kafka Integration | ⬜ |
| 15 | Third-Party Integration | ⬜ |
| 16 | REST API & gRPC | ⬜ |
| 17 | Security | ⬜ |
| 18 | Architecture & Production | ⬜ |

---

## Module 01 — Language Foundations
> *Estimasi: 2–3 minggu*

- [ ] **01.01 — Setup Environment**
  - [ ] Install Rust via rustup
  - [ ] Cargo (build, run, test)
  - [ ] rustfmt — code formatter
  - [ ] clippy — linter
  - [ ] rust-analyzer — IDE support

- [ ] **01.02 — Macros Dasar & Output**
  - [ ] Apa itu macro & kenapa ada tanda `!`
  - [ ] `println!`, `print!`, `eprintln!`
  - [ ] Format specifiers: `{}`, `{:?}`, `{:#?}`, `{:.2}`, `{:>10}`
  - [ ] `format!` — return String tanpa print
  - [ ] Macro sehari-hari: `vec!`, `panic!`, `assert!`
  - [ ] Bedanya macro vs function

- [ ] **01.03 — Variables & Mutability**
  - [ ] `let` vs `let mut`
  - [ ] Immutable by default (vs Go)
  - [ ] Shadowing
  - [ ] `const` & `static`

- [ ] **01.04 — Data Types**
  - [ ] Scalar: integer (`i8`–`i128`, `u8`–`u128`), float (`f32`, `f64`)
  - [ ] Boolean & char
  - [ ] Compound: tuple & array
  - [ ] Type inference
  - [ ] Type casting

- [ ] **01.05 — Functions**
  - [ ] `fn` keyword & parameter types
  - [ ] Return type & return value
  - [ ] Expression vs statement (kenapa tidak pakai `return`)
  - [ ] Nested functions

- [ ] **01.06 — Control Flow**
  - [ ] `if` / `else if` / `else` sebagai expression
  - [ ] `loop` & `break` dengan return value
  - [ ] `while`
  - [ ] `for` & range (`0..5` vs `0..=5`)
  - [ ] `match` dasar

- [ ] **01.07 — Structs**
  - [ ] Defining & instantiating structs
  - [ ] Field shorthand
  - [ ] Struct update syntax
  - [ ] Methods dengan `impl`
  - [ ] Associated functions (konstruktor)
  - [ ] vs Go struct + receiver

- [ ] **01.08 — Enums Dasar**
  - [ ] Defining enums
  - [ ] `match` dengan enum
  - [ ] vs Go iota

- [ ] **01.09 — Testing Basics**
  - [ ] `#[test]` & `#[cfg(test)]`
  - [ ] `assert!`, `assert_eq!`, `assert_ne!`
  - [ ] `#[should_panic]`
  - [ ] Doc tests
  - [ ] `cargo test` — run & filter

- [ ] **Mini Project:** CLI currency converter IDR ↔ USD/SGD lengkap dengan unit test

---

## Module 02 — Ownership, Borrowing & Lifetimes
> *Estimasi: 3–4 minggu*

- [ ] **02.01 — Konsep Ownership**
  - [ ] Apa itu ownership & kenapa ada
  - [ ] 3 aturan ownership
  - [ ] Move semantics
  - [ ] Stack vs Heap (pengantar)

- [ ] **02.02 — Clone vs Copy**
  - [ ] Copy trait — tipe yang di-copy otomatis
  - [ ] Clone trait — eksplisit deep copy
  - [ ] Kapan pakai mana

- [ ] **02.03 — References & Borrowing**
  - [ ] `&` — immutable reference
  - [ ] `&mut` — mutable reference
  - [ ] Aturan borrowing
  - [ ] Dangling references

- [ ] **02.04 — Borrow Checker**
  - [ ] Cara baca error borrow checker
  - [ ] Common error patterns
  - [ ] Tips debug

- [ ] **02.05 — String vs &str**
  - [ ] `String` — owned, heap allocated
  - [ ] `&str` — borrowed, string slice
  - [ ] Konversi antara keduanya
  - [ ] Kapan pakai mana

- [ ] **02.06 — Slices**
  - [ ] Array slices `&[T]`
  - [ ] String slices `&str`
  - [ ] Slice sebagai function parameter

- [ ] **02.07 — Lifetimes**
  - [ ] Apa itu lifetime & kenapa ada
  - [ ] Lifetime annotation syntax `'a`
  - [ ] Lifetime di function & struct
  - [ ] `'static` lifetime
  - [ ] Lifetime elision rules

- [ ] **Mini Project:** In-memory transaction log dengan proper ownership

---

## Module 03 — Type System Lanjutan
> *Estimasi: 3–4 minggu*

- [ ] **03.01 — Traits**
  - [ ] Apa itu trait (vs Go interface)
  - [ ] Defining & implementing trait
  - [ ] Default implementations
  - [ ] Trait bounds: `fn<T: Trait>`
  - [ ] `where` clauses
  - [ ] Trait objects: `&dyn Trait` vs `impl Trait`

- [ ] **03.02 — Common Standard Traits**
  - [ ] `Display` & `Debug`
  - [ ] `Clone` & `Copy`
  - [ ] `From` & `Into`
  - [ ] `PartialEq`, `Eq`, `PartialOrd`, `Ord`
  - [ ] `Default`

- [ ] **03.03 — Generics**
  - [ ] Generic functions
  - [ ] Generic structs & enums
  - [ ] Monomorphization — kenapa zero-cost
  - [ ] vs Go generics

- [ ] **03.04 — Enums Lanjutan**
  - [ ] Enum dengan data (algebraic data types)
  - [ ] Enum dengan berbagai tipe data per variant
  - [ ] Method pada enum

- [ ] **03.05 — Option\<T\>**
  - [ ] Apa itu `Option` & kenapa tidak ada `null`
  - [ ] `Some` & `None`
  - [ ] Pattern matching dengan `Option`
  - [ ] Helper methods: `unwrap`, `unwrap_or`, `map`, `and_then`

- [ ] **03.06 — Pattern Matching Lanjutan**
  - [ ] `match` yang exhaustive
  - [ ] `if let` & `while let`
  - [ ] Destructuring struct & tuple
  - [ ] Guard conditions
  - [ ] `@` bindings

- [ ] **03.07 — Type Aliases & Newtype Pattern**
  - [ ] `type` alias
  - [ ] Newtype pattern — type-safe wrappers
  - [ ] Kapan pakai mana

- [ ] **Mini Project:** Domain model payment transaction (Status enum, Amount newtype, TransactionId newtype)

---

## Module 04 — Error Handling
> *Estimasi: 2 minggu*

- [ ] **04.01 — Result\<T, E\>**
  - [ ] Apa itu `Result`
  - [ ] `Ok` & `Err`
  - [ ] vs Go `(value, error)`
  - [ ] Pattern matching dengan `Result`

- [ ] **04.02 — ? Operator**
  - [ ] Cara kerja `?` operator
  - [ ] Early return otomatis
  - [ ] Syarat penggunaan `?`

- [ ] **04.03 — Custom Error Types**
  - [ ] Membuat error type sendiri
  - [ ] Implement `Error` trait
  - [ ] Implement `Display` untuk error

- [ ] **04.04 — thiserror Crate**
  - [ ] Derive macro untuk error
  - [ ] Error chaining
  - [ ] Kapan pakai `thiserror`

- [ ] **04.05 — anyhow Crate**
  - [ ] Dynamic error type
  - [ ] `context` & `with_context`
  - [ ] Kapan pakai `anyhow` vs `thiserror`

- [ ] **04.06 — Error Propagation Patterns**
  - [ ] Error di library vs application
  - [ ] Error conversion dengan `From`
  - [ ] Error hierarchy

- [ ] **Mini Project:** Error hierarchy payment engine (InsufficientFunds, InvalidAccount, NetworkTimeout)

---

## Module 05 — Memory Management & Smart Pointers
> *Estimasi: 3 minggu*

- [ ] **05.01 — Stack vs Heap**
  - [ ] Perbedaan stack & heap
  - [ ] Kapan Rust alokasi ke heap
  - [ ] Implikasi performa

- [ ] **05.02 — Box\<T\>**
  - [ ] Apa itu `Box`
  - [ ] Heap allocation eksplisit
  - [ ] Recursive types dengan `Box`
  - [ ] `Box` untuk trait objects

- [ ] **05.03 — Rc\<T\>**
  - [ ] Reference counting
  - [ ] `Rc::clone` & reference count
  - [ ] Keterbatasan: single-threaded only
  - [ ] Kapan pakai `Rc`

- [ ] **05.04 — Arc\<T\>**
  - [ ] Atomic reference counting
  - [ ] Thread-safe shared ownership
  - [ ] `Arc` vs `Rc`

- [ ] **05.05 — RefCell\<T\> & Cell\<T\>**
  - [ ] Interior mutability pattern
  - [ ] Runtime borrow checking
  - [ ] `RefCell::borrow` & `borrow_mut`
  - [ ] Kapan pakai `RefCell`

- [ ] **05.06 — Weak\<T\>**
  - [ ] Weak reference
  - [ ] Breaking reference cycles
  - [ ] Upgrade & downgrade

- [ ] **05.07 — Drop Trait**
  - [ ] Custom destructor
  - [ ] Drop order
  - [ ] `std::mem::drop` untuk early drop

- [ ] **Mini Project:** Connection pool sederhana dengan `Arc<Mutex<Vec<Connection>>>`

---

## Module 06 — Functional: Closures & Iterators
> *Estimasi: 2–3 minggu*

- [ ] **06.01 — Closures**
  - [ ] Syntax closure
  - [ ] Capturing environment
  - [ ] `Fn`, `FnMut`, `FnOnce` traits
  - [ ] Move closures
  - [ ] Closure sebagai parameter & return value

- [ ] **06.02 — Iterator Trait**
  - [ ] Apa itu iterator
  - [ ] `next()` method
  - [ ] Lazy evaluation — kenapa efisien
  - [ ] `IntoIterator`

- [ ] **06.03 — Iterator Adapters**
  - [ ] `map` & `flat_map`
  - [ ] `filter` & `filter_map`
  - [ ] `fold` & `reduce`
  - [ ] `chain` & `zip`
  - [ ] `take` & `skip`
  - [ ] `enumerate` & `peekable`
  - [ ] `any` & `all` & `find`

- [ ] **06.04 — Collecting Results**
  - [ ] `collect()` ke `Vec`, `HashMap`, `HashSet`
  - [ ] Type annotation untuk `collect`
  - [ ] `collect` Result — iterator of Results

- [ ] **06.05 — Custom Iterator**
  - [ ] Implement `Iterator` trait
  - [ ] Iterator dengan state
  - [ ] Infinite iterators

- [ ] **06.06 — Collections Deep Dive**
  - [ ] `Vec` — methods & patterns
  - [ ] `HashMap` — Entry API
  - [ ] `HashSet` — set operations

- [ ] **Mini Project:** Transaction report generator (filter → group → sum dengan iterator chains)

---

## Module 07 — Concurrency & Async
> *Estimasi: 4–5 minggu*

- [ ] **07.01 — OS Threads**
  - [ ] `thread::spawn` & `JoinHandle`
  - [ ] `thread::join`
  - [ ] Move closures di thread
  - [ ] vs Go goroutines

- [ ] **07.02 — Message Passing**
  - [ ] `mpsc` channel
  - [ ] `Sender` & `Receiver`
  - [ ] Multiple producers
  - [ ] vs Go channels

- [ ] **07.03 — Shared State**
  - [ ] `Mutex<T>` — mutual exclusion
  - [ ] `RwLock<T>` — read/write lock
  - [ ] `Arc<Mutex<T>>` — thread-safe shared state
  - [ ] Deadlock & cara menghindari

- [ ] **07.04 — Async/Await Dasar**
  - [ ] Apa itu `Future`
  - [ ] `async fn` & `.await`
  - [ ] Perbedaan dari OS threads
  - [ ] vs Go goroutines

- [ ] **07.05 — Tokio Runtime**
  - [ ] Setup Tokio
  - [ ] `tokio::main`
  - [ ] `tokio::spawn` — async task
  - [ ] `tokio::select!` (vs Go `select`)
  - [ ] `tokio::time`: sleep, timeout, interval

- [ ] **07.06 — Tokio Synchronization**
  - [ ] `tokio::sync::Mutex`
  - [ ] `tokio::sync::RwLock`
  - [ ] `tokio::sync::Semaphore`
  - [ ] `tokio::sync::mpsc`, `oneshot`, `broadcast`
  - [ ] Kapan pakai `std` vs `tokio` sync

- [ ] **07.07 — Concurrency Patterns**
  - [ ] Rate limiting
  - [ ] Circuit breaker
  - [ ] Timeout & retry dengan backoff
  - [ ] Worker pool pattern

- [ ] **Mini Project:** Concurrent payment request processor dengan rate limiting dan timeout

---

## Module 08 — Data Structures & Algorithms *(Ongoing)*
> *Paralel dari Module 03 hingga selesai — 1–2 jam/minggu*

- [ ] **08.01 — Built-in Collections**
  - [ ] `Vec<T>` — dynamic array
  - [ ] `VecDeque<T>` — double-ended queue
  - [ ] `HashMap<K,V>` — hash table
  - [ ] `HashSet<T>` — hash set
  - [ ] `BTreeMap<K,V>` — sorted map
  - [ ] `BinaryHeap<T>` — priority queue

- [ ] **08.02 — Implement from Scratch**
  - [ ] Stack dengan `Vec`
  - [ ] Queue dengan `VecDeque`
  - [ ] Linked List dengan `Box<T>` + `Option<T>`
  - [ ] Binary Tree dengan `Box<T>`
  - [ ] Graph dengan `HashMap` (adjacency list)

- [ ] **08.03 — Sorting Algorithms**
  - [ ] Bubble sort
  - [ ] Selection sort
  - [ ] Insertion sort
  - [ ] Merge sort (recursive)
  - [ ] Quick sort

- [ ] **08.04 — Searching Algorithms**
  - [ ] Linear search
  - [ ] Binary search
  - [ ] BFS — Breadth First Search
  - [ ] DFS — Depth First Search

- [ ] **08.05 — Recursion & Dynamic Programming**
  - [ ] Rekursi dasar
  - [ ] Memoization dengan `HashMap`
  - [ ] Fibonacci DP
  - [ ] Classic DP problems

- [ ] **08.06 — Common Patterns LeetCode/HackerRank**
  - [ ] Two pointers
  - [ ] Sliding window
  - [ ] Fast & slow pointers
  - [ ] Tree & graph traversal
  - [ ] Iterator chains vs manual loop
  - [ ] Entry API `HashMap`
  - [ ] Custom sort dengan `Ord` trait

> **Platform:** LeetCode ✅ | HackerRank ✅ | Exercism.io ✅  
> **Target:** 20 soal easy → implement DS scratch → 15 soal medium → 2–3 soal/minggu ongoing

---

## Module 09 — Code Organization & Project Structure
> *Estimasi: 2 minggu*

- [ ] **09.01 — Module System**
  - [ ] `mod` keyword
  - [ ] `pub`, `pub(crate)`, `pub(super)`
  - [ ] `use` & `as`
  - [ ] `super` & `crate`
  - [ ] Re-exports dengan `pub use`

- [ ] **09.02 — File-Based Modules**
  - [ ] `mod` dalam file terpisah
  - [ ] `mod.rs` vs `filename.rs`
  - [ ] Nested modules

- [ ] **09.03 — Crates**
  - [ ] Binary crate vs library crate
  - [ ] `Cargo.toml` — dependencies, features, profiles
  - [ ] Semantic versioning
  - [ ] Dev dependencies vs regular

- [ ] **09.04 — Workspaces**
  - [ ] Setup workspace
  - [ ] Shared dependencies
  - [ ] Monorepo untuk microservices

- [ ] **09.05 — Folder Structure Clean Architecture**
  - [ ] Domain layer
  - [ ] Application layer
  - [ ] Infrastructure layer
  - [ ] API layer

- [ ] **Mini Project:** Scaffold payment service dengan struktur Clean Architecture

---

## Module 10 — Design Patterns in Rust
> *Estimasi: 3–4 minggu*

- [ ] **10.01 — Builder Pattern**
  - [ ] Kenapa Builder di Rust sangat umum
  - [ ] Method chaining
  - [ ] Validasi saat `build()`

- [ ] **10.02 — Strategy Pattern**
  - [ ] Via Traits (compile-time)
  - [ ] Via `Box<dyn Trait>` (runtime)

- [ ] **10.03 — Repository Pattern**
  - [ ] Trait sebagai interface
  - [ ] Implementasi konkret
  - [ ] Mock implementasi untuk testing

- [ ] **10.04 — Dependency Injection**
  - [ ] Via generic type parameter
  - [ ] Via `Box<dyn Trait>`
  - [ ] Kapan pakai mana

- [ ] **10.05 — Newtype Pattern**
  - [ ] Type-safe wrappers
  - [ ] Prevent primitive obsession
  - [ ] Implement methods pada newtype

- [ ] **10.06 — Typestate Pattern**
  - [ ] State machine di compile time
  - [ ] Phantom types
  - [ ] Contoh payment state machine

- [ ] **10.07 — Extension Trait**
  - [ ] Extend behaviour tipe eksternal

- [ ] **10.08 — SOLID in Rust**
  - [ ] SRP via modules
  - [ ] OCP via Traits
  - [ ] LSP via Trait bounds
  - [ ] ISP via multiple Traits
  - [ ] DIP via `Box<dyn Trait>`

- [ ] **Mini Project:** Payment processing pipeline dengan Typestate Pattern

---

## Module 11 — Testing In Depth
> *Estimasi: 3 minggu*

- [ ] **11.01 — Unit Test Lanjutan**
  - [ ] Test organization best practices
  - [ ] Test helpers & fixtures
  - [ ] Setup & teardown
  - [ ] Parameterized tests dengan `rstest`

- [ ] **11.02 — Mocking**
  - [ ] `mockall` crate
  - [ ] Mock dari Trait
  - [ ] Manual mock
  - [ ] Kapan pakai `mockall` vs manual

- [ ] **11.03 — Integration Test**
  - [ ] `tests/` folder
  - [ ] Test isolation
  - [ ] `testcontainers-rs`

- [ ] **11.04 — HTTP Testing**
  - [ ] `axum::test` untuk handler test
  - [ ] `wiremock-rs` untuk mock external API

- [ ] **11.05 — Async Testing**
  - [ ] `#[tokio::test]`
  - [ ] Testing concurrent code
  - [ ] Testing dengan timeout

- [ ] **11.06 — Test Coverage & CI**
  - [ ] `cargo-tarpaulin`
  - [ ] GitHub Actions setup

- [ ] **Mini Project:** Full test suite payment service (unit + integration + API)

---

## Module 12 — Database Integration
> *Estimasi: 3–4 minggu*

- [ ] **12.01 — SQLx Setup**
  - [ ] Koneksi ke PostgreSQL & MySQL
  - [ ] Connection pool dengan `deadpool`
  - [ ] Environment & config

- [ ] **12.02 — SQLx Queries**
  - [ ] `query!` macro — compile-time check
  - [ ] `fetch_one`, `fetch_all`, `fetch_optional`
  - [ ] `execute` untuk INSERT/UPDATE/DELETE
  - [ ] Named binding

- [ ] **12.03 — Transactions**
  - [ ] Begin, commit, rollback
  - [ ] Nested transactions (savepoints)
  - [ ] Error handling di transaction

- [ ] **12.04 — Migrations**
  - [ ] `sqlx-cli`
  - [ ] Up & down migrations
  - [ ] Migration di production

- [ ] **12.05 — Diesel ORM**
  - [ ] Schema definition
  - [ ] CRUD operations
  - [ ] Kapan pakai Diesel vs SQLx

- [ ] **12.06 — Patterns**
  - [ ] Repository pattern dengan SQLx
  - [ ] Unit of Work pattern
  - [ ] DB error mapping ke domain error

- [ ] **Mini Project:** Transaction repository lengkap (save, find_by_id, find_by_status, update_status)

---

## Module 13 — Redis Integration
> *Estimasi: 2–3 minggu*

- [ ] **13.01 — Setup fred Crate**
  - [ ] Koneksi & konfigurasi
  - [ ] Connection pool

- [ ] **13.02 — Basic Operations**
  - [ ] `GET`, `SET`, `DEL`
  - [ ] `EXPIRE` & `TTL`
  - [ ] `EXISTS` & type checking

- [ ] **13.03 — Data Structures Redis**
  - [ ] Hash: `HSET`, `HGET`, `HDEL`
  - [ ] Counter: `INCR`, `DECR`, `INCRBY`
  - [ ] Sorted Sets: `ZADD`, `ZRANGE`, `ZSCORE`
  - [ ] List: `LPUSH`, `RPUSH`, `LPOP`

- [ ] **13.04 — Pub/Sub**
  - [ ] Publisher
  - [ ] Subscriber
  - [ ] Broadcast pattern

- [ ] **13.05 — Patterns untuk Payment**
  - [ ] Idempotency key storage
  - [ ] Rate limiting per user/merchant
  - [ ] Distributed lock (Redlock)
  - [ ] Cache-aside pattern

- [ ] **Mini Project:** Idempotency middleware + Rate limiter

---

## Module 14 — Kafka Integration
> *Estimasi: 3–4 minggu*

- [ ] **14.01 — Setup rdkafka**
  - [ ] Konfigurasi producer & consumer
  - [ ] Topic management

- [ ] **14.02 — Producer**
  - [ ] Sync vs async producer
  - [ ] Message key & partitioning strategy
  - [ ] Delivery report & error handling
  - [ ] Config tuning: acks, retries, batch.size

- [ ] **14.03 — Consumer**
  - [ ] Consumer groups
  - [ ] Auto vs manual offset commit
  - [ ] Rebalancing handling
  - [ ] Config tuning: max.poll.interval

- [ ] **14.04 — Serialization**
  - [ ] JSON dengan serde
  - [ ] Avro dengan schema registry
  - [ ] Protobuf messages

- [ ] **14.05 — Patterns untuk Payment**
  - [ ] Outbox pattern
  - [ ] Saga choreography via events
  - [ ] Dead Letter Queue (DLQ)
  - [ ] Exactly-once semantics

- [ ] **Mini Project:** Payment event pipeline (PaymentInitiated, PaymentProcessed, PaymentFailed, DLQ handler)

---

## Module 15 — Third-Party Integration
> *Estimasi: 3–4 minggu*

- [ ] **15.01 — HTTP Client dengan reqwest**
  - [ ] `GET`, `POST`, `PUT`, `DELETE`
  - [ ] Headers & authentication
  - [ ] JSON request/response dengan serde
  - [ ] Timeout configuration
  - [ ] Connection pooling

- [ ] **15.02 — Resiliency Patterns**
  - [ ] Retry dengan exponential backoff
  - [ ] Circuit breaker
  - [ ] Timeout & deadline propagation
  - [ ] Bulkhead pattern

- [ ] **15.03 — Payment Gateway Integration**
  - [ ] Midtrans API
  - [ ] Xendit API
  - [ ] SNAP BI
  - [ ] ISO8583 via TCP
  - [ ] Webhook handler & signature verification

- [ ] **15.04 — Authentication Third-Party**
  - [ ] OAuth2 flow
  - [ ] JWT validation dengan `jsonwebtoken` crate
  - [ ] mTLS untuk inter-service

- [ ] **Mini Project:** Payment gateway adapter (Trait + Midtrans/Xendit implementasi + Mock)

---

## Module 16 — REST API & gRPC
> *Estimasi: 4–5 minggu*

- [ ] **16.01 — Axum Fundamentals**
  - [ ] Router & routes
  - [ ] Handlers & extractors
  - [ ] State management
  - [ ] Nested routes

- [ ] **16.02 — Request & Response**
  - [ ] JSON dengan serde
  - [ ] Path, Query, Header extractors
  - [ ] Request body validation
  - [ ] Response types & status codes

- [ ] **16.03 — Middleware di Axum**
  - [ ] tower middleware
  - [ ] Auth middleware: JWT, API Key
  - [ ] Logging middleware
  - [ ] CORS

- [ ] **16.04 — Error Handling di HTTP Layer**
  - [ ] `IntoResponse` untuk error types
  - [ ] Consistent error response format

- [ ] **16.05 — Tonic & gRPC**
  - [ ] Protobuf definition (`.proto` files)
  - [ ] Code generation dengan `tonic-build`
  - [ ] Server implementation
  - [ ] Client implementation
  - [ ] Streaming: unary, server, client, bidirectional

- [ ] **16.06 — Interceptors**
  - [ ] Auth interceptor
  - [ ] Logging interceptor

- [ ] **16.07 — REST vs gRPC**
  - [ ] Kapan pakai mana

- [ ] **Mini Project:** Payment API (REST: initiation, status check | gRPC: fraud check, settlement)

---

## Module 17 — Security
> *Estimasi: 3–4 minggu*

- [ ] **17.01 — Authentication & Authorization**
  - [ ] JWT: generate, validate, refresh token
  - [ ] OAuth2 flow lengkap
  - [ ] API Key management
  - [ ] RBAC (Role-Based Access Control)

- [ ] **17.02 — Cryptography**
  - [ ] Password hashing: Argon2, bcrypt
  - [ ] Symmetric encryption: AES-GCM
  - [ ] Asymmetric encryption: RSA, ECDSA
  - [ ] Hashing: SHA-256, HMAC
  - [ ] Crates: `ring`, `rust-crypto`, `argon2`

- [ ] **17.03 — Input Validation & Sanitization**
  - [ ] `validator` crate
  - [ ] Sanitasi input user
  - [ ] Prevent injection attacks

- [ ] **17.04 — Secure Communication**
  - [ ] TLS configuration di Axum
  - [ ] mTLS untuk inter-service
  - [ ] Certificate management
  - [ ] Security headers (HSTS, CSP, X-Frame-Options)

- [ ] **17.05 — Secrets Management**
  - [ ] Environment variables
  - [ ] HashiCorp Vault integration
  - [ ] Secret rotation

- [ ] **17.06 — OWASP Top 10**
  - [ ] Injection (SQL, NoSQL, Command)
  - [ ] Broken Authentication
  - [ ] Sensitive Data Exposure
  - [ ] Security Misconfiguration
  - [ ] Logging & Monitoring

- [ ] **17.07 — PCI-DSS Awareness**
  - [ ] Relevant requirements untuk payment
  - [ ] Cardholder data protection
  - [ ] Audit trail requirements
  - [ ] Tokenization concepts

- [ ] **17.08 — Audit Logging & Anomaly Detection**
  - [ ] Structured security logs
  - [ ] Sensitive operation tracking
  - [ ] Alerting patterns

- [ ] **Mini Project:** Secure payment API (JWT + RBAC + encrypted data + full audit trail)

---

## Module 18 — Architecture & Production
> *Estimasi: 4–5 minggu*

- [ ] **18.01 — Clean Architecture**
  - [ ] Layer separation
  - [ ] Dependency rule
  - [ ] Implementasi di Rust

- [ ] **18.02 — Hexagonal / Ports & Adapters**
  - [ ] Ports (Traits)
  - [ ] Adapters (Implementations)
  - [ ] Cara test dengan mock adapters

- [ ] **18.03 — Domain-Driven Design**
  - [ ] Entities & Value Objects
  - [ ] Aggregates
  - [ ] Domain events
  - [ ] Bounded context

- [ ] **18.04 — Microservices Patterns**
  - [ ] Saga pattern untuk distributed transactions
  - [ ] Outbox pattern
  - [ ] CQRS basics

- [ ] **18.05 — Observability**
  - [ ] Structured logging dengan `tracing` crate
  - [ ] Metrics dengan prometheus
  - [ ] Distributed tracing

- [ ] **18.06 — Production Readiness**
  - [ ] Config management dengan `config` crate
  - [ ] Health check endpoint
  - [ ] Graceful shutdown
  - [ ] Docker & containerization

---

## 🏆 Capstone Project — Mini Payment Gateway

> Integrasi semua modul dalam satu project nyata

- [ ] REST API → Axum: payment initiation, status check
- [ ] gRPC → Tonic: fraud check, settlement service
- [ ] Database → PostgreSQL via SQLx
- [ ] Cache → Redis: idempotency, rate limit, session
- [ ] Events → Kafka: saga & outbox pattern
- [ ] Third-Party → Midtrans/Xendit dengan circuit breaker
- [ ] Security → JWT + RBAC + encryption + audit trail
- [ ] Testing → Unit + Integration + E2E
- [ ] Architecture → Clean Architecture + Hexagonal

---

## Timeline

| Bulan | Modul |
|-------|-------|
| Bulan 1 | Module 01 + 02 |
| Bulan 2 | Module 03 + 04 |
| Bulan 3 | Module 05 + 06 + DSA Part 1 |
| Bulan 4 | Module 07 + DSA Part 2 |
| Bulan 5 | Module 09 + 10 + DSA Part 3 |
| Bulan 6 | Module 11 + 12 + DSA Part 4 |
| Bulan 7 | Module 13 + 14 |
| Bulan 8 | Module 15 + 16 |
| Bulan 9 | Module 17 + 18 |

---

*Checklist ini bisa di-update setiap kali menyelesaikan topik. Ubah `- [ ]` menjadi `- [x]` untuk menandai progress.*
