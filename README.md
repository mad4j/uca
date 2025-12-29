# USCA - Component Framework for Embedded Systems

A Schema-Centric Architecture (SCA) library for building component-based embedded systems using Protocol Buffers.

## Overview

USCA provides a robust framework for developing modular embedded systems with well-defined component interfaces. The library is built using a schema-centric approach where all data structures are defined in Protocol Buffer files, ensuring type safety, interoperability, and clear contracts between components.

## Features

- **Multiple Proto Specifications**: Organized into logical modules:
  - `common.proto`: Common types and definitions (ComponentId, Status, Timestamp, Result)
  - `component.proto`: Component definitions and lifecycle management
  - `messaging.proto`: Message passing between components with priority queuing
  - `config.proto`: Configuration management with flexible value types

- **Type-Safe API**: All types are generated from Protocol Buffers
- **Modular Design**: Clean separation of concerns across modules
- **Embedded Systems Focus**: Lightweight and efficient for resource-constrained environments

## Building the Library

### Prerequisites

- Rust 1.70 or later
- Protocol Buffers compiler (`protoc`)

Install `protoc` on Debian/Ubuntu:
```bash
sudo apt-get install protobuf-compiler
```

### Build

```bash
cargo build
```

The build process automatically:
1. Compiles all `.proto` files in the `proto/` directory
2. Generates Rust code in `src/generated/`
3. Builds the library

### Test

```bash
cargo test
```

## Usage Example

```rust
use usca::component::{Component, ComponentType};
use usca::common::ComponentId;
use usca::messaging::{Message, MessagePriority};

// Create a component
let component = Component {
    id: Some(ComponentId {
        id: "sensor-001".to_string(),
    }),
    name: "Temperature Sensor".to_string(),
    description: "DHT22 temperature sensor".to_string(),
    version: "1.0.0".to_string(),
    r#type: ComponentType::Sensor as i32,
    dependencies: vec![],
};

// Create a message
let message = Message {
    sender: Some(ComponentId { id: "sensor-001".to_string() }),
    receiver: Some(ComponentId { id: "controller-001".to_string() }),
    topic: "temperature/reading".to_string(),
    payload: vec![25, 30], // Example: 25.30°C
    priority: MessagePriority::Normal as i32,
    ..Default::default()
};
```

## Project Structure

```
usca/
├── proto/              # Protocol Buffer definitions
│   ├── common.proto    # Common types
│   ├── component.proto # Component definitions
│   ├── messaging.proto # Message passing
│   └── config.proto    # Configuration
├── src/
│   ├── lib.rs         # Main library file
│   └── generated/     # Generated Rust code (auto-generated)
├── build.rs           # Build script for proto compilation
└── Cargo.toml         # Rust package manifest
```

## Architecture

The library follows a Schema-Centric Architecture (SCA) approach:

1. **Schema First**: All data structures are defined in `.proto` files
2. **Code Generation**: Rust code is automatically generated from schemas
3. **Type Safety**: Strong typing ensures compile-time correctness
4. **Interoperability**: Protocol Buffers enable language-agnostic communication

## License

MIT
