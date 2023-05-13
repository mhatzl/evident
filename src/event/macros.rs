
#[macro_export]
macro_rules! create_set_event {
    ($id_t:ty, $entry_t:ty, $interim_event_t:ty) => {
        /// Macro to set an event for the given [`Id`].
        /// The environment variable `CARGO_PKG_NAME` set by cargo is used as crate name.
        ///
        /// **Arguments:**
        ///
        /// * `id` ... Must be a valid `Id`
        /// * `msg` ... `String` variable or literal of the main message for the event
        #[macro_export]
        macro_rules! set_event {
            ($id:ident, $msg:ident) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interim_event_t>::set_event(
                    $id,
                    env!("CARGO_PKG_NAME"),
                    $msg,
                    file!(),
                    line!(),
                    module_path!(),
                )
            };
            ($id:ident, $msg:literal) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interim_event_t>::set_event(
                    $id,
                    env!("CARGO_PKG_NAME"),
                    $msg,
                    file!(),
                    line!(),
                    module_path!(),
                )
            };
            ($id:ident, $msg:expr) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interim_event_t>::set_event(
                    $id,
                    env!("CARGO_PKG_NAME"),
                    $msg,
                    file!(),
                    line!(),
                    module_path!(),
                )
            };
            ($id:expr, $msg:expr) => {
                $crate::event::EventFns::<$id_t, $entry_t, $interim_event_t>::set_event(
                    $id,
                    env!("CARGO_PKG_NAME"),
                    $msg,
                    file!(),
                    line!(),
                    module_path!(),
                )
            };
        }
    };
}
