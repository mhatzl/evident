//! *evident* makes it easy to create custom multithreaded pub/sub functionality.
//! Communication is achieved by capturing **events** with distinct **IDs**,
//! and forwarding those events to subscribers of the related IDs.
//!
//! The [tests/min_concretise](https://github.com/mhatzl/evident/tree/main/tests/min_concretise) folder
//! contains a minimal example on how to create your custom pub/sub setup.
//!
//! A custom filter is added in the [tests/min_filter](https://github.com/mhatzl/evident/tree/main/tests/min_filter) folder.
//! This filter may be used to prevent events from being captured.
//!
//! Checkout the [tests/min_msg](https://github.com/mhatzl/evident/tree/main/tests/min_msg) folder
//! if you want to send custom event messages instead of regular [`String`]s.

pub mod creation_macros;
pub mod event;
pub mod publisher;
pub mod subscription;

// Re-export external crates used in API
pub use once_cell;
pub use uuid;
