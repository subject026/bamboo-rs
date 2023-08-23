use real_bamboo::{
    router::{Route, Router},
    view::BambooView,
};
use termion::event::Key;

enum AppViews {
    Home(Home),
    Settings(Settings),
}

pub struct Home {
    home_message: String,
}

impl Home {
    pub fn new() -> Self {
        Self {
            home_message: "this is the home screen".to_string(),
        }
    }
}

impl BambooView for Home {
    fn handle_keypress(&mut self, key: Key) {
        // update focused menu item...
    }
    fn render(&self) {
        println!("{}", self.home_message)
    }
}

pub struct Settings {
    settings_message: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            settings_message: "this is the settings screen".to_string(),
        }
    }
}

impl BambooView for Settings {
    fn handle_keypress(&mut self, key: &Key) {
        if key == Key::Char('1') {
            // self.router.push(Route {
            //     path: "/home".to_string(),
            //     view: Box::new(Home::new()),
            // })
        }
    }

    fn render(&self) {
        println!("{}", self.settings_message)
    }
}
