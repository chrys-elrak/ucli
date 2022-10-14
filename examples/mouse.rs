use ucli::item::Item;
use ucli::select::Select;
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new_str("Cat", "🐈", true),
        Item::new_str("Dog", "🐕", false),
        Item::new_str("Mouse", "🐁", false),
    ]);
    let value = Main::new(&options)
        .enable_mouse()
        .render()
        .get();
    println!("You selected: {:?}", value);
}
