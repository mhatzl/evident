/// Macro to create a static publisher.
///
/// ## Usage
///
/// In the following example, text between `<>` is used as placeholder.\
/// The visibility setting at the beginning is also **optional**.
///
/// ```ignore
/// evident::create_static_publisher!(
///     pub <Name for the publisher>,
///     <Struct implementing `evident::publisher::Id`>,
///     <Struct implementing `evident::event::EventEntry`>,
///     <Struct implementing `evident::event::IntermediaryEvent`>,
///     CAPTURE_CHANNEL_BOUND = <`usize` literal for the channel bound used to capture events>,
///     SUBSCRIPTION_CHANNEL_BOUND = <`usize` literal for the channel bound used per subscription>,
///     non_blocking = <`bool` literal defining if event finalizing should be non-blocking (`true`), or block the thread (`false`)>
/// );
/// ```
///
/// **Example with dummy implementations:**
///
/// ```ignore
/// evident::create_static_publisher!(
///     pub MY_PUBLISHER,
///     MyId,
///     MyEventEntry,
///     MyIntermEvent,
///     CAPTURE_CHANNEL_BOUND = 100,
///     SUBSCRIPTION_CHANNEL_BOUND = 50,
///     non_blocking = true
/// );
/// ```
///
#[macro_export]
macro_rules! create_static_publisher {
    ($publisher_name:ident,
        $id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        CAPTURE_CHANNEL_BOUND = $cap_channel_bound:literal,
        SUBSCRIPTION_CHANNEL_BOUND = $sub_channel_bound:literal,
        non_blocking = $try_capture:literal
    ) => {
        $crate::z__create_static_publisher!($publisher_name,
            $id_t,
            $entry_t,
            $interm_event_t,
            $cap_channel_bound,
            $sub_channel_bound,
            $try_capture
        );
    };
    ($visibility:vis $publisher_name:ident,
        $id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        CAPTURE_CHANNEL_BOUND = $cap_channel_bound:literal,
        SUBSCRIPTION_CHANNEL_BOUND = $sub_channel_bound:literal,
        non_blocking = $try_capture:literal
    ) => {
        $crate::z__create_static_publisher!($publisher_name,
            $id_t,
            $entry_t,
            $interm_event_t,
            $cap_channel_bound,
            $sub_channel_bound,
            $try_capture
            scope=$visibility
        );
    };
}

#[macro_export]
macro_rules! z__create_static_publisher {
    ($publisher_name:ident,
        $id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        $cap_channel_bound:literal,
        $sub_channel_bound:literal,
        $try_capture:literal
        $(scope=$visibility:vis)?
    ) => {
        $($visibility)? static $publisher_name: $crate::once_cell::sync::Lazy<
            $crate::publisher::EvidentPublisher<
                $id_t,
                $entry_t,
                $cap_channel_bound,
                $sub_channel_bound,
            >,
        > = $crate::once_cell::sync::Lazy::new(|| {
            $crate::publisher::EvidentPublisher::<
                $id_t,
                $entry_t,
                $cap_channel_bound,
                $sub_channel_bound,
            >::new(|event| {
                $publisher_name.on_event(event);
            })
        });

        impl Drop for $interm_event_t {
            fn drop(&mut self) {
                if $try_capture {
                    $publisher_name.try_capture(self);
                } else {
                    $publisher_name.capture(self);
                }
            }
        }

        // Note: Re-impl `finalize()` for better IntelliSense.
        impl $interm_event_t {
            pub fn finalize(self) -> $crate::event::intermediary::FinalizedEvent<$id_t> {
                $crate::event::intermediary::IntermediaryEvent::<$id_t, $entry_t>::finalize(self)
            }
        }

        impl From<$interm_event_t> for $id_t {
            fn from(intermed_event: $interm_event_t) -> Self {
                $crate::event::intermediary::IntermediaryEvent::<$id_t, $entry_t>::finalize(intermed_event).into_event_id()
            }
        }

        impl PartialEq for $entry_t {
            fn eq(&self, other: &Self) -> bool {
                $crate::event::entry::EventEntry::<$id_t>::get_event_id(self) == $crate::event::entry::EventEntry::<$id_t>::get_event_id(other)
                && $crate::event::entry::EventEntry::<$id_t>::get_entry_id(self) == $crate::event::entry::EventEntry::<$id_t>::get_entry_id(other)
            }
        }

        impl Eq for $entry_t {}

        impl std::hash::Hash for $entry_t
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $crate::event::entry::EventEntry::<$id_t>::get_entry_id(self).hash(state);
            }
        }
    };
}

/// Macro to create the `set_event!()` macro for a concrete implementation.
///
/// ## Usage
///
/// In the following example, text between `<>` is used as placeholder.
/// `no_export` may be set at the beginning to prevent `#[macro_export]` from being added.
///
/// Note: Set fully qualified paths to make the macro accessible from anywhere.
///
/// ```ignore
/// evident::create_set_event_macro!(
///     <Struct implementing `evident::publisher::Id`>,
///     <Struct implementing `evident::event::EventEntry`>,
///     <Struct implementing `evident::event::IntermediaryEvent`>
/// );
/// ```
///
/// **Example with dummy implementations:**
///
/// ```ignore
/// evident::create_set_event_macro!(
///     my_crate::my_mod::MyId,
///     my_crate::my_mod::MyEventEntry,
///     my_crate::my_mod::MyInterimEvent
/// );
/// ```
///     
#[macro_export]
macro_rules! create_set_event_macro {
    ($id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty
    ) => {
        #[macro_export]
        macro_rules! set_event {
            ($id:expr) => {
                $crate::event::set_event::<$id_t, $entry_t, $interm_event_t>(
                    $id,
                    $crate::this_origin!(),
                )
            };
            ($id:expr, $msg:expr) => {
                $crate::event::set_event_with_msg::<$id_t, $entry_t, $interm_event_t>(
                    $id,
                    $msg,
                    $crate::this_origin!(),
                )
            };
        }
    };
    (no_export
        $id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty
    ) => {
        macro_rules! set_event {
            ($id:expr) => {
                $crate::event::set_event::<$id_t, $entry_t, $interm_event_t>(
                    $id,
                    $crate::this_origin!(),
                )
            };
            ($id:expr, $msg:expr) => {
                $crate::event::set_event_with_msg::<$id_t, $entry_t, $interm_event_t>(
                    $id,
                    $msg,
                    $crate::this_origin!(),
                )
            };
        }
    };
}
