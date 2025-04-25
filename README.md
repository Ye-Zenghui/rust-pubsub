# Rust PubSub

A thread-safe, in-memory publish-subscribe library for Rust, designed for efficient and flexible inter-thread communication. Each topic supports multiple publishers and subscribers, arbitrary message formats, cross-file communication, customizable queue behavior for each subscriber, and the option to process callbacks in dedicated threads. Each subscriber uses a separate crossbeam-channel, allowing different behaviors for the same topic (such as channel depth, behavior when the channel is full, and message processing method - either in a separate thread (subscribe) or in the local thread (subscribe_manual)), making it ideal for modular Rust projects.

[**中文文档**](#中文文档)

## Features

- **Thread-Safe**: Built with `crossbeam-channel` for safe message passing between threads.
- **Multi-Publisher and Multi-Subscriber**: Supports multiple publishers and subscribers for the same topic.
- **Modular Design**: Enables publishing and subscribing across files and modules via a singleton `PubSub` instance.
- **Arbitrary Message Formats**: Supports any type implementing `Send + Sync + Clone + 'static`, including custom structs.
- **Customizable Queue Behavior**: Each subscriber can choose to overwrite old messages or stop writing when the queue is full.
- **Callback Processing**: Callbacks run in dedicated threads for non-blocking operation.
- **Flexible Subscriptions**: Offers manual receiving and callback-based subscriptions.
- **Timeout Support**: Provides blocking, non-blocking, and timeout-based operations.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-pubsub = "0.1.0"
```

## Usage Examples

Below are examples demonstrating multi-publisher and multi-subscriber communication, arbitrary message formats (using a custom struct), customizable queue-full behavior, callback processing in dedicated threads, and modular design across files.

### 1. Multi-Publisher and Multi-Subscriber Across Files with Custom Struct

This example shows two publishers and two subscribers in separate files, using a custom struct `CustomMessage` as the message type. One subscriber uses a callback in a dedicated thread, highlighting modular design and arbitrary message formats.

#### `src/main.rs`

```rust
// Custom message struct
#[derive(Clone)]
pub struct CustomMessage {
    pub id: u32,
    pub content: String,
}

mod publisher;
mod subscriber;

fn main() {
    // Start two subscribers
    subscriber::manual_subscriber();
    subscriber::callback_subscriber();

    // Start two publishers
    publisher::publisher_one();
    publisher::publisher_two();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
```

#### `src/publisher.rs`

```rust
use rust_pubsub::PubSub;

use crate::CustomMessage;

pub fn publisher_one() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);

    std::thread::spawn(move || {
        loop {
            pubsub.publish(
                topic_id,
                CustomMessage {
                    id: 1,
                    content: "Message from publisher 1".to_string(),
                },
            );
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
}

pub fn publisher_two() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let topic_id = pubsub.create_publisher(topic);
    std::thread::spawn(move || {
        loop {
            pubsub.publish(
                topic_id,
                CustomMessage {
                    id: 2,
                    content: "Message from publisher 2".to_string(),
                },
            );

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
}

```

#### `src/subscriber.rs`

```rust
use rust_pubsub::{PubSub, TopicConfig};
use std::thread;

use crate::CustomMessage;

pub fn manual_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, true); // Overwrite when queue is full
    let receiver = pubsub.subscribe_manual::<CustomMessage>(topic, config);
    thread::spawn(move || {
        loop {
            if let Some(msg) = receiver.recv() {
                println!("Manual subscriber received: ID={}, Content={}", msg.id, msg.content);
            }
        }
    });
}

pub fn callback_subscriber() {
    let pubsub = PubSub::instance();
    let topic = "multi_topic";
    let config = TopicConfig::new(10, false); // Stop writing when queue is full
    pubsub.subscribe::<CustomMessage, _>(topic, config, |msg: &CustomMessage| {
        println!("Callback subscriber received: ID={}, Content={}", msg.id, msg.content); // The closure will execute in a separate internal thread
    });
}
```


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

---

# 中文文档

Rust PubSub 是一个线程安全的、基于内存的发布-订阅库，专为 Rust 设计，旨在实现高效的线程间通信。每个topic支持多个发布者和订阅者、任意消息格式、跨文件通信、每个订阅者可自定义队列满行为以及是否在专用线程中处理回调，每个订阅者使用单独的crossbeam-channel，因此每个订阅者可对同一个topci指定不同的行为(如通道深度，通道满时的行为，消息处理方式是在单独的线程中处理(subscribe)还是在本地线程处理(subscribe_manual))，非常适合模块化的 Rust 项目。

[**English Documentation**](#rust-pubsub)

## 功能

- **线程安全**：使用 `crossbeam-channel` 实现线程间安全消息传递。
- **多发布者和多订阅者**：支持同一主题的多个发布者和订阅者。
- **模块化设计**：通过单例 `PubSub` 实例，支持跨文件和模块的发布和订阅。
- **任意消息格式**：支持任何实现 `Send + Sync + Clone + 'static` 的类型，包括自定义结构体。
- **可自定义队列行为**：每个订阅者可配置队列满时覆盖旧消息或停止写入。
- **回调处理**：回调在单独线程中处理，确保非阻塞。
- **灵活订阅**：支持手动接收和基于回调的订阅。
- **超时支持**：提供阻塞、非阻塞和带超时操作。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rust-pubsub = "0.1.0"
```

## 使用示例

以下示例展示了多发布者和多订阅者通信、任意消息格式（使用自定义结构体）、可自定义队列满行为、在专用线程中处理回调以及跨文件模块化设计。

### 1. 跨文件的多发布者和多订阅者（使用自定义结构体）

此示例展示在不同文件中定义的两个发布者和两个订阅者，使用自定义结构体 `CustomMessage` 作为消息类型。一个订阅者使用回调，在内部专用线程中处理。

#### `src/main.rs`

```rust
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
```

#### `src/publisher.rs`

```rust
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
```

#### `src/subscriber.rs`

```rust
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
```


## 许可证

本项目采用 Apache 2.0 或 MIT 许可证，供您选择。