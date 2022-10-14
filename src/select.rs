#[derive(Debug, Clone)]
pub struct Select<T> 
where T:Clone
{
    pub items: Vec<Item<T>>,
    pub selected: i32,
    pub current: usize,
}

impl<T> Select<T>
where T:Clone
 {
    pub fn new(items: Vec<Item<T>>) -> Self {
        Self {
            items,
            selected: -1,
            current: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item<T> {
    pub text: String,
    pub value: T,
    pub disabled: bool,
    pub is_current: bool,
}

impl<T> Item<T> {
    pub fn new(text: String, value: T, disabled: Option<bool>) -> Self {
        Self {
            text,
            value,
            disabled: disabled.unwrap_or(false),
            is_current: false,
        }
    }
}