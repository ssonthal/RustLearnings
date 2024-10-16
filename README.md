# RustLearnings

# Important Libraries in Rust

What all libraries does rust have?
A lot of them
https://actix.rs/ - Extremely fast http server
https://serde.rs/ - Serializing and deserialising data in rust
https://tokio.rs/ - Asynchronous runtime in rust
https://docs.rs/reqwest/latest/reqwest/ - Send HTTP requests
https://docs.rs/sqlx/latest/sqlx/ - Connect to sql database

chrono
rand

Leftovers - Traits, Generics and Lifetimes, Multithreading, macros, async ops (Futures)


ChatGPT suggestions on what to build from easy to expert level => 

It’s great that you’ve learned the concepts of Rust and now want to dive into practical coding! Here’s a list of projects, arranged from easy to hard, that will help you solidify your Rust skills:

Beginner Level
CLI Utilities:

Build small command-line tools. Rust’s ownership model and concurrency support make it ideal for this.
Project 1: To-Do List CLI
Create a simple command-line to-do list application where you can add, edit, and delete tasks. This will help you get comfortable with standard I/O and basic Rust syntax.
Project 2: File Renamer
A tool to bulk rename files in a directory based on user input or predefined patterns (e.g., adding prefixes, suffixes, or replacing substrings). Learn file I/O, error handling, and string manipulation.
Basic Web Scraper:

Project 3: Create a web scraper that extracts data from a website (e.g., scrape headlines from a news website). Use the reqwest and scraper crates. This will introduce you to working with external libraries and handling HTTP requests.
Intermediate Level
Multithreaded Prime Number Finder:

Project 4: Write a program that finds prime numbers within a given range, using multithreading to speed up the process. This will help you get more comfortable with Rust’s concurrency features like threads and channels.
REST API Backend:

Project 5: Build a basic REST API using Rocket or Actix (popular Rust web frameworks). The API could manage simple resources like books, users, etc. This will teach you how to structure a Rust web application, work with HTTP methods, and manage data in a structured format like JSON.
Mini HTTP Server:

Project 6: Build a simple HTTP server from scratch using raw TCP sockets. Serve basic HTML files or JSON responses. This will help you understand networking at a lower level and gain more insight into how servers work internally.
Advanced Level
Game Development:

Project 7: Use Bevy or Amethyst to create a simple 2D game, like Snake or Tetris. Game development will give you experience with real-time updates, rendering, and handling events. It’s a fun way to improve your Rust skills while building something visual and interactive.
Concurrency with Tokio/Async:

Project 8: Build a concurrent web crawler or file downloader using Tokio (an asynchronous runtime for Rust). This project will solidify your understanding of asynchronous programming in Rust and help you work on network-related tasks.
Compiler or Interpreter:

Project 9: Build a simple language interpreter or a compiler for a toy language. Start small, such as implementing a basic calculator language that supports arithmetic operations. Rust’s memory management and type system make it a great fit for such projects.
Expert Level
Blockchain from Scratch:

Project 10: Create a simple blockchain implementation from scratch, including features like block creation, validation, and proof-of-work. If you're interested in Web3, this will be a great project to explore how distributed ledgers work, and you can integrate concepts like cryptography.
Operating System:

Project 11: Write a basic operating system kernel from scratch. There are resources like the Writing an OS in Rust tutorial to guide you through the process. This is an extremely challenging project that will give you deep insights into low-level programming.
Continuous Learning:
Contribute to Open-Source: Start contributing to open-source Rust projects on GitHub. This will expose you to large codebases, help you collaborate with others, and solidify your understanding of Rust’s best practices.

