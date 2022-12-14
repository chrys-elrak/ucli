#[cfg(test)]
pub mod test {
    use crate::{item::UCLISelectItem, select::UCLISelect, ucli::Main};

    /// Select one element shoud return the selected element
    /// Select hapen when the user press enter
    #[test]
    pub fn get_one() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 1),
            UCLISelectItem::default("Option 2", 2),
            UCLISelectItem::default("Option 3", 10),
        ]);
        let value = Main::new(select).render(true).get();
        assert_eq!(value, Some(10));
    }
    /// Return err
    /// Press quit button 'q'
    #[test]
    pub fn no_selection_error() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 10),
            UCLISelectItem::default("Option 2", 10),
            UCLISelectItem::default("Option 3", 5),
        ]);
        let value = Main::new(select).render(true).get_value();
        assert_eq!(value, Err("No item selected"));
    }
    /// Get Ok()
    #[test]
    pub fn get_ok() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 10),
            UCLISelectItem::default("Option 2", 10),
            UCLISelectItem::default("Option 3", 10),
        ]);
        let value = Main::new(select).set_default_value(0).render(true).get_value().unwrap();
        assert_eq!(value, 10);
    }
    /// The default value should be selected
    #[test]
    pub fn get_default() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 10),
            UCLISelectItem::default("Option 2", 10),
            UCLISelectItem::default("Option 3", 5),
        ]);
        let value = Main::new(select).set_default_value(2).render(true).get();
        assert_eq!(value, Some(5));
    }
    /// Can't get the value 5 because it is disabled
    #[test]
    pub fn cant_get() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 10),
            UCLISelectItem::new("Option 2".to_string(), 5, true),
            UCLISelectItem::default("Option 3", 10),
        ]);
        let value = Main::new(select).render(true).get();
        assert_ne!(value.unwrap(), 5);
    }
    /// None should be returned when the user press `q`
    /// User does not select any value
    #[test]
    pub fn get_none() {
        let select = UCLISelect::new(vec![
            UCLISelectItem::default("Option 1", 10),
            UCLISelectItem::default("Option 2", 10),
            UCLISelectItem::default("Option 3", 10),
        ]);
        let value = Main::new(select).render(true).get();
        assert_eq!(value, None);
    }
}
