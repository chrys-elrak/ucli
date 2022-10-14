use crate::select::Select;

const DEFAULT: &str = "";
const SELECTED: &str = ">";
const DISABLED: &str = "x";

pub struct Main {
    pub select: Select,
    pub default: &'static str,
    pub selected: &'static str,
    pub disabled: &'static str,
}

impl Main {
    pub fn new(select: &Select) -> Self {
        Self {
            select: select.clone(),
            default: DEFAULT,
            selected: SELECTED,
            disabled: DISABLED,
        }
    }
    /// Set the default puce to use
    /// By default, it's an empty string
    pub fn set_default(&mut self, puce: &'static str) {
        self.default = puce;
    }
    /// Set the selected puce to use
    /// By default, it's a `>`
    pub fn set_selected(&mut self, puce: &'static str) {
        self.selected = puce;
    }
    /// Set the disabled puce to use
    /// By default, it's a `x`
    pub fn set_disabled(&mut self, puce: &'static str) {
        self.disabled = puce;
    }
    pub fn render(&self) {
        for (i, item) in self.select.items.iter().enumerate() {
            if i == self.select.current {
                print!("{} ", self.selected);
            } else {
                print!("{} ", self.default);
            }
            if item.disabled {
                println!("{} {}", self.disabled, item.text);
            } else {
                println!("{}", item.text);
            }
        }
    }
}