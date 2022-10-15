#[cfg(test)]
pub mod test {
    use crate::{select::{Select}, item::Item, ucli::Main};
    #[test]
    pub fn select_one() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 10, false),
        ]);
        let value = Main::new(&select)
            .set_default_puce("⚪")
            .set_selected_puce("🟢")
            .set_disabled_puce("❌")
            .render()
            .get();
        println!("You selected: {:?}", value);
        assert_eq!(value, Some(10));
    }
}