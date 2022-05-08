
/// An option that a UCI GUI can use to configure an engine
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UciOption {
    pub name: String,
    pub option: UciOptionType,
}

/// Different option kinds and their configurations
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UciOptionType {
    Button,
    Check {
        default: bool,
    },
    Spin {
        min: i64,
        max: i64,
        default: i64,
    },
    Combo {
        // TODO: See if we can use a better type than Vec
        options: Vec<String>,
        default: String,
    },
    String {
        default: String,
    },
}

/// Key value pair for a [UciOption]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UciConfig {
    // TODO: See if we can instead reference the original &UciOption
    pub name: String,
    pub value: Option<String>,
}