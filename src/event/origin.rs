/// Structure representing the origin of a log-id.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Origin {
    pub crate_name: String,

    /// Module path where the log-id was set
    pub module_path: String,

    /// Filename where the log-id was set
    pub filename: String,

    /// Linenumber where the log-id was set
    pub line_nr: u32, 
}

impl Origin {
    /// Create a new [`Origin`].
    pub fn new(crate_name: &str, module_path: &str, filename: &str, line_nr: u32) -> Self {
        Origin {
            crate_name: crate_name.to_string(),
            module_path: module_path.to_string(),
            filename: filename.to_string(),
            line_nr,
        }
    }
}

impl From<&Origin> for String {
    /// Outputs given [`Origin`] as `file="<filename>", line=<line number>, module="<module path>"`.
    fn from(origin: &Origin) -> Self {
        format!(
            "file=\"{}\", line={}, module=\"{}\"",
            origin.filename, origin.line_nr, origin.module_path
        )
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
