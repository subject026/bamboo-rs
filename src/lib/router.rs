use crate::view::BambooView;

/**
 * https://doc.rust-lang.org/book/ch17-02-trait-objects.html
 */
pub struct Router {
    routes: &'static Vec<Route>,
}

impl Router {
    // receives routes to register
    // tree?
    pub fn new() -> Self {
        Self {
            routes: &Vec::new(),
        }
    }
    pub fn push(&mut self, item: Route) {
        // new instance of route
        self.routes.push(item);
    }
    pub fn pop(&mut self) {
        self.routes.pop();
    }
    pub fn peek(&self) -> Option<&mut Route> {
        let copy = self.routes.last().clone().unwrap();
    }
}

pub struct Route {
    pub path: String,
    pub view: Box<dyn BambooView>,
}
