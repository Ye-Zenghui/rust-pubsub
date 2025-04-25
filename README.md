Rust PubSub
A thread-safe, in-memory publish-subscribe library for Rust, designed for efficient inter-thread communication. It supports multiple publishers and subscribers across files, arbitrary message formats, customizable queue-full behavior, and callback processing in dedicated threads, ideal for modular Rust projects.
中文文档
Features

Thread-Safe: Built with crossbeam-channel for safe message passing between threads.
Multi-Publisher and Multi-Subscriber: Supports multiple publishers and subscribers for the same topic.
Modular Design: Enables publishing and subscribing across files and modules via a singleton PubSub instance.
Arbitrary Message Formats: Supports any type implementing Send + Sync + Clone + 'static, including custom structs.
Customizable Queue Behavior: Each subscriber can choose to overwrite old messages or stop writing when the queue is full.
Callback Processing: Callbacks run in dedicated threads for non-blocking operation.
Flexible Subscriptions: Offers manual receiving and callback-based subscriptions.
Timeout Support: Provides blocking, non-blocking, and timeout-based operations.

Installation
Add to your Cargo.toml:
[dependencies]
rust-pubsub = "0.1.0"

Usage Examples
Below are examples demonstrating multi-publisher and multi-subscriber communication, arbitrary message formats (using a custom struct), customizable queue-full behavior, callback processing in dedicated threads, and modular design across files.
1. Multi-Publisher and Multi-Subscriber Across Files with Custom Struct
This example shows two publishers and two subscribers in separate files, using a custom struct CustomMessage as the message type. One subscriber uses a callback in a dedicated thread, highlighting modular design and arbitrary message formats.
src/main.rs
use rust_pubsub::TopicConfig;
use std::thread;

mod publisher;
mod subscriber;

fn main() {
    // Start subscribers
    subscriber::manual_subscriber();
    subscriber::callback_subscriber();

    // Start publishers
    thread::spawn(|| publisher::publisher_one());
    thread::spawn(|| publisher::publisher_two());

    // Wait to observe
    thread::sleep(std::time::Duration::from_millis(100));
}

src/publisher.rs
use rust_pubsub::{PubSub, TopicConfig};

// Custom message struct
#[derive(Clone)]
pub struct CustomMessage {
    pub id: u32,
    pub content: String,
}

pub fn publisher_one() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);
    pubsub.publish(topic_id, CustomMessage {
        id: 1,
        content: "Message from Publisher 1".to_string(),
    });
}

pub fn publisher_two() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);
    pubsub.publish(topic_id, CustomMessage {
        id: 2,
        content: "Message from Publisher 2".to_string(),
    });
}

src/subscriber.rs
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

// Custom message struct (must be defined or imported)
#[derive(Clone)]
pub struct CustomMessage {
    pub id: u32,
    pub content: String,
}

pub fn manual_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, false); // Stop writing when full
    let receiver = pubsub.subscribe_manual::<CustomMessage>(topic, config);
    thread::spawn(move || {
        if let Some(msg) = receiver.try_recv() {
            println!("Manual Subscriber: ID={}, Content={}", msg.id, msg.content);
        }
    });
}

pub fn callback_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, false); // Stop writing when full
    pubsub.subscribe::<CustomMessage, _>(topic, config, |msg: &CustomMessage| {
        println!("Callback Subscriber: ID={}, Content={}", msg.id, msg.content); // Runs in a dedicated thread
    });
}

2. Customizable Queue-Full Behavior
This example shows two subscribers with different queue-full behaviors: one overwrites old messages, the other stops writing when the queue is full.
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "queue_topic";
let overwrite_config = TopicConfig::new(2, true); // Overwrite when full
let no_write_config = TopicConfig::new(2, false); // Stop writing when full

// Subscriber 1: Overwrite mode
let receiver1 = pubsub.subscribe_manual::<String>(topic, overwrite_config);
thread::spawn(move || {
    while let Some(msg) = receiver1.try_recv() {
        println!("Overwrite Subscriber: {}", msg);
    }
});

// Subscriber 2: No-write mode
let receiver2 = pubsub.subscribe_manual::<String>(topic, no_write_config);
thread::spawn(move || {
    while let Some(msg) = receiver2.try_recv() {
        println!("No-Write Subscriber: {}", msg);
    }
});

// Publish messages to fill queues (could be in a separate file)
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "Message 1".to_string());
pubsub.publish(topic_id, "Message 2".to_string());
pubsub.publish(topic_id, "Message 3".to_string()); // Overwrite affects Subscriber 1 only

// Wait to observe
thread::sleep(std::time::Duration::from_millis(100));

3. Callback Subscription with Dedicated Thread
This example demonstrates a callback-based subscription, with the callback running in a dedicated thread. Any message type can be used.
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "callback_topic";
let config = TopicConfig::new(5, true); // Overwrite when full

// Subscribe with callback (could be in a separate file)
pubsub.subscribe::<String, _>(topic, config, |msg: &String| {
    println!("Callback: {}", msg); // Runs in a dedicated thread
});

// Publish (could be in a separate file)
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "Callback Message".to_string());

// Wait to observe
thread::sleep(std::time::Duration::from_millis(100));

4. Publishing with Timeout
This example shows publishing with a timeout, receiving in a separate thread. Any message type can be used.
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "timeout_topic";
let config = TopicConfig::new(1, false); // Stop writing when full

// Subscriber thread (could be in a separate file)
let receiver = pubsub.subscribe_manual::<i32>(topic, config);
thread::spawn(move || {
    if let Some(msg) = receiver.recv_timeout(Some(100)) {
        println!("Received: {}", msg);
    }
});

// Publish with timeout (could be in a separate file)
let topic_id = pubsub.create_publisher(topic);
pubsub.publish_with_timeout(topic_id, 42, Some(100));

// Wait to observe
thread::sleep(std::time::Duration::from_millis(100));

5. Overwrite Mode with Full Queue
This example demonstrates publishing to a full queue with overwrite mode, receiving in a separate thread. Any message type can be used.
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "overwrite_topic";
let config = TopicConfig::new(2, true); // Overwrite when full

// Subscriber thread (could be in a separate file)
let receiver = pubsub.subscribe_manual::<String>(topic, config);
thread::spawn(move || {
    while let Some(msg) = receiver.try_recv() {
        println!("Received: {}", msg);
    }
});

// Publish to fill and overwrite queue (could be in a separate file)
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "Message 1".to_string());
pubsub.publish(topic_id, "Message 2".to_string());
pubsub.publish(topic_id, "Message 3".to_string()); // Overwrites oldest message

// Wait to observe
thread::sleep(std::time::Duration::from_millis(100));

License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

中文文档
Rust PubSub 是一个线程安全的、基于内存的发布-订阅库，专为 Rust 设计，旨在实现高效的线程间通信。它支持多个发布者和订阅者、任意消息格式、跨文件通信、每个订阅者可自定义队列满行为以及在专用线程中处理回调，非常适合模块化的 Rust 项目。
English Documentation
功能

线程安全：使用 crossbeam-channel 实现线程间安全消息传递。
多发布者和多订阅者：支持同一主题的多个发布者和订阅者。
模块化设计：通过单例 PubSub 实例，支持跨文件和模块的发布和订阅。
任意消息格式：支持任何实现 Send + Sync + Clone + 'static 的类型，包括自定义结构体。
可自定义队列行为：每个订阅者可配置队列满时覆盖旧消息或停止写入。
回调处理：回调在单独线程中处理，确保非阻塞。
灵活订阅：支持手动接收和基于回调的订阅。
超时支持：提供阻塞、非阻塞和带超时操作。

安装
在 Cargo.toml 中添加：
[dependencies]
rust-pubsub = "0.1.0"

使用示例
以下示例展示了多发布者和多订阅者通信、任意消息格式（使用自定义结构体）、可自定义队列满行为、在专用线程中处理回调以及跨文件模块化设计。
1. 跨文件的多发布者和多订阅者（使用自定义结构体）
此示例展示在不同文件中定义的两个发布者和两个订阅者，使用自定义结构体 CustomMessage 作为消息类型。一个订阅者使用回调，在内部专用线程中处理。
src/main.rs
use rust_pubsub::TopicConfig;
use std::thread;

mod publisher;
mod subscriber;

fn main() {
    // 启动订阅者
    subscriber::manual_subscriber();
    subscriber::callback_subscriber();

    // 启动发布者
    thread::spawn(|| publisher::publisher_one());
    thread::spawn(|| publisher::publisher_two());

    // 等待观察
    thread::sleep(std::time::Duration::from_millis(100));
}

src/publisher.rs
use rust_pubsub::{PubSub, TopicConfig};

// 自定义消息结构体
#[derive(Clone)]
pub struct CustomMessage {
    pub id: u32,
    pub content: String,
}

pub fn publisher_one() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);
    pubsub.publish(topic_id, CustomMessage {
        id: 1,
        content: "来自发布者 1 的消息".to_string(),
    });
}

pub fn publisher_two() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);
    pubsub.publish(topic_id, CustomMessage {
        id: 2,
        content: "来自发布者 2 的消息".to_string(),
    });
}

src/subscriber.rs
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

// 自定义消息结构体（必须定义或导入）
#[derive(Clone)]
pub struct CustomMessage {
    pub id: u32,
    pub content: String,
}

pub fn manual_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, false); // 队列满时停止写入
    let receiver = pubsub.subscribe_manual::<CustomMessage>(topic, config);
    thread::spawn(move || {
        if let Some(msg) = receiver.try_recv() {
            println!("手动订阅者：ID={}, 内容={}", msg.id, msg.content);
        }
    });
}

pub fn callback_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, false); // 队列满时停止写入
    pubsub.subscribe::<CustomMessage, _>(topic, config, |msg: &CustomMessage| {
        println!("回调订阅者：ID={}, 内容={}", msg.id, msg.content); // 在专用线程中运行
    });
}

2. 可自定义队列满行为
此示例展示两个订阅者配置不同的队列满行为：一个覆盖旧消息，另一个队列满时停止写入。可使用任意消息类型。
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "queue_topic";
let overwrite_config = TopicConfig::new(2, true); // 队列满时覆盖
let no_write_config = TopicConfig::new(2, false); // 队列满时停止写入

// 订阅者 1：覆盖模式
let receiver1 = pubsub.subscribe_manual::<String>(topic, overwrite_config);
thread::spawn(move || {
    while let Some(msg) = receiver1.try_recv() {
        println!("覆盖订阅者：{}", msg);
    }
});

// 订阅者 2：停止写入模式
let receiver2 = pubsub.subscribe_manual::<String>(topic, no_write_config);
thread::spawn(move || {
    while let Some(msg) = receiver2.try_recv() {
        println!("停止写入订阅者：{}", msg);
    }
});

// 发布消息以填充队列（可在单独文件中）
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "消息 1".to_string());
pubsub.publish(topic_id, "消息 2".to_string());
pubsub.publish(topic_id, "消息 3".to_string()); // 仅影响覆盖订阅者

// 等待观察
thread::sleep(std::time::Duration::from_millis(100));

3. 带专用线程的回调订阅
此示例展示回调订阅，回调在内部专用线程中处理。可使用任意消息类型。
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "callback_topic";
let config = TopicConfig::new(5, true); // 队列满时覆盖

// 使用回调订阅（可在单独文件中）
pubsub.subscribe::<String, _>(topic, config, |msg: &String| {
    println!("回调：{}", msg); // 在专用线程中运行
});

// 发布消息（可在单独文件中）
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "回调消息".to_string());

// 等待观察
thread::sleep(std::time::Duration::from_millis(100));

4. 带超时的发布
此示例展示使用超时发布消息，接收在单独线程中进行。可使用任意消息类型。
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "timeout_topic";
let config = TopicConfig::new(1, false); // 队列满时停止写入

// 订阅者线程（可在单独文件中）
let receiver = pubsub.subscribe_manual::<i32>(topic, config);
thread::spawn(move || {
    if let Some(msg) = receiver.recv_timeout(Some(100)) {
        println!("接收到：{}", msg);
    }
});

// 使用超时发布（可在单独文件中）
let topic_id = pubsub.create_publisher(topic);
pubsub.publish_with_timeout(topic_id, 42, Some(100));

// 等待观察
thread::sleep(std::time::Duration::from_millis(100));

5. 队列满时的覆盖模式
此示例展示队列满时使用覆盖模式，接收在单独线程中进行。可使用任意消息类型。
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

let pubsub = PubSub::instance();
let topic = "overwrite_topic";
let config = TopicConfig::new(2, true); // 队列满时覆盖

// 订阅者线程（可在单独文件中）
let receiver = pubsub.subscribe_manual::<String>(topic, config);
thread::spawn(move || {
    while let Some(msg) = receiver.try_recv() {
        println!("接收到：{}", msg);
    }
});

// 发布消息以填充并覆盖队列（可在单独文件中）
let topic_id = pubsub.create_publisher(topic);
pubsub.publish(topic_id, "消息 1".to_string());
pubsub.publish(topic_id, "消息 2".to_string());
pubsub.publish(topic_id, "消息 3".to_string()); // 覆盖最早的消息

// 等待观察
thread::sleep(std::time::Duration::from_millis(100));

许可证
本项目采用 Apache 2.0 或 MIT 许可证，供您选择。
