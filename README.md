# My Rust Project

This project is a Rust application designed to manage and search for products. It includes various modules that handle product definitions, search indexing, utility functions, and a planned command-line interface.

## Project Structure

```
my-rust-project
├── src
│   ├── main.rs          # Entry point of the application
│   ├── lib.rs           # Main library file integrating various modules
│   ├── produto.rs       # Defines the Produto struct and its methods
│   ├── indice.rs        # Defines the IndiceBusca struct for searching products
│   ├── utils.rs         # Utility functions for handling stopwords and filtering
│   └── cli.rs           # Future development for command-line interface
├── tests
│   └── test_indice.rs   # Automated tests for IndiceBusca functionality
└── Cargo.toml           # Configuration file for the Rust project
```

## Features

- **Produto Management**: Define and manage products with associated properties.
- **Search Functionality**: Efficiently search and index products using the IndiceBusca struct.
- **Utility Functions**: Handle common tasks such as filtering search results and managing stopwords.
- **Future CLI Development**: A planned command-line interface for user interaction.

## Getting Started

To get started with the project, clone the repository and run the following commands:

```bash
cargo build
cargo run
```

## Testing

To run the tests for the IndiceBusca functionality, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.