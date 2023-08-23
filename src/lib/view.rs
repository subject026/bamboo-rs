use termion::event::Key;

pub trait BambooView {
    fn handle_keypress(&mut self, key: &Key);
    fn render(&self);
}
