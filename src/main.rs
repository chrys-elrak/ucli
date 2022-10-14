use ucli::select::{Select, Item};
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new("Option 1".to_string(), "1".to_string(), None),
        Item::new("Option 1".to_string(), "1".to_string(), None),
        Item::new("Option 1".to_string(), "1".to_string(), None),
        Item::new("Option 1".to_string(), "1".to_string(), None),
    ]);

    Main::new(&options)
    .render();
}