//! `astra` command-line entry point.

use std::io::{self, Write as _};

/// Version string reported by `astra`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builds the line printed on startup.
fn version_line() -> String {
    format!("astra {VERSION}")
}

/// Prints the version and exits.
fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    writeln!(stdout, "{}", version_line())
}

#[cfg(test)]
mod tests {
    use super::version_line;

    #[test]
    fn version_line_starts_with_name() {
        assert!(version_line().starts_with("astra "));
    }
}
