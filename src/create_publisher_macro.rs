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
        $crate::__create_static_publisher!($publisher_name,
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
        $crate::__create_static_publisher!($publisher_name,
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
macro_rules! __create_static_publisher {
    ($publisher_name:ident,
        $id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty,
        $cap_channel_bound:literal,
        $sub_channel_bound:literal,
        $try_capture:literal
        $(scope=$visibility:vis)?
    ) => {
        $($visibility)? static $publisher_name: once_cell::sync::Lazy<
            $crate::publisher::EvidentPublisher<
                $id_t,
                $entry_t,
                $cap_channel_bound,
                $sub_channel_bound,
            >,
        > = once_cell::sync::Lazy::new(|| {
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

        impl $interm_event_t {
            pub fn finalize(self) -> $id_t {
                let id = $crate::event::entry::EventEntry::<$id_t>::get_event_id(
                    $crate::event::intermediary::IntermediaryEvent::<$id_t, $entry_t>::get_entry(
                        &self,
                    ),
                )
                .clone();
                drop(self);
                id
            }
        }

        impl From<$interm_event_t> for $id_t {
            fn from(intermed_event: $interm_event_t) -> Self {
                intermed_event.finalize()
            }
        }
    };
}

#[macro_export]
macro_rules! create_set_event_macro {
    ($id_t:ty,
        $entry_t:ty,
        $interm_event_t:ty
    ) => {
        #[macro_export]
        macro_rules! set_event {
            ($id:expr, $msg:expr) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interm_event_t>::set_event(
                    std::convert::Into::<$id_t>::into($id),
                    ($msg).into(),
                    env!("CARGO_PKG_NAME"),
                    file!(),
                    line!(),
                    module_path!(),
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
            ($id:expr, $msg:expr) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interm_event_t>::set_event(
                    std::convert::Into::<$id_t>::into($id),
                    ($msg).into(),
                    env!("CARGO_PKG_NAME"),
                    file!(),
                    line!(),
                    module_path!(),
                )
            };
        }
    };
}
