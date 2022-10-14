#[derive(Debug, Clone)]
pub struct Item<T> {
    pub text: String,
    pub value: T,
    pub disabled: bool, // if true, the item is disabled => cannot be selected
    pub is_current: bool, // the current item is highlighted
}

impl<T> Item<T> {
    pub fn new_str(text: &str, value: T, disabled: Option<bool>) -> Self {
        Self {
            text: text.to_string(),
            value,
            disabled: disabled.unwrap_or(false),
            is_current: false,
        }
    }
    pub fn new(text: String, value: T, disabled: Option<bool>) -> Self {
        Self {
            text,
            value,
            disabled: disabled.unwrap_or(false),
            is_current: false,
        }
    }
}