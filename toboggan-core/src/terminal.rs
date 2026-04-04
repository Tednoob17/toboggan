use serde::{Deserialize, Serialize};

/// Configuration for an embedded terminal in a slide.
///
/// Parsed from `<!-- term: path/to/cwd -->` or `<!-- term: path/to/cwd | command -->` comments.
/// Multiple terminals per slide are supported (rendered side by side).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TerminalConfig {
    /// Working directory for the terminal session
    pub cwd: String,
    /// Theme for the terminal ("dark" or "light"), defaults to "dark"
    #[serde(default = "default_theme", skip_serializing_if = "is_dark")]
    pub theme: String,
    /// Optional command to run instead of the default shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<String>,
}

fn default_theme() -> String {
    "dark".to_string()
}

fn is_dark(theme: &str) -> bool {
    theme == "dark"
}

impl TerminalConfig {
    #[must_use]
    pub fn new(cwd: impl Into<String>) -> Self {
        Self {
            cwd: cwd.into(),
            theme: default_theme(),
            cmd: None,
        }
    }

    #[must_use]
    pub fn with_theme(mut self, theme: impl Into<String>) -> Self {
        self.theme = theme.into();
        self
    }

    #[must_use]
    pub fn with_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.cmd = Some(cmd.into());
        self
    }
}
