
#[derive(Debug, Clone)]
pub struct Select {
    pub items: Vec<Item>,
    pub selected: usize,
    pub current: usize,
}


#[derive(Debug, Clone)]
pub struct Item {
    pub text: String,
    pub value: String,
    pub disabled: bool,
}

impl Select {
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            selected: 0,
            current: 0
        }
    }
}

impl Item {
    pub fn new(text: String, value: String, disabled: Option<bool>) -> Self {
        Self {
            text,
            value,
            disabled: disabled.unwrap_or(false),
        }
    }
}