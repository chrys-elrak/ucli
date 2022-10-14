use ucli::item::Item;
use ucli::select::Select;
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new_str("Akondro", 10, true),
        Item::new_str("Tsaramaso", 5, false),
        Item::new("Pibasy".to_string(), 44, false),
        Item::new("Manga".to_string(), 77, false),
    ]);

    let value = Main::new(&options)
        .set_default_puce("⚪")
        .set_selected_puce("🟢")
        .set_disabled_puce("❌")
        .render()
        .get();
    println!("You selected: {:?}", value);
}
