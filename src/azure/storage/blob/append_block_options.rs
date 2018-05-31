#[derive(Debug, Clone, PartialEq)]
pub struct AppendOptions {
    pub timeout: Option<u64>,
}

pub const APPEND_OPTIONS_DEFAULT: AppendOptions = AppendOptions {
    timeout: None,
};
