/// Structure representing the origin of an event.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Origin {
    /// Module path where the event was set.
    ///
    /// Note: Use `module_path!()`.
    pub module_path: &'static str,

    /// Filename where the event was set.
    ///
    /// Note: Use `file!()`.
    pub filename: &'static str,

    /// Line number where the event was set.
    ///
    /// Note: Use `line!()`.
    pub line_nr: u32,
}

impl Origin {
    /// Create a new [`Origin`].
    pub fn new(module_path: &'static str, filename: &'static str, line_nr: u32) -> Self {
        Origin {
            module_path,
            filename,
            line_nr,
        }
    }
}

impl From<&Origin> for String {
    /// Outputs given [`Origin`] as `crate="<crate name>", module="<module path>", file="<filename>", line=<line number>`.
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

#[macro_export]
macro_rules! this_origin {
    () => {
        $crate::event::origin::Origin::new(module_path!(), file!(), line!())
    };
}
