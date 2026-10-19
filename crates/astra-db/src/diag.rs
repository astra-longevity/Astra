//! Validation diagnostics: errors and warnings in the format of `BUILD_SPEC.md` §4.

/// One validation message, rendered on a single stderr line.
#[derive(Clone, Debug)]
pub struct Diag {
    /// `true` for a warning, `false` for an error.
    pub warning: bool,
    /// Rule code, e.g. `E020`.
    pub code: &'static str,
    /// File path relative to the repository root.
    pub file: String,
    /// Optional field the message is about.
    pub field: Option<String>,
    /// Message in English.
    pub msg: String,
}

impl Diag {
    /// Renders the diagnostic as `<level> <code> <file>[:<field>]: <message>`.
    #[must_use]
    pub fn render(&self) -> String {
        let level = if self.warning { "warning" } else { "error" };
        self.field.as_ref().map_or_else(
            || format!("{level} {} {}: {}", self.code, self.file, self.msg),
            |field| format!("{level} {} {}:{field}: {}", self.code, self.file, self.msg),
        )
    }
}

/// Collected diagnostics of a whole build.
#[derive(Debug, Default)]
pub struct Diags {
    /// Diagnostics in emission order.
    pub items: Vec<Diag>,
}

impl Diags {
    /// Records an error.
    pub fn error(&mut self, code: &'static str, file: &str, msg: String) {
        self.items.push(Diag { warning: false, code, file: file.to_owned(), field: None, msg });
    }

    /// Records a warning.
    pub fn warn(&mut self, code: &'static str, file: &str, msg: String) {
        self.items.push(Diag { warning: true, code, file: file.to_owned(), field: None, msg });
    }

    /// Number of errors.
    #[must_use]
    pub fn errors(&self) -> usize {
        self.items.iter().filter(|diag| !diag.warning).count()
    }

    /// Number of warnings.
    #[must_use]
    pub fn warnings(&self) -> usize {
        self.items.iter().filter(|diag| diag.warning).count()
    }
}
