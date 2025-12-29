//! Example: Basic component and messaging usage
//!
//! This example demonstrates how to:
//! - Create components
//! - Send messages between components
//! - Manage component state
//! - Handle configuration

use usca::common::{ComponentId, ComponentStatus, Timestamp};
use usca::component::{Component, ComponentState, ComponentType, LifecycleAction, LifecycleCommand};
use usca::config::{ComponentConfig, ConfigValue};
use usca::messaging::{Message, MessagePriority, Subscription};

fn main() {
    println!("USCA Component Framework - Basic Example\n");

    // Create a sensor component
    let sensor = create_sensor_component();
    println!("Created sensor component:");
    println!("  ID: {:?}", sensor.id.as_ref().unwrap().id);
    println!("  Name: {}", sensor.name);
    println!("  Type: {:?}", ComponentType::try_from(sensor.r#type).unwrap());
    println!();

    // Create an actuator component
    let actuator = create_actuator_component();
    println!("Created actuator component:");
    println!("  ID: {:?}", actuator.id.as_ref().unwrap().id);
    println!("  Name: {}", actuator.name);
    println!();

    // Create component state
    let state = create_component_state(&sensor.id.as_ref().unwrap());
    println!("Component state:");
    println!("  Status: {:?}", ComponentStatus::try_from(state.status).unwrap());
    println!("  Last update: {} micros", state.last_update.as_ref().unwrap().micros);
    println!();

    // Create a message from sensor to actuator
    let message = create_message(
        &sensor.id.as_ref().unwrap(),
        &actuator.id.as_ref().unwrap(),
        "sensor/temperature",
        vec![25, 30], // Temperature: 25.30°C
    );
    println!("Created message:");
    println!("  From: {}", message.sender.as_ref().unwrap().id);
    println!("  To: {}", message.receiver.as_ref().unwrap().id);
    println!("  Topic: {}", message.topic);
    println!("  Priority: {:?}", MessagePriority::try_from(message.priority).unwrap());
    println!();

    // Create a subscription
    let subscription = Subscription {
        subscriber: actuator.id.clone(),
        topics: vec!["sensor/temperature".to_string(), "sensor/humidity".to_string()],
        min_priority: MessagePriority::Normal as i32,
    };
    println!("Created subscription:");
    println!("  Subscriber: {}", subscription.subscriber.as_ref().unwrap().id);
    println!("  Topics: {:?}", subscription.topics);
    println!();

    // Create component configuration
    let config = create_component_config(&sensor.id.as_ref().unwrap());
    println!("Component configuration:");
    for (key, value) in &config.parameters {
        println!("  {}: {:?}", key, value);
    }
    println!();

    // Lifecycle command
    let command = LifecycleCommand {
        component_id: sensor.id.clone(),
        action: LifecycleAction::Start as i32,
    };
    println!("Lifecycle command:");
    println!("  Component: {}", command.component_id.as_ref().unwrap().id);
    println!("  Action: {:?}", LifecycleAction::try_from(command.action).unwrap());
    println!();

    println!("Example completed successfully!");
}

fn create_sensor_component() -> Component {
    Component {
        id: Some(ComponentId {
            id: "sensor-temp-001".to_string(),
        }),
        name: "Temperature Sensor".to_string(),
        description: "DHT22 temperature and humidity sensor".to_string(),
        version: "1.0.0".to_string(),
        r#type: ComponentType::Sensor as i32,
        dependencies: vec![],
    }
}

fn create_actuator_component() -> Component {
    Component {
        id: Some(ComponentId {
            id: "actuator-fan-001".to_string(),
        }),
        name: "Cooling Fan".to_string(),
        description: "PWM-controlled cooling fan".to_string(),
        version: "1.0.0".to_string(),
        r#type: ComponentType::Actuator as i32,
        dependencies: vec!["sensor-temp-001".to_string()],
    }
}

fn create_component_state(component_id: &ComponentId) -> ComponentState {
    ComponentState {
        component_id: Some(component_id.clone()),
        status: ComponentStatus::Running as i32,
        last_update: Some(Timestamp {
            micros: 1640000000000000, // Example timestamp
        }),
        properties: vec![
            ("temperature".to_string(), "25.3".to_string()),
            ("humidity".to_string(), "45.2".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}

fn create_message(
    sender: &ComponentId,
    receiver: &ComponentId,
    topic: &str,
    payload: Vec<u8>,
) -> Message {
    Message {
        sender: Some(sender.clone()),
        receiver: Some(receiver.clone()),
        timestamp: Some(Timestamp {
            micros: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as i64,
        }),
        topic: topic.to_string(),
        payload,
        priority: MessagePriority::Normal as i32,
    }
}

fn create_component_config(component_id: &ComponentId) -> ComponentConfig {
    let mut parameters = std::collections::HashMap::new();

    parameters.insert(
        "sample_rate".to_string(),
        ConfigValue {
            value: Some(usca::config::config_value::Value::IntValue(1000)),
        },
    );

    parameters.insert(
        "enabled".to_string(),
        ConfigValue {
            value: Some(usca::config::config_value::Value::BoolValue(true)),
        },
    );

    parameters.insert(
        "device_path".to_string(),
        ConfigValue {
            value: Some(usca::config::config_value::Value::StringValue(
                "/dev/i2c-1".to_string(),
            )),
        },
    );

    ComponentConfig {
        component_id: Some(component_id.clone()),
        parameters,
    }
}
