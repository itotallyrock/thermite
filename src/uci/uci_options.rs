
/// An option that a UCI GUI can use to configure an engine
pub struct UciOption {
    pub name: String,
    pub option: UciOptionType,
}

pub enum UciOptionType {
    Button,
    Check {
        default: Option<bool>,
    },
    Spin {
        min: Option<i64>,
        max: Option<i64>,
        default: Option<i64>,
    },
    Combo {
        // TODO: See if we can use a better type than Vec
        options: Vec<String>,
        default: Option<String>,
    },
    String {
        default: Option<String>,
    },
}

/// Key value pair for a [UciOption]
pub struct UciConfig {
    // TODO: See if we can instead reference the original &UciOption
    pub name: String,
    pub value: Option<String>,
}