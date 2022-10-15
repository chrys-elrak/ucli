use std::vec;

use crate::item::Item;

#[derive(Debug, Clone)]
pub struct Select<T> 
where T:Clone
{
    pub items: Vec<Item<T>>,
    pub selected: i32,
    pub current: i32,
    pub selections: Vec<Item<T>>
}

impl<T> Select<T>
where T:Clone
 {
    pub fn new(items: Vec<Item<T>>) -> Self {
        Self {
            items,
            selected: -1, // -1 means no item is selected
            current: -1, // for cursor tracking
            selections: vec![]
        }
    }
}

