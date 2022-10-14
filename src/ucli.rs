use crate::select::{Item, Select};
use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode},
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use std::io::{stdout, Write};

const DEFAULT: &str = "";
const SELECTED: &str = ">";
const DISABLED: &str = "x";

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
                cursor::MoveTo(0, 0),
                terminal::Clear(ClearType::All),
            )
            .unwrap();
            for i in 0..self.select.items.len() {
                let mut it = self.select.items[i].clone();
                it.is_current = self.select.current == i;
                match it.is_current {
                    true => {
                        execute!(
                            self.stdout,
                            cursor::MoveTo(0, i as u16),
                            PrintStyledContent(it.text.green()),
                        )
                        .unwrap();
                    }
                    false => {
                        execute!(
                            self.stdout,
                            cursor::MoveTo(0, i as u16),
                            PrintStyledContent(it.text.red()),
                        )
                        .unwrap();
                    }
                }
            }
            match read().unwrap() {
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
                            if self.select.current < (self.select.items.len() - 1) as usize {
                                self.select.current += 1;
                            }
                        }
                        KeyCode::Enter => {}
                        _ => {}
                    }
                }
                Event::Mouse(e) => {
                    println!("{}", e.row);
                    if e.row < self.select.items.len() as u16 {
                        self.select.current = e.row as usize;
                    }
                }
                _ => {}
            };
            self.stdout.flush().unwrap();
        }
        disable_raw_mode().unwrap();
        self
    }
    /// Get the selected item
    /// It should be called after `render`
    /// It returns the selected item or `None` if no item is selected
    pub fn get(&self) -> Option<Item<T>>
    where
        T: Clone,
    {
        if self.select.selected > 0 {
            return Some(self.select.items[self.select.current].clone());
        }
        None
    }
}
