use std::collections::HashMap;
use std::path::Path;

use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

/// Types allowed in the required_metadata table of the config file. Used to
/// require specific metadata identifiers have specific types by "yr fmt".
#[derive(strum_macros::Display, Deserialize, Serialize, Debug)]
pub enum MetaValueType {
    String,
    Integer,
    Float,
    Bool,
}

/// Configuration structure for "yr" commands.
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    /// Format specific configuration information.
    pub fmt: FormatConfig,
}

/// Format specific configuration information.
#[derive(Deserialize, Serialize, Debug)]
pub struct FormatConfig {
    /// Rule specific formatting information.
    pub rule: Rule,
    /// Meta specific formatting information.
    pub meta: Meta,
    /// Pattern specific formatting information.
    pub patterns: Patterns,
}

/// Rule specific formatting information.
#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    /// Indent section headers (meta, strings, condition).
    pub indent_section_headers: bool,
    /// Indent section contents one level past section headers.
    pub indent_section_contents: bool,
    /// Number of spaces for indent. Set to 0 to use tabs.
    pub indent_spaces: u8,
    /// Insert a newline after the rule declaration but before the curly brace.
    pub newline_before_curly_brace: bool,
    /// Insert an empty line before section headers.
    pub empty_line_before_section_header: bool,
    /// Insert an empty line after section headers.
    pub empty_line_after_section_header: bool,
}

/// Meta specific formatting information.
#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    /// Align values to longest key.
    pub align_values: bool,
    /// Map of required metadata identifiers and their expected type.
    pub required: HashMap<String, MetaValueType>,
}

/// Pattern specific formatting information.
#[derive(Deserialize, Serialize, Debug)]
pub struct Patterns {
    /// Align patterns to the longest name.
    pub align_values: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            fmt: FormatConfig {
                rule: Rule {
                    indent_section_headers: true,
                    indent_section_contents: true,
                    indent_spaces: 2,
                    newline_before_curly_brace: false,
                    empty_line_before_section_header: true,
                    empty_line_after_section_header: false,
                },
                meta: Meta { align_values: true, required: HashMap::new() },
                patterns: Patterns { align_values: true },
            },
        }
    }
}

/// Load config file from a given path. Path must contain a valid TOML file or
/// this function will propagate the error. For structure of the config file
/// see "YARA-X Config Guide.md".
pub fn load_config_from_file(
    config_file: &Path,
) -> Result<Config, figment::Error> {
    let config: Config =
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file_exact(config_file))
            .extract()?;
    Ok(config)
}
