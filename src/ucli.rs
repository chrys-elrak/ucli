use crate::{item::UCLISelectItem, select::UCLISelect};
use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode},
    execute,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::io::{stdout, Stdout};

const DEFAULT_ICON: &str = "*";
const SELECTED_ICON: &str = ">";
const DISABLED_ICON: &str = "x";

/// The main struct for the ucli library.
/// Usage:
/// ```
/// use ucli::item::UCLISelectItem;
/// use ucli::select::UCLISelect;
/// use ucli::ucli::Main;
/// let options = UCLISelect::new(vec![
///     UCLISelectItem::new("Akondro".to_string(), 10, true),
///     UCLISelectItem::new("Tsaramaso".to_string(), 5, false), 
///     UCLISelectItem::new("Pibasy".to_string(), 44, false),
/// ]);
/// let value = Main::new(options)
///     .set_default_puce("⚪")
///     .set_selected_puce("🟢")
///     .set_disabled_puce("❌")
///     .render()
///     .get();
/// println!("You selected: {:?}", value);
/// ```
#[derive(Debug)]
pub struct Main<T> {
    select: UCLISelect<T>,
    default_icon: String,
    selected_icon: String,
    disabled_icon: String,
    use_mouse: bool,
    quit_cmd: KeyCode,
    stdout: Stdout,
    prompt: String,
}

impl<T: Clone> Main<T> {
    pub fn new(select: UCLISelect<T>) -> Self {
        Self {
            select,
            default_icon: String::from(DEFAULT_ICON),
            selected_icon: String::from(SELECTED_ICON),
            disabled_icon: String::from(DISABLED_ICON),
            stdout: stdout(),
            use_mouse: false,
            quit_cmd: KeyCode::Char('q'),
            prompt: String::from("Select an option: "),
        }
    }

    /// Set default value for select
    /// Take the index of the item for the default value
    /// Zero based index
    /// # Exemples
    /// ```
    /// use ucli::item::UCLISelectItem;
    /// use ucli::select::UCLISelect;
    /// use ucli::ucli::Main;
    /// let options = UCLISelect::new(vec![
    ///     UCLISelectItem::new("Akondro".to_string(), 10, true),
    ///     UCLISelectItem::new("Tsaramaso".to_string(), 5, false),
    ///     UCLISelectItem::new("Pibasy".to_string(), 44, false),]);
    /// let value = Main::new(options)
    ///     .set_default_value(1)
    ///     .render()
    ///     .get();
    ///  assert_eq!(value, Some(5));
    /// ```
    /// When the default vaue is disabled, this will make code panic
    /// ```should_panic
    /// use ucli::item::UCLISelectItem;
    /// use ucli::select::UCLISelect;
    /// use ucli::ucli::Main;
    /// let options = UCLISelect::new(vec![
    ///     UCLISelectItem::new("Akondro".to_string(), 10, true),
    ///     UCLISelectItem::new("Tsaramaso".to_string(), 5, false),
    ///     UCLISelectItem::new("Pibasy".to_string(), 44, false),
    /// ]);
    /// let value = Main::new(options)
    ///     .set_default_value(0) // this element is disabled
    ///     .render()
    ///     .get();
    /// ```
    pub fn set_default_value(&mut self, index: i32) -> &mut Self {
        if self.select.items.len() > index as usize {
            let it = self.get_item(index as usize);
            if !it.disabled {
                self.select.set_current(index);
                self.select.set_selected();
            } else {
                panic!("The default item is disabled, please select another item to be the default selected item or make sure the item is not disabled");
            }
        }
        self
    }

    /// Set the default puce to use
    /// By default, it's an empty string
    pub fn set_default_puce(&mut self, puce: &str) -> &mut Self {
        self.default_icon = puce.to_string();
        self
    }

    /// Set the selected puce to use
    /// By default, it's a `>`
    pub fn set_selected_puce(&mut self, puce: &str) -> &mut Self {
        self.selected_icon = puce.to_string();
        self
    }

    /// Set the disabled puce to use
    /// By default, it's a `x`
    pub fn set_disabled_puce(&mut self, puce: &str) -> &mut Self {
        self.disabled_icon = puce.to_string();
        self
    }

    /// Enable mouse support
    /// By default, it's disabled
    pub fn enable_mouse(&mut self) -> &mut Self {
        self.use_mouse = true;
        self
    }

    /// Set the quit command
    /// By default, it's `q`
    pub fn set_quit_cmd(&mut self, cmd: char) -> &mut Self {
        self.quit_cmd = KeyCode::Char(cmd);
        self
    }

    /// Get item from select by index
    /// Zero based index
    /// Private function
    fn get_item(&self, index: usize) -> &UCLISelectItem<T> {
        &self.select.items[index]
    }

    /// Set message propmt to display
    pub fn set_prompt(&mut self, message: String) -> &mut Self {
        // remove all lines from message
        self.prompt = message.replace("\n", "");
        self
    }

    /// Print disabled item
    fn print_disabled(&mut self, cursor: usize) -> &Self {
        let it = &self.select.items[cursor - 1];
        execute!(
            self.stdout,
            cursor::MoveTo(0, cursor as u16),
            PrintStyledContent(format!("{} {}", self.disabled_icon, it.text).black()),
        )
        .expect("Failed to print disabled item");
        self
    }

    /// Print one item
    pub fn print(&mut self, cursor: usize) -> &mut Self {
        let it = &self.select.items[cursor - 1];
        execute!(
            self.stdout,
            cursor::MoveTo(0, cursor as u16),
            PrintStyledContent(format!("{} {}", self.default_icon, it.text).white()),
        )
        .expect("Failed to print");
        self
    }

    /// Print current highlighted item
    pub fn print_current(&mut self, cursor: usize) -> &mut Self {
        let it = &self.select.items[cursor - 1];
        execute!(
            self.stdout,
            cursor::MoveTo(0, cursor as u16),
            PrintStyledContent(
                format!("{} {}", self.selected_icon, it.text)
                    .black()
                    .on_green()
            ),
        )
        .expect("Failed to print current");
        self
    }

    /// Render the select
    /// It returns the current instance of the struct
    /// It should the last method called before `get`
    pub fn render(&mut self, clear: bool) -> &mut Self {
        enable_raw_mode().unwrap();
        if self.use_mouse {
            execute!(self.stdout, event::EnableMouseCapture).unwrap();
        }
        'MAIN_LOOP: loop {
            execute!(
                self.stdout,
                cursor::MoveTo(0, 0),
                cursor::Hide,
                Print(&self.prompt),
            )
            .unwrap();
            if clear {
                execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
            }
            'PRINT_LOOP: for i in 0..self.select.items.len() {
                let it = &self.select.items[i];
                let cursor = i as usize + 1;
                if it.disabled {
                    self.print_disabled(cursor);
                    continue 'PRINT_LOOP;
                }
                if self.select.current == i as i32 {
                    self.print_current(cursor);
                    continue 'PRINT_LOOP;
                }
                self.print(cursor);
            }
            match read().unwrap() {
                // Handle the key event => UP, DOWN, ENTER, QUIT
                Event::Key(key) => {
                    if key.code == self.quit_cmd {
                        break 'MAIN_LOOP;
                    }
                    match key.code {
                        KeyCode::Up => {
                            if self.select.current > 0 {
                                self.select.current -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.select.current < (self.select.items.len() - 1) as i32 {
                                self.select.current += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if self.select.current < 0 {
                                break 'MAIN_LOOP;
                            }
                            if self.select() {
                                break 'MAIN_LOOP;
                            }
                        }
                        _ => {}
                    }
                }
                // Handle the mouse event => HOVER and LEFT_CLICK
                Event::Mouse(e) => {
                    if e.row >= 1 && e.row <= self.select.items.len() as u16 {
                        if e.column < self.select.items[e.row as usize - 1].text.len() as u16 {
                            self.select.current = e.row as i32 - 1;
                            if e.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                                if self.select() {
                                    break 'MAIN_LOOP;
                                }
                            }
                        }
                    }
                }
                _ => {}
            };
        }
        disable_raw_mode().unwrap();
        execute!(
            self.stdout,
            cursor::MoveTo(0, self.select.items.len() as u16 + 1),
            event::DisableMouseCapture,
            cursor::Show,
        )
        .unwrap();
        self
    }

    /// Select the element int the current cursor if not disabled
    fn select(&mut self) -> bool {
        let it = self.get_item(self.select.current as usize);
        if !it.disabled {
            self.select.set_selected();
            return true;
        }
        false
    }

    /// Get the selected item
    /// It should be called after `render`
    /// It returns the selected item value or `None` if no item is selected
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Some(it) = self.select.get_selected() {
            return Some(it.value.clone());
        }
        None
    }
    /// Get the direct value of the selected item
    /// It should be called after `render`
    pub fn get_value(&self) -> Result<T, &'static str> {
        if let Some(s) = self.select.get_selected() {
            return Ok(s.value.clone());
        }
        Err("No item selected")
    }
}
