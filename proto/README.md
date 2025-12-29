# Proto Files Documentation

This directory contains the Protocol Buffer definitions for the USCA Component Framework.

## Files

### common.proto
Common types and definitions used across all modules:
- `ComponentId`: Unique identifier for components
- `ComponentStatus`: Status enumeration (INITIALIZING, READY, RUNNING, STOPPED, ERROR)
- `Timestamp`: Microsecond-precision timestamps
- `Result`: Generic result wrapper for operations

### component.proto
Component definitions and lifecycle management:
- `Component`: Component metadata (id, name, description, version, type, dependencies)
- `ComponentType`: Types of components (SENSOR, ACTUATOR, CONTROLLER, SERVICE)
- `LifecycleCommand`: Commands for component lifecycle (INITIALIZE, START, STOP, SHUTDOWN)
- `ComponentState`: Current state of a component with properties

### messaging.proto
Message passing between components:
- `Message`: Message structure with sender, receiver, topic, payload, priority
- `MessagePriority`: Priority levels (LOW, NORMAL, HIGH, CRITICAL)
- `MessageQueue`: Queue metadata and management
- `Subscription`: Topic subscriptions with priority filtering

### config.proto
Configuration management:
- `SystemConfig`: Overall system configuration
- `ComponentConfig`: Per-component configuration
- `ConfigValue`: Flexible configuration value wrapper supporting:
  - String values
  - Integer values
  - Double values
  - Boolean values
  - Binary data
- `ConfigUpdateRequest`: Configuration update messages

## Design Principles

1. **Modularity**: Each proto file focuses on a specific concern
2. **Extensibility**: Using `oneof` and maps for flexible configurations
3. **Type Safety**: Strong typing for all entities and operations
4. **Interoperability**: Protocol Buffers enable cross-language compatibility
5. **Embedded Focus**: Lightweight structures suitable for resource-constrained systems

## Building

Proto files are automatically compiled during the Rust build process via `build.rs`. Generated Rust code is placed in `src/generated/` and is excluded from version control.

## Adding New Proto Files

1. Create the new `.proto` file in this directory
2. Add the file path to `proto_files` array in `build.rs`
3. Run `cargo build` to generate the Rust code
4. Re-export the generated module in `src/lib.rs`
