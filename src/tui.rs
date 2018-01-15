#![cfg(feature = "use-tui")]
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, LinearLayout,ListView, SelectView, TextView, EditView};
use self::cursive::align::HAlign;
use self::cursive::direction::Orientation;
use pass;
use std;
use std::process;


pub fn main() {
    let mut siv = Cursive::new();

    //siv.load_theme(include_str!("../res/style.toml")).unwrap();
    siv.load_theme_file("res/style.toml").unwrap();

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
            s.call_on_id("results", |l: &mut SelectView| {
                let r = pass::search(&passwords, String::from(q));
                l.clear();
                for p in r.iter() {
                    l.add_item(p.name.clone(), p.name.clone());
                }
            });
        })
        .fixed_width(72);

    let results = SelectView::<String>::new()
        .with_id("results")
        .full_height();

    siv.add_layer(
        LinearLayout::new(Orientation::Vertical)
            .child(
                Dialog::around(
                    LinearLayout::new(Orientation::Vertical)
                        .child(searchbox)
                        .child(results)
                        .fixed_width(72)
                )
                    .title("Ripasso")
            )
            .child(
                LinearLayout::new(Orientation::Horizontal)
                    .child(TextView::new("CTRL-X: Quit "))
                    .child(TextView::new("CTRL-J: Down "))
                    .child(TextView::new("CTRL-N: Up "))
                    .full_width()
            )
    );
    siv.run();
}
