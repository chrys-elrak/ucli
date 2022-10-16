use ucli::item::UCLISelectItem;
use ucli::select::UCLISelect;
use ucli::ucli::Main;

fn main() {
    let options = UCLISelect::new(vec![
        UCLISelectItem::new("Cat".to_string(), "🐈", true),
        UCLISelectItem::new("Dog".to_string(), "🐕", false),
        UCLISelectItem::new("Mouse".to_string(), "🐁", false),
    ]);
    let value = Main::new(options)
        .enable_mouse()
        .render()
        .get();
    println!("You selected: {:?}", value);
}
