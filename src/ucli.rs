use crate::{item::Item, select::Select};
use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode},
    execute,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::io::{stdout, Write};

const DEFAULT: &str = "*";
const SELECTED: &str = ">";
const DISABLED: &str = "x";

/// The main struct for the ucli library.
/// Usage:
/// ```
/// use ucli::item::Item;
/// use ucli::select::Select;
/// use ucli::ucli::Main;
/// let options = Select::new(vec![Item::new_str("Akondro", 10, true),  Item::new_str("Tsaramaso", 5, false), Item::new("Pibasy".to_string(), 44, false),]);
/// let value = Main::new(&options)
/// .set_default_puce("⚪")
/// .set_selected_puce("🟢")
/// .set_disabled_puce("❌")
/// .render()
/// .get();
/// println!("You selected: {:?}", value);
/// ```
pub struct Main<T>
where
    T: Clone,
{
    select: Select<T>,
    default: &'static str,
    selected: &'static str,
    disabled: &'static str,
    use_mouse: bool,
    quit_cmd: KeyCode,
    stdout: std::io::Stdout,
    prompt: String,
}

impl<T> Main<T>
where
    T: Clone,
{
    pub fn new(select: &Select<T>) -> Self {
        Self {
            select: select.clone(),
            default: DEFAULT,
            selected: SELECTED,
            disabled: DISABLED,
            stdout: stdout(),
            use_mouse: false,
            quit_cmd: KeyCode::Char('q'),
            prompt: String::from("Select an option: "),
        }
    }
    /// Set the default puce to use
    /// By default, it's an empty string
    pub fn set_default_puce(&mut self, puce: &'static str) -> &mut Self {
        self.default = puce;
        self
    }
    /// Set the selected puce to use
    /// By default, it's a `>`
    pub fn set_selected_puce(&mut self, puce: &'static str) -> &mut Self {
        self.selected = puce;
        self
    }
    /// Set the disabled puce to use
    /// By default, it's a `x`
    pub fn set_disabled_puce(&mut self, puce: &'static str) -> &mut Self {
        self.disabled = puce;
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
    fn get_item(&self, index: usize) -> &Item<T> {
        &self.select.items[index]
    }
    pub fn prompt(&mut self, message: String) -> &mut Self {
        // remove all lines from message
        self.prompt = message.replace("\n", "");
        self
    }
    /// Render the select
    /// It returns the current instance of the struct
    /// It should the last method called before `get`
    pub fn render(&mut self) -> &mut Self {
        enable_raw_mode().unwrap();
        if self.use_mouse {
            execute!(self.stdout, event::EnableMouseCapture).unwrap();
        }
        'MAIN_LOOP: loop {
            execute!(
                self.stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                Print(self.prompt.clone()),
            )
            .unwrap();
            self.stdout.flush().unwrap();
            for i in 0..self.select.items.len() {
                let mut it = self.select.items[i].clone();
                it.is_current = self.select.current == i as i32;
                let cursor = i as usize + 1;
                if it.disabled {
                    execute!(
                        self.stdout,
                        cursor::MoveTo(0, cursor as u16),
                        PrintStyledContent(format!("{} {}", self.disabled, it.text).black()),
                    )
                    .unwrap();
                } else {
                    match it.is_current {
                        true => {
                            execute!(
                                self.stdout,
                                cursor::MoveTo(0, cursor as u16),
                                PrintStyledContent(
                                    format!("{} {}", self.selected, it.text).black().on_green()
                                ),
                            )
                            .unwrap();
                        }
                        false => {
                            execute!(
                                self.stdout,
                                cursor::MoveTo(0, cursor as u16),
                                PrintStyledContent(format!("{} {}", self.default, it.text).white()),
                            )
                            .unwrap();
                        }
                    }
                }
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
                            let it = self.get_item(self.select.current as usize);
                            if !it.disabled {
                                self.select.selected = self.select.current as i32;
                                break 'MAIN_LOOP;
                            }
                        }
                        _ => {}
                    }
                }
                // Handle the mouse event => HOVER and LEFT_CLICK
                Event::Mouse(e) => {
                    if e.row >= 1 && e.row <= self.select.items.len() as u16
                    {
                        if e.column < self.select.items[e.row as usize - 1].text.len() as u16 {
                            self.select.current = e.row as i32 - 1;
                            if e.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                                let it = self.get_item(self.select.current as usize);
                                if !it.disabled {
                                    self.select.selected = self.select.current as i32;
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
        ).unwrap();
        self
    }
    /// Get the selected item
    /// It should be called after `render`
    /// It returns the selected item value or `None` if no item is selected
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        if self.select.selected >= 0 {
            let e = self.select.items[self.select.current as usize].to_owned();
            return Some(e.value);
        }
        None
    }
}
