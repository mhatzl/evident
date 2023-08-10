//! Contains the [`Origin`] structure used to know where the event was set.

/// Structure to point to a location in the program code.
/// It is used to know where the event was set, but may be used for other use cases aswell.
///
/// [<req>event.origin]
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Origin {
    /// Module path to the code location.
    ///
    /// Note: Use `module_path!()`.
    pub module_path: &'static str,

    /// Filename where the code is located.
    ///
    /// Note: Use `file!()`.
    pub filename: &'static str,

    /// Line number where the code is located.
    ///
    /// Note: Use `line!()`.
    pub line_nr: u32,
}

impl Origin {
    /// Create a new [`Origin`].
    ///
    /// # Arguments
    ///
    /// * `module_path` ... Module path to the code location
    /// * `filename` ... Filename where the code is located
    /// * `line_nr` ... Line number where the code is located
    ///
    /// [<req>event.origin]
    pub fn new(module_path: &'static str, filename: &'static str, line_nr: u32) -> Self {
        Origin {
            module_path,
            filename,
            line_nr,
        }
    }
}

impl From<&Origin> for String {
    /// Formats given [`Origin`] as `module="<module path>", file="<filename>", line=<line number>`.
    fn from(origin: &Origin) -> Self {
        format!(
            "module=\"{}\", file=\"{}\", line={}",
            origin.module_path, origin.filename, origin.line_nr
        )
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

/// Convenience wrapper to create an [`Origin`] for the code position this macro is used at.
///
/// [<req>event.origin], [<req>qa.ux.macros]
#[macro_export]
macro_rules! this_origin {
    () => {
        $crate::event::origin::Origin::new(module_path!(), file!(), line!())
    };
}
