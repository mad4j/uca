//! USCA - Component Framework for Embedded Systems
//!
//! This library provides a schema-centric architecture (SCA) framework
//! for building component-based embedded systems.
//!
//! # Modules
//!
//! - `common`: Common types and definitions
//! - `component`: Component definitions and lifecycle management
//! - `messaging`: Message passing between components
//! - `config`: Configuration management

// Include generated protobuf code
mod generated {
    // Include all generated proto files into the private module
    include!("generated/usca.common.rs");
    include!("generated/usca.component.rs");
    include!("generated/usca.messaging.rs");
    include!("generated/usca.config.rs");
}

// Re-export main types for convenience with explicit imports
pub mod common {
    //! Common types and definitions used across the framework
    pub use crate::generated::{ComponentId, ComponentStatus, Result, Timestamp};
}

pub mod component {
    //! Component definitions and lifecycle management
    pub use crate::generated::{
        Component, ComponentState, ComponentType, LifecycleAction, LifecycleCommand,
    };
}

pub mod messaging {
    //! Message passing between components
    pub use crate::generated::{Message, MessagePriority, MessageQueue, Subscription};
}

pub mod config {
    //! Configuration management
    pub use crate::generated::{
        config_value, ComponentConfig, ConfigUpdateRequest, ConfigValue, SystemConfig,
    };
}

// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_component_creation() {
        use component::{Component, ComponentType};
        use common::ComponentId;

        let comp = Component {
            id: Some(ComponentId {
                id: "test-component".to_string(),
            }),
            name: "Test Component".to_string(),
            description: "A test component".to_string(),
            version: "1.0.0".to_string(),
            r#type: ComponentType::Sensor as i32,
            dependencies: vec![],
        };

        assert_eq!(comp.name, "Test Component");
    }

    #[test]
    fn test_message_creation() {
        use common::{ComponentId, Timestamp};
        use messaging::{Message, MessagePriority};

        let msg = Message {
            sender: Some(ComponentId {
                id: "sender".to_string(),
            }),
            receiver: Some(ComponentId {
                id: "receiver".to_string(),
            }),
            timestamp: Some(Timestamp { micros: 123456789 }),
            topic: "test/topic".to_string(),
            payload: vec![1, 2, 3, 4],
            priority: MessagePriority::Normal as i32,
        };

        assert_eq!(msg.topic, "test/topic");
    }
}
