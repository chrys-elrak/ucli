use ucli::item::Item;
use ucli::select::Select;
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new_str("Pomma", 10, true),
        Item::new_str("Paiso", 5, false),
        Item::new("Pibasy".to_string(), 44, false),
    ]);
    let value = Main::new(&options)
        .set_default_puce("⚪")
        .set_selected_puce("🟢")
        .set_disabled_puce("❌")
        .render()
        .get();
    println!("You selected: {:?}", value);
}
