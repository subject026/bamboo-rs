use std::io::Error;

use real_bamboo::{
    bamboo::Bamboo,
    router::{Route, Router},
};
use views::{Home, Settings};

mod views;

fn main() -> Result<(), Error> {
    let mut router = Router::new();

    // initial view
    router.push(Route {
        path: "/settings".to_string(),
        view: Box::new(Settings::new()),
    });

    Bamboo::new(router).run()
}
