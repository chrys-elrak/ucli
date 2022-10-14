use ucli::item::Item;
use ucli::select::Select;
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new_str("Akondro", 10, Some(true)),
        Item::new_str("Tsaramaso", 5, None),
        Item::new("Pibasy".to_string(), 44, None),
        Item::new("Manga".to_string(), 77, None),
    ]);

    let value = Main::new(&options)
        .enable_mouse()
        .set_quit_cmd('a')
        .render()
        .get();
    println!("You selected: {:?}", value);
}
