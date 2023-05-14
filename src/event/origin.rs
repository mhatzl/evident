/// Structure representing the origin of an event.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Origin {
    pub crate_name: &'static str,

    /// Module path where the event was set
    pub module_path: &'static str,

    /// Filename where the event was set
    pub filename: &'static str,

    /// Line number where the event was set
    pub line_nr: u32,
}

impl Origin {
    /// Create a new [`Origin`].
    pub fn new(
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> Self {
        Origin {
            crate_name,
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
            "crate=\"{}\", module=\"{}\", file=\"{}\", line={}",
            origin.crate_name, origin.module_path, origin.filename, origin.line_nr
        )
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
