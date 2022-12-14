use crate::item::UCLISelectItem;
#[derive(Debug, Clone)]
pub struct UCLISelect<T> {
    pub items: Vec<UCLISelectItem<T>>,
    selected: i32,
    pub current: i32,
}

impl<T> UCLISelect<T> {
    pub fn new(items: Vec<UCLISelectItem<T>>) -> Self {
        Self {
            items,
            selected: -1, // -1 means no item is selected
            current: -1, // for cursor tracking
        }
    }
    /// Set the selected item to the current item
    pub fn set_selected(&mut self) -> &Self {
        self.selected = self.current;
        self
    }
    /// Set the current item
    pub fn set_current(&mut self, i: i32) -> &Self {
        self.current = i;
        self
    }
    pub fn get_selected(&self) -> Option<&UCLISelectItem<T>> {
        if self.selected >= 0 {
            return Some(&self.items[self.selected as usize]);
        }
        None
    }
}

