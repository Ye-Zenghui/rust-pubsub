//! # Rust PubSub
//!
//! A thread-safe, in-memory publish-subscribe library for Rust, designed for efficient and flexible
//! inter-thread communication. It supports both manual message receiving and callback-based subscriptions,
//! with configurable queue depth and overwrite behavior.
//!
//! ## Features
//!
//! - **Thread-Safe**: Uses `crossbeam-channel` for safe message passing between threads.
//! - **Flexible Subscriptions**: Supports manual message receiving and callback-based subscriptions.
//! - **Configurable Channels**: Allows customization of queue depth and overwrite behavior per subscription.
//! - **Type-Safe Messaging**: Supports any `Send + Sync + Clone + 'static` type for messages.
//! - **Timeout Support**: Provides blocking, non-blocking, and timeout-based message receiving and publishing.
//!
//! ## 功能（中文）
//!
//! - **线程安全**：使用 `crossbeam-channel` 实现线程间安全消息传递。
//! - **灵活订阅**：支持手动消息接收和基于回调的订阅模式。
//! - **可配置通道**：允许为每个订阅配置队列深度和覆盖行为。
//! - **类型安全消息**：支持任何满足 `Send + Sync + Clone + 'static` 的消息类型。
//! - **超时支持**：提供阻塞、非阻塞和带超时的消息接收与发布。
//!
//! ## API Overview
//!
//! The library offers two main subscription methods:
//!
//! - **`subscribe_manual`**: Returns a receiver that allows manually polling for messages.
//! - **`subscribe`**: Takes a callback closure that processes messages in a dedicated thread.
//!   This non-blocking approach runs the provided closure in its own thread, making it ideal
//!   for operations that shouldn't block the main thread.
//!
//! ## Usage Examples
//!
//! ### 1. Manual Subscription with Non-Blocking Receive
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//!
//! let pubsub = PubSub::instance();
//! let topic = "test_topic";
//! let config = TopicConfig::new(10, false); // Queue depth 10, no overwrite
//!
//! // Create publisher
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe manually
//! let receiver = pubsub.subscribe_manual::<String>(topic, config);
//!
//! // Publish a message
//! pubsub.publish(topic_id, "Hello, World!".to_string());
//!
//! // Try to receive the message
//! if let Some(msg) = receiver.try_recv() {
//!     println!("Received: {}", msg);
//! }
//! ```
//!
//! ### 2. Callback-Based Subscription
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//!
//! let pubsub = PubSub::instance();
//! let topic = "callback_topic";
//! let config = TopicConfig::new(5, true); // Queue depth 5, overwrite enabled
//!
//! // Create publisher
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe with a callback
//! let subscriber_id = pubsub.subscribe::<String, _>(topic, config, |msg: &String| {
//!     println!("Callback received: {}", msg);
//! });
//!
//! // Publish a message
//! pubsub.publish(topic_id, "Callback message".to_string());
//!
//! // Wait briefly to ensure callback executes
//! std::thread::sleep(std::time::Duration::from_millis(100));
//!
//! // Unsubscribe
//! pubsub.unsubscribe(&subscriber_id);
//! ```
//!
//! ### 3. Non-Blocking Callback Processing in Dedicated Thread
//!
//! This example demonstrates how the `subscribe` method's callback is processed in its own dedicated thread,
//! allowing your main thread to continue execution without being blocked by message processing.
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//! use std::sync::{Arc, Mutex};
//! use std::thread;
//! use std::time::Duration;
//!
//! // Create a shared counter to demonstrate the callback running in a separate thread
//! let counter = Arc::new(Mutex::new(0));
//! let counter_clone = counter.clone();
//!
//! let pubsub = PubSub::instance();
//! let topic = "thread_topic";
//! let config = TopicConfig::new(5, true);
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe with a callback that will increment the counter
//! pubsub.subscribe::<i32, _>(topic, config, move |msg: &i32| {
//!     // This closure runs in a dedicated thread
//!     println!("Processing message: {} in a dedicated thread", msg);
//!     
//!     // Simulate some processing time
//!     thread::sleep(Duration::from_millis(500));
//!     
//!     // Update the shared counter
//!     let mut count = counter_clone.lock().unwrap();
//!     *count += 1;
//!     println!("Counter updated to: {}", *count);
//! });
//!
//! // Publish multiple messages
//! for i in 1..=5 {
//!     pubsub.publish(topic_id, i);
//!     
//!     // Main thread can continue doing work without waiting for message processing
//!     println!("Main thread: Published message {}", i);
//! }
//!
//! // Main thread can do other work while callbacks are processed in background
//! println!("Main thread: Continuing with other work immediately");
//!
//! // Wait briefly to allow some callback processing to occur
//! thread::sleep(Duration::from_millis(1000));
//!
//! // Check how many messages were processed
//! let final_count = *counter.lock().unwrap();
//! println!("Messages processed so far: {}", final_count);
//! ```
//!
//! ### 4. Publishing with Timeout
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//!
//! let pubsub = PubSub::instance();
//! let topic = "timeout_topic";
//! let config = TopicConfig::new(1, false); // Queue depth 1, no overwrite
//!
//! // Create publisher
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe manually
//! let receiver = pubsub.subscribe_manual::<i32>(topic, config);
//!
//! // Publish with timeout (100ms)
//! pubsub.publish_with_timeout(topic_id, 42, Some(100));
//!
//! // Receive with timeout (100ms)
//! if let Some(msg) = receiver.recv_timeout(Some(100)) {
//!     println!("Received: {}", msg);
//! }
//! ```
//!
//! ### 5. Overwrite Mode with Full Queue
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//!
//! let pubsub = PubSub::instance();
//! let topic = "overwrite_topic";
//! let config = TopicConfig::new(2, true); // Queue depth 2, overwrite enabled
//!
//! // Create publisher
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe manually
//! let receiver = pubsub.subscribe_manual::<String>( topic, config);
//!
//! // Publish multiple messages to fill queue
//! pubsub.publish(topic_id, "Message 1".to_string());
//! pubsub.publish(topic_id, "Message 2".to_string());
//! pubsub.publish(topic_id, "Message 3".to_string()); // Overwrites oldest
//!
//! // Receive messages
//! while let Some(msg) = receiver.try_recv() {
//!     println!("Received: {}", msg);
//! }
//! ```
//!
//! ### 6. Multiple Subscribers
//!
//! ```rust
//! use rust_pubsub::{PubSub, TopicConfig};
//!
//! let pubsub = PubSub::instance();
//! let topic = "multi_subscriber_topic";
//! let config = TopicConfig::new(10, false);
//!
//! // Create publisher
//! let topic_id = pubsub.create_publisher(topic);
//!
//! // Subscribe multiple times
//! let receiver1 = pubsub.subscribe_manual::<String>(topic, config.clone());
//! let receiver2 = pubsub.subscribe_manual::<String>(topic, config.clone());
//!
//! // Publish a message
//! pubsub.publish(topic_id, "Broadcast message".to_string());
//!
//! // Receive from both subscribers
//! println!("Receiver 1: {:?}", receiver1.try_recv());
//! println!("Receiver 2: {:?}", receiver2.try_recv());
//! ```
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rust-pubsub = "0.1.0"
//! ```
//!
//! ## 安装（中文）
//!
//! 在您的 `Cargo.toml` 中添加以下内容：
//!
//! ```toml
//! [dependencies]
//! rust-pubsub = "0.1.0"
//! ```
//!
//! ## License
//!
//! Licensed under either of Apache License, Version 2.0 or MIT license at your option.

use crossbeam_channel::{Receiver, Sender, bounded};
use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

// Your original code follows here, unchanged
lazy_static! {
    static ref PUBSUB: Arc<PubSub> = Arc::new(PubSub::new());
}

#[derive(Clone)]
pub struct TopicConfig {
    queue_depth: usize,
    overwrite: bool,
}

impl TopicConfig {
    pub fn new(queue_depth: usize, overwrite: bool) -> Self {
        TopicConfig {
            queue_depth,
            overwrite,
        }
    }
}

#[derive(Clone)]
struct MessageWrapper {
    data: Arc<dyn Any + Send + Sync>,
}

#[derive(Clone)]
struct ChannelPair {
    sender: Sender<MessageWrapper>,
    receiver: Receiver<MessageWrapper>,
    config: TopicConfig,
    subscriber_id: String,
}

impl ChannelPair {
    fn new(
        sender: Sender<MessageWrapper>,
        receiver: Receiver<MessageWrapper>,
        config: TopicConfig,
        subscriber_id: String,
    ) -> Self {
        ChannelPair {
            sender,
            receiver,
            config,
            subscriber_id,
        }
    }
}

struct TopicData {
    #[allow(dead_code)]
    name: String,
    channel_pairs: Vec<ChannelPair>,
}

struct SubscriberData {
    topic: String,
    #[allow(dead_code)]
    receiver: Receiver<MessageWrapper>,
    #[allow(dead_code)]
    callback: Option<Arc<dyn Fn(&dyn Any) + Send + Sync>>,
}

#[derive(Clone)]
pub struct ManualReceiver<T: 'static> {
    receiver: Receiver<MessageWrapper>,
    subscriber_id: String,
    pubsub: Arc<PubSub>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> ManualReceiver<T> {
    pub fn try_recv(&self) -> Option<T> {
        let msg = self.receiver.try_recv().ok();

        match msg {
            Some(msg) => {
                if let Some(data) = msg.downcast::<T>() {
                    return Some(data.to_owned());
                }
                None
            }
            None => None,
        }
    }

    pub fn recv(&self) -> Option<T> {
        self.recv_timeout(None)
    }

    pub fn recv_timeout(&self, timeout_ms: Option<u64>) -> Option<T> {
        let msg = match timeout_ms {
            Some(ms) => self.receiver.recv_timeout(Duration::from_millis(ms)).ok(),
            None => self.receiver.recv().ok(),
        };

        match msg {
            Some(msg) => {
                if let Some(data) = msg.downcast::<T>() {
                    return Some(data.to_owned());
                }
                None
            }
            None => None,
        }
    }

    pub fn unsubscribe(self) {
        self.pubsub.unsubscribe(&self.subscriber_id);
    }
}

impl MessageWrapper {
    fn new<T: Send + Sync + Clone + 'static>(data: T) -> Self {
        MessageWrapper {
            data: Arc::new(data),
        }
    }

    fn downcast<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

pub struct PubSub {
    topics: Mutex<Vec<TopicData>>,
    topic_map: Mutex<HashMap<String, usize>>,
    subscribers: Mutex<HashMap<String, SubscriberData>>,
}

impl PubSub {
    fn new() -> Self {
        PubSub {
            topics: Mutex::new(Vec::new()),
            topic_map: Mutex::new(HashMap::new()),
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    pub fn instance() -> Arc<PubSub> {
        PUBSUB.clone()
    }

    pub fn create_publisher(&self, topic: &str) -> usize {
        let mut topic_map = self.topic_map.lock().unwrap();

        if let Some(&index) = topic_map.get(topic) {
            return index;
        }

        let mut topics = self.topics.lock().unwrap();
        let new_index = topics.len();

        topics.push(TopicData {
            name: topic.to_string(),
            channel_pairs: Vec::new(),
        });

        topic_map.insert(topic.to_string(), new_index);

        new_index
    }

    pub fn subscribe_manual<T: Send + Sync + Clone + 'static>(
        &self,
        topic: &str,
        config: TopicConfig,
    ) -> ManualReceiver<T>
    where
        T: 'static,
    {
        let subscriber_id = Uuid::new_v4().to_string();
        let (tx, rx) = bounded(config.queue_depth);
        let topic_str = topic.to_string();

        let topic_index = self.create_publisher(topic);

        {
            let mut topics = self.topics.lock().unwrap();
            topics[topic_index].channel_pairs.push(ChannelPair::new(
                tx,
                rx.clone(),
                config.clone(),
                subscriber_id.clone(),
            ));
        }

        {
            self.subscribers.lock().unwrap().insert(
                subscriber_id.clone(),
                SubscriberData {
                    topic: topic_str.clone(),
                    receiver: rx.clone(),
                    callback: None,
                },
            );
        }

        ManualReceiver {
            receiver: rx,
            subscriber_id,
            pubsub: PubSub::instance(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn subscribe<T, F>(&self, topic: &str, config: TopicConfig, callback: F) -> String
    where
        T: Send + Sync + Clone + 'static,
        F: Fn(&T) + Send + Sync + 'static,
    {
        let subscriber_id = Uuid::new_v4().to_string();
        let (tx, rx) = bounded(config.queue_depth);
        let topic_str = topic.to_string();

        let topic_index = self.create_publisher(topic);

        {
            let mut topics = self.topics.lock().unwrap();
            topics[topic_index].channel_pairs.push(ChannelPair::new(
                tx,
                rx.clone(),
                config.clone(),
                subscriber_id.clone(),
            ));
        }

        let callback_wrapper: Arc<dyn Fn(&dyn Any) + Send + Sync> =
            Arc::new(move |data: &dyn Any| {
                if let Some(t) = data.downcast_ref::<T>() {
                    callback(t);
                }
            });

        {
            self.subscribers.lock().unwrap().insert(
                subscriber_id.clone(),
                SubscriberData {
                    topic: topic_str.clone(),
                    receiver: rx.clone(),
                    callback: Some(callback_wrapper.clone()),
                },
            );
        }

        let rx_clone = rx.clone();
        let callback_for_thread = callback_wrapper.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = rx_clone.recv() {
                if let Some(data) = msg.downcast::<T>() {
                    callback_for_thread(data);
                }
            }
        });

        subscriber_id
    }

    pub fn try_publish<T: Send + Sync + Clone + 'static>(&self, topic_id: usize, message: T) {
        let msg = MessageWrapper::new(message);

        let channel_pairs = {
            let topics = self.topics.lock().unwrap();

            if topic_id >= topics.len() {
                return;
            }

            if topics[topic_id].channel_pairs.is_empty() {
                return;
            }

            topics[topic_id].channel_pairs.clone()
        };

        for pair in channel_pairs.iter() {
            if pair.config.overwrite {
                while pair.sender.is_full() {
                    let _ = pair.receiver.try_recv();
                }
            }

            let _ = pair.sender.try_send(msg.clone());
        }
    }

    pub fn publish<T: Send + Sync + Clone + 'static>(&self, topic_id: usize, message: T) {
        self.publish_with_timeout(topic_id, message, None);
    }

    pub fn publish_with_timeout<T: Send + Sync + Clone + 'static>(
        &self,
        topic_id: usize,
        message: T,
        max_wait_ms: Option<u64>,
    ) {
        let msg = MessageWrapper::new(message);

        let channel_pairs = {
            let topics = self.topics.lock().unwrap();

            if topic_id >= topics.len() {
                return;
            }

            if topics[topic_id].channel_pairs.is_empty() {
                return;
            }

            topics[topic_id].channel_pairs.clone()
        };

        for pair in channel_pairs.iter() {
            if pair.config.overwrite {
                while pair.sender.is_full() {
                    let _ = pair.receiver.try_recv();
                }
                let _ = pair.sender.try_send(msg.clone());
            } else {
                match max_wait_ms {
                    Some(ms) => {
                        let _ = pair
                            .sender
                            .send_timeout(msg.clone(), Duration::from_millis(ms));
                    }
                    None => {
                        let _ = pair.sender.send(msg.clone());
                    }
                }
            }
        }
    }

    pub fn unsubscribe(&self, subscriber_id: &str) {
        let topic_opt = {
            let mut subscribers = self.subscribers.lock().unwrap();
            if let Some(data) = subscribers.remove(subscriber_id) {
                Some(data.topic)
            } else {
                None
            }
        };

        if let Some(topic) = topic_opt {
            let topic_index_opt = {
                let topic_map = self.topic_map.lock().unwrap();
                topic_map.get(&topic).cloned()
            };

            if let Some(topic_index) = topic_index_opt {
                let mut topics = self.topics.lock().unwrap();
                if let Some(topic_data) = topics.get_mut(topic_index) {
                    topic_data
                        .channel_pairs
                        .retain(|pair| pair.subscriber_id != subscriber_id);

                    if topic_data.channel_pairs.is_empty() {
                        let mut topic_map = self.topic_map.lock().unwrap();
                        topic_map.remove(&topic);
                    }
                }
            }
        }
    }
}
