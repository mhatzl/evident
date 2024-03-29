//! Contains macros used to create a static publisher, and the `set_event!()` macro.

/// Macro to create a static publisher.
///
/// ## Usage
///
/// In the following example, text between `<>` is used as placeholder.\
/// The visibility setting at the beginning is also **optional**.
///
/// ```text
/// evident::create_static_publisher!(
///     <visibility specifier> <Name for the publisher>,
///     id_type = <Struct implementing `evident::event::Id`>,
///     msg_type = <Struct implementing `evident::event::Msg`>,
///     entry_type = <Struct implementing `evident::event::entry::EventEntry`>,
///     interm_event_type = <Struct implementing `evident::event::intermediary::IntermediaryEvent`>,
///     filter_type = <Optional Struct implementing `evident::event::filter::Filter`>,
///     filter = <Optional instance of the filter. Must be set if filter type is set>,
///     capture_channel_bound = <`usize` expression for the channel bound used to capture events>,
///     subscription_channel_bound = <`usize` expression for the channel bound used per subscription>,
///     capture_mode = <`evident::publisher::CaptureMode` defining if event finalizing should be non-blocking (`NonBlocking`), or block the thread (`Blocking`)>,
///     timestamp_kind = <`evident::publisher::EventTimestampKind` defining if event timestamp should be set on creation (`Created`), or on capture (`Captured`)>
/// );
/// ```
///
/// **Example without filter:**
///
/// ```text
/// evident::create_static_publisher!(
///     pub MY_PUBLISHER,
///     id_type = MyId,
///     msg_type = String,
///     entry_type = MyEventEntry,
///     interm_event_type = MyIntermEvent,
///     capture_channel_bound = 100,
///     subscription_channel_bound = 50,
///     capture_mode = CaptureMode::Blocking,
///     timestamp_kind = EventTimestampKind::Captured
/// );
/// ```
///
/// **Example with filter:**
///
/// ```text
/// evident::create_static_publisher!(
///     pub MY_PUBLISHER,
///     id_type = MyId,
///     msg_type = String,
///     entry_type = MyEventEntry,
///     interm_event_type = MyIntermEvent,
///     filter_type = MyFilter,
///     filter = MyFilter::default(),
///     capture_channel_bound = 100,
///     subscription_channel_bound = 50,
///     capture_mode = CaptureMode::NonBlocking,
///     timestamp_kind = EventTimestampKind::Captured
/// );
/// ```
///
/// [req:qa.ux.macros]
#[macro_export]
macro_rules! create_static_publisher {
    ($publisher_name:ident,
        id_type = $id_t:ty,
        msg_type = $msg_t:ty,
        entry_type = $entry_t:ty,
        interm_event_type = $interm_event_t:ty,
        $(filter_type=$filter_t:ty,)?
        $(filter=$filter:expr,)?
        capture_channel_bound = $cap_channel_bound:expr,
        subscription_channel_bound = $sub_channel_bound:expr,
        capture_mode = $capture_mode:expr,
        timestamp_kind = $timestamp_kind:expr
    ) => {
        $crate::z__setup_static_publisher!(
            $publisher_name,
            $id_t,
            $msg_t,
            $entry_t,
            $interm_event_t,
            $cap_channel_bound,
            $sub_channel_bound,
            $capture_mode,
            $timestamp_kind
            $(, filter_type=$filter_t)?
            $(, filter=$filter)?
        );
    };
    ($visibility:vis $publisher_name:ident,
        id_type = $id_t:ty,
        msg_type = $msg_t:ty,
        entry_type = $entry_t:ty,
        interm_event_type = $interm_event_t:ty,
        $(filter_type=$filter_t:ty,)?
        $(filter=$filter:expr,)?
        capture_channel_bound = $cap_channel_bound:expr,
        subscription_channel_bound = $sub_channel_bound:expr,
        capture_mode = $capture_mode:expr,
        timestamp_kind = $timestamp_kind:expr
    ) => {
        $crate::z__setup_static_publisher!(
            $publisher_name,
            $id_t,
            $msg_t,
            $entry_t,
            $interm_event_t,
            $cap_channel_bound,
            $sub_channel_bound,
            $capture_mode,
            $timestamp_kind,
            scope = $visibility
            $(, filter_type=$filter_t)?
            $(, filter=$filter)?
        );
    };
}

/// Internal macro to set up a static publisher.
///
/// **Note:** Use [`create_static_publisher`](crate::create_static_publisher) instead.
#[doc(hidden)]
#[macro_export]
macro_rules! z__setup_static_publisher {
    ($publisher_name:ident,
        $id_t:ty,
        $msg_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        $cap_channel_bound:expr,
        $sub_channel_bound:expr,
        $capture_mode:expr,
        $timestamp_kind:expr
        $(, scope=$visibility:vis)?
        $(, filter_type=$filter_t:ty)?
        $(, filter=$filter:expr)?
    ) => {

        $crate::z__create_static_publisher!(
            $publisher_name,
            $id_t,
            $msg_t,
            $entry_t,
            $interm_event_t,
            $(filter_type=$filter_t,)?
            $(filter=$filter,)?
            $cap_channel_bound,
            $sub_channel_bound,
            $capture_mode,
            $timestamp_kind
            $(, scope=$visibility)?
        );

        impl Drop for $interm_event_t {
            fn drop(&mut self) {
                $publisher_name._capture(self);
            }
        }

        // Note: Re-impl `finalize()` for better IntelliSense.
        impl $interm_event_t {
            pub fn finalize(self) -> $crate::event::finalized::FinalizedEvent<$id_t> {
                $crate::event::intermediary::IntermediaryEvent::<$id_t, $msg_t, $entry_t>::finalize(self)
            }
        }

        impl From<$interm_event_t> for $id_t {
            fn from(intermed_event: $interm_event_t) -> Self {
                $crate::event::intermediary::IntermediaryEvent::<$id_t, $msg_t, $entry_t>::finalize(intermed_event).into_event_id()
            }
        }

        impl PartialEq for $entry_t {
            fn eq(&self, other: &Self) -> bool {
                $crate::event::entry::EventEntry::<$id_t, $msg_t>::get_event_id(self) == $crate::event::entry::EventEntry::<$id_t, $msg_t>::get_event_id(other)
                && $crate::event::entry::EventEntry::<$id_t, $msg_t>::get_entry_id(self) == $crate::event::entry::EventEntry::<$id_t, $msg_t>::get_entry_id(other)
            }
        }

        impl Eq for $entry_t {}

        impl std::hash::Hash for $entry_t
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $crate::event::entry::EventEntry::<$id_t, $msg_t>::get_entry_id(self).hash(state);
            }
        }
    };
}

/// Internal macro to create a static publisher.
///
/// **Note:** Use [`create_static_publisher`](crate::create_static_publisher) instead.
#[doc(hidden)]
#[macro_export]
macro_rules! z__create_static_publisher {
    ($publisher_name:ident,
        $id_t:ty,
        $msg_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        filter_type=$filter_t:ty,
        filter=$filter:expr,
        $cap_channel_bound:expr,
        $sub_channel_bound:expr,
        $capture_mode:expr,
        $timestamp_kind:expr
        $(, scope=$visibility:vis)?
    ) => {
        $($visibility)? static $publisher_name: $crate::once_cell::sync::Lazy<
            $crate::publisher::EvidentPublisher<$id_t, $msg_t, $entry_t, $filter_t>,
        > = $crate::once_cell::sync::Lazy::new(|| {
            $crate::publisher::EvidentPublisher::<
                $id_t,
                $msg_t,
                $entry_t,
                $filter_t
            >::with(|event| {
                $publisher_name.on_event(event);
            }, $filter, $capture_mode, $cap_channel_bound, $sub_channel_bound, $timestamp_kind)
        });
    };
    ($publisher_name:ident,
        $id_t:ty,
        $msg_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        $cap_channel_bound:expr,
        $sub_channel_bound:expr,
        $capture_mode:expr,
        $timestamp_kind:expr
        $(, scope=$visibility:vis)?
    ) => {
        type DummyFilter = $crate::event::filter::DummyFilter<$id_t, $msg_t>;

        $($visibility)? static $publisher_name: $crate::once_cell::sync::Lazy<
            $crate::publisher::EvidentPublisher<$id_t, $msg_t, $entry_t, DummyFilter>,
        > = $crate::once_cell::sync::Lazy::new(|| {
            $crate::publisher::EvidentPublisher::<
                $id_t,
                $msg_t,
                $entry_t,
                DummyFilter
            >::new(|event| {
                $publisher_name.on_event(event);
            }, $capture_mode, $cap_channel_bound, $sub_channel_bound, $timestamp_kind)
        });
    }
}

/// Macro to create the `set_event!()` macro for a concrete implementation.
///
/// ## Usage
///
/// In the following example, text between `<>` is used as placeholder.
/// `no_export,` may be set at the beginning to prevent `#[macro_export]` from being added.
///
/// Note: Set fully qualified paths for the types to make the macro accessible from anywhere.
///
/// ```text
/// evident::create_set_event_macro!(
///     id_type = <Struct implementing `evident::publisher::Id`>,
///     entry_type = <Struct implementing `evident::event::EventEntry`>,
///     interm_event_type = <Struct implementing `evident::event::IntermediaryEvent`>
/// );
/// ```
///
/// **Example with dummy implementations:**
///
/// ```text
/// evident::create_set_event_macro!(
///     id_type = my_crate::my_mod::MyId,
///     entry_type = my_crate::my_mod::MyEventEntry,
///     interm_event_type = my_crate::my_mod::MyInterimEvent
/// );
/// ```
///
/// **Example with no export:**
///
/// ```text
/// evident::create_set_event_macro!(
///     no_export,
///     id_type = my_crate::my_mod::MyId,
///     entry_type = my_crate::my_mod::MyEventEntry,
///     interm_event_type = my_crate::my_mod::MyInterimEvent
/// );
/// ```
///
/// [req:qa.ux.macros]
#[macro_export]
macro_rules! create_set_event_macro {
    (id_type = $id_t:ty,
        msg_type = $msg_t:ty,
        entry_type = $entry_t:ty,
        interm_event_type = $interm_event_t:ty
    ) => {
        /// Macro to set an event.
        ///
        /// **Variants:**
        ///
        /// - `set_event!(id)` ... Set an event for the given event-ID without a message
        /// - `set_event!(id, msg)` ... Set an event for the given event-ID with the given message
        ///
        /// **Examples:**
        ///
        /// ```ignore
        /// let id = YourIdType { ... };
        ///
        /// set_event!(id).finalize();
        /// ```
        ///
        /// ```ignore
        /// let id = YourIdType { ... };
        /// let msg = "Your event message.";
        ///
        /// set_event!(id, msg).finalize();
        /// ```
        ///
        /// [req:event.set], [req:qa.ux.macros]
        #[macro_export]
        macro_rules! set_event {
            ($id:expr) => {
                $crate::event::set_event::<$id_t, $msg_t, $entry_t, $interm_event_t>(
                    $id,
                    $crate::this_origin!(),
                )
            };
            ($id:expr, $msg:expr) => {
                $crate::event::set_event_with_msg::<$id_t, $msg_t, $entry_t, $interm_event_t>(
                    $id,
                    $msg,
                    $crate::this_origin!(),
                )
            };
        }
    };
    (no_export,
        id_type = $id_t:ty,
        msg_type = $msg_t:ty,
        entry_type = $entry_t:ty,
        interm_event_type = $interm_event_t:ty
    ) => {
        /// Macro to set an event.
        ///
        /// **Variants:**
        ///
        /// - `set_event!(id)` ... Set an event for the given event-ID without a message
        /// - `set_event!(id, msg)` ... Set an event for the given event-ID with the given message
        ///
        /// **Examples:**
        ///
        /// ```ignore
        /// let id = YourIdType { ... };
        ///
        /// set_event!(id).finalize();
        /// ```
        ///
        /// ```ignore
        /// let id = YourIdType { ... };
        /// let msg = "Your event message.";
        ///
        /// set_event!(id, msg).finalize();
        /// ```
        ///
        /// [req:event.set], [req:qa.ux.macros]
        macro_rules! set_event {
            ($id:expr) => {
                $crate::event::set_event::<$id_t, $msg_t, $entry_t, $interm_event_t>(
                    $id,
                    $crate::this_origin!(),
                )
            };
            ($id:expr, $msg:expr) => {
                $crate::event::set_event_with_msg::<$id_t, $msg_t, $entry_t, $interm_event_t>(
                    $id,
                    $msg,
                    $crate::this_origin!(),
                )
            };
        }
    };
}
