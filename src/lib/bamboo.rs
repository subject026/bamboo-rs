use std::io;
use std::io::Error;

use termion::event::Key;
use termion::input::TermRead;

use crate::router::{Route, Router};
use crate::terminal::Terminal;

pub struct Bamboo {
    router: Router,
    should_quit: bool,
    terminal: Terminal,
}

impl Bamboo {
    pub fn new(router: Router) -> Self {
        Self {
            router,
            terminal: Terminal::default().expect("Failed to initilize terminal"),
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // init
        println!("{}", "im running!!");

        let initial_route = self.router.peek().unwrap();
        initial_route.view.render();

        // start the loop
        loop {
            if self.should_quit == true {
                break;
            }
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        let current_route = self.router.peek().unwrap();
        current_route.view.render();
        Ok(())
    }
    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            // Key::Char(c) => {
            //     self.document.insert_char(&self.cursor_position, c);
            //     self.move_cursor(Key::Right)
            // }
            // Key::Delete => self.document.delete_char(&self.cursor_position),
            // Key::Backspace => {
            //     if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
            //         self.move_cursor(Key::Left);
            //         self.document.delete_char(&self.cursor_position);
            //     }
            // }
            // Key::Up
            // | Key::Down
            // | Key::Left
            // | Key::Right
            // | Key::PageUp
            // | Key::PageDown
            // | Key::End
            // | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }

        let current_route = self.router.peek().unwrap();
        current_route.view.handle_keypress(&pressed_key);

        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
