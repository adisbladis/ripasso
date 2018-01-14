#![cfg(feature = "use-tui")]
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, LinearLayout,ListView, TextView, EditView};
use self::cursive::align::HAlign;
use self::cursive::direction::Orientation;
use pass;
use std;
use std::process;


pub fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Load and watch all the passwords in the background
    let (password_rx, passwords) = match pass::watch() {
        Ok(t) => t,
        Err(e) => {
            //writeln!(&mut std::io::stderr(), "Error: {}", e);
            process::exit(0x01);
        }
    };

    let searchbox = EditView::new()
        .on_edit(move |s, q, l| {
            s.call_on_id("results", |l: &mut ListView| {
                let r = pass::search(&passwords, String::from(q));
                l.clear();
                for p in r.iter() {
                    l.add_child(
                        &format!("item {}", &p.name),
                        TextView::new(p.name.clone()).fixed_height(5),
                    );
                }
                l.focus();
            });
        })
        .fixed_width(20);


    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::around(
            LinearLayout::new(Orientation::Vertical)
                .child(searchbox)
                .child(
                    ListView::new()
                        .with_id("results")
                )
                .full_height()
        )
            .title("Ripasso")
    );


    // Starts the event loop.
    siv.run();
}
