#![cfg(feature = "use-tui")]
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::views::{Dialog, EditView};

pub fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(EditView::new())
                         .title("Cursive")
                         .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}
