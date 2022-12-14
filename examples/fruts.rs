use ucli::item::UCLISelectItem;
use ucli::select::UCLISelect;
use ucli::ucli::Main;

fn main() {
    let options = UCLISelect::new(vec![
        UCLISelectItem::new("Pomma".to_string(), 10, false),
        UCLISelectItem::new("Paiso".to_string(), 5, false),
        UCLISelectItem::new("Pibasy".to_string(), 44, false),
    ]);
    for i in 0..5 {
        let value = Main::new(options.clone())
            .set_default_puce("⚪")
            .set_selected_puce("🟢")
            .set_disabled_puce("❌")
            .set_default_value(0)
            .render()
            .get();
            println!("You selected: {:?}", value);
    }
}
