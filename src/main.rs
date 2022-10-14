use ucli::select::{Item, Select};
use ucli::ucli::Main;

fn main() {
    let options = Select::new(vec![
        Item::new("Akondro".to_string(), 10, Some(true)),
        Item::new("Tsaramaso".to_string(), 5, None),
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
