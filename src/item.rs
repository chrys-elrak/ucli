#[derive(Debug, Clone)]
pub struct UCLISelectItem<T> {
    pub text: String,
    pub value: T,
    pub disabled: bool, // if true, the item is disabled => cannot be selected
}

impl<T> UCLISelectItem<T> {
    pub fn new(text: String, value: T, disabled:bool) -> Self {
        Self {
            text,
            value,
            disabled,
        }
    }
    pub fn default(text: &str, value: T) -> Self {
        Self {
            text: text.to_string(),
            value,
            disabled: false,
        }
    }
}