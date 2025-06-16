# Asynchronous Rust Primitives with Tokio

## Author
Luis Carlos Osorio Jayk ([@luiscarlosjayk](https://github.com/luiscarlosjayk))

## Description
A comprehensive guide and practical examples of using Tokio's asynchronous primitives in Rust. This project serves as both a learning resource and a reference for implementing asynchronous patterns in Rust applications.

## Prerequisites
- Rust (latest stable version recommended)
- Cargo (Rust's package manager)
- Basic understanding of Rust programming language
- Familiarity with asynchronous programming concepts

## Topics
The topics covered in this project are:

### 1. A brief introduction to concepts related to asynchronous programming

Here we present different concepts like tasks, parallelism, concurrency, threads, await,
that will help us understand a little better what's being provided by the Tokio framework.

### 2. Tokio's framework introduction that are backbone for understanding its asynchronous primitives

In this section we discover how to setup the Tokio's runtime, how to create our first task, what's a blocking task and how to spawn one thus we don't block threads, and some useful macros like Join and Select.

### 3. Tokio's asynchronous primitives
Here we discuss some useful primitives provided by Tokio, like Oncecell, Mutex, Semaphore, Notify, Barrier, RwLock, and different types of channels like: Oneshot, Mpsc, Watch and Broadcast.

Each one is explained by short slides and a demo.

## How to follow presentation
This project uses VsCode extension [Demo Time](https://demotime.elio.dev) for slides and code examples generation by steps.

All demos are listed in the [.demo](/.demo) directory.

Finished codebase of demos can be found at [/src/complete](./src/completed/)

## Slides

Slides are written in markdown format and can be found at [/.demo/slides](/.demo/slides)

## Project Structure
```
.
├── src/
│   ├── completed/    # Complete working examples
│   ├── examples/     # Interactive examples used during live coding sessions
│   └── ...           # Other source files
├── .demo/
│   ├── content/      # Core presentation content and learning materials
│   ├── patches/      # Code changes and modifications for each demo step
│   ├── slides/       # Main presentation slides in markdown format
│   ├── snapshots/    # Code state captures at different presentation stages
│   ├── snippets/     # Reusable code examples and templates
│   └── ...           # Demo files
├── .assets/
│   └── ...           # Supporting media files, diagrams, and visual resources
└── README.md
```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments
- Tokio team for their excellent async runtime
- The Rust community for their continuous support and resources

## Resources
- [Tokio Website](https://tokio.rs/) - Official website with guides and documentation
- [Tokio Documentation](https://docs.rs/tokio/latest/tokio/) - API documentation and examples
- [Tokio GitHub Repository](https://github.com/tokio-rs/tokio) - Source code and issues

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

