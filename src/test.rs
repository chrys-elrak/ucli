#[cfg(test)]
pub mod test {

    use crate::{item::Item, select::Select, ucli::Main};
    /// Select one element shoud return the selected element
    /// Select hapen when the user press enter
    #[test]
    pub fn get_one() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 10, false),
        ]);
        let value = Main::new(&select).render().get();
        assert_eq!(value, Some(10));
    }
    /// Return err
    #[test]
    pub fn no_selection_error() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 5, false),
        ]);
        let value = Main::new(&select).render().get_value();
        assert_eq!(value, Err("No item selected"));
    }
    /// Get Ok()
    #[test]
    pub fn get_ok() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 10, false),
        ]);
        let value = Main::new(&select).default(0).render().get_value().unwrap();
        assert_eq!(value, 10);
    }
    /// The default value should be selected
    #[test]
    pub fn get_default() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 5, false),
        ]);
        let value = Main::new(&select).default(2).render().get();
        assert_eq!(value, Some(5));
    }
    /// Can't get the value 5 because it is disabled
    #[test]
    pub fn cant_get() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 5, true),
            Item::new_str("Option 3", 10, false),
        ]);
        let value = Main::new(&select).render().get();
        assert_ne!(value.unwrap(), 5);
    }
    /// None should be returned when the user press `q`
    /// User does not select any value
    #[test]
    pub fn get_none() {
        let select = Select::new(vec![
            Item::new_str("Option 1", 10, false),
            Item::new_str("Option 2", 10, false),
            Item::new_str("Option 3", 10, false),
        ]);
        let value = Main::new(&select).render().get();
        assert_eq!(value, None);
    }
}
