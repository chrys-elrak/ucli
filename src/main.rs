use ucli::item::Item;
use ucli::select::Select;
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new_str("Akondro", 10, false),
        Item::new_str("Tsaramaso", 5, false),
        Item::new("Pibasy".to_string(), 44, false),
        Item::new("Manga".to_string(), 77, false),
    ]);

    let value = Main::new(&options)
        .enable_mouse()
        .prompt("Select a value:".to_string())
        .set_quit_cmd('a')
        .render()
        .get();
    println!("You selected: {:?}", value);
}
