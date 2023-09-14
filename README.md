# evident

![build-test](https://github.com/mhatzl/evident/actions/workflows/rust.yml/badge.svg?branch=main)
![mantra-sync](https://github.com/mhatzl/evident/actions/workflows/mantra_sync.yml/badge.svg?branch=main)
[![crates.io](https://img.shields.io/crates/v/evident)](https://crates.io/crates/evident)

This crate makes it easy to create custom multithreaded pub/sub functionality for Rust applications.
It uses **events** to send information between publisher and subscriber, and **IDs** are used to subscribe and identify these **events** (hence *ev**ID**ent*).

The pub/sub communication is done over a central static *publisher* that captures set events,
and forwards them to subscribers. To allow using *evident* in different scenarios,
a publisher must be created per scenario, and cannot be provided by *evident* itself.
See the [setup](#setup) section below on how to create your custom pub/sub instance.

## Setup

**To customize *evident* to fit your use case, you need to implement the following traits:**

- [`Id`](https://docs.rs/evident/latest/evident/event/trait.Id.html) ... Defines the structure of the ID that is used to identify events
- [`EventEntry`](https://docs.rs/evident/latest/evident/event/entry/trait.EventEntry.html) ... Allows adding additional information to an event
- [`IntermediaryEvent`](https://docs.rs/evident/latest/evident/event/intermediary/trait.IntermediaryEvent.html) ... Allows automatic capturing of events once they go out of scope

**Optional traits to further customize *evident*:**

- [`Filter`](https://docs.rs/evident/latest/evident/event/filter/trait.Filter.html) ... To prevent capturing events
- [`Msg`](https://docs.rs/evident/latest/evident/event/trait.Msg.html) ... Allows creating a custom message to be sent with an event

**Creating your pub/sub instance:**

- [`create_static_publisher!()`](https://docs.rs/evident/latest/evident/macro.create_static_publisher.html) ... Convenience macro to create your custom [`EvidentPublisher`](https://docs.rs/evident/latest/evident/publisher/struct.EvidentPublisher.html)
- [`create_set_event_macro!()`](https://docs.rs/evident/latest/evident/macro.create_set_event_macro.html) ... Convenience macro to create the `set_event!()` macro that may be used to set your custom events

**Examples:**

- [/tests/min_concretise](https://github.com/mhatzl/evident/tree/main/tests/min_concretise) ... Contains a minimal pub/sub setup
- [/tests/min_filter](https://github.com/mhatzl/evident/tree/main/tests/min_filter) ... Contains a minimal pub/sub setup using a custom filter
- [/tests/min_msg](https://github.com/mhatzl/evident/tree/main/tests/min_msg) ... Contains a minimal pub/sub setup with a custom message

## Usage

After creating your own publisher, you can set events using the `set_event!()` macro.

**Example:**

```rust
let some_id = MinId { id: 3 };
let msg = "Some msg";

let sub = PUBLISHER.subscribe(some_id).unwrap();

set_event!(some_id, msg).finalize();

let event = sub
    .get_receiver()
    .recv_timeout(std::time::Duration::from_millis(100))
    .unwrap();
```

**Note:** `finalize()` is set explicitly to ensure the event is sent before the subscription tries to receive it.
Otherwise, it would be sent once the event gets out of scope (is dropped).

# License

MIT Licensed
