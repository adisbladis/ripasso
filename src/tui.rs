#![cfg(feature = "use-tui")]
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, LinearLayout,ListView, SelectView, TextView, EditView};
use self::cursive::align::HAlign;
use self::cursive::direction::Orientation;
use self::cursive::event::Event;

use pass;
use std;
use std::process;


pub fn main() {

    // Load and watch all the passwords in the background
    let (password_rx, passwords) = match pass::watch() {
        Ok(t) => t,
        Err(e) => {
            //writeln!(&mut std::io::stderr(), "Error: {}", e);
            process::exit(0x01);
        }
    };

    let mut siv = Cursive::new();

    siv.add_global_callback(Event::CtrlChar('y'),|s|{
        println!("Quit");
    } );

    siv.add_global_callback(Event::CtrlChar('n'),|s|{
        s.call_on_id("results", |l: &mut SelectView| {
            l.select_down(1);
        });
    } );

    siv.add_global_callback(Event::CtrlChar('p'),|s|{
        s.call_on_id("results", |l: &mut SelectView| {
            l.select_up(1);
        });
    } );

    //siv.load_theme(include_str!("../res/style.toml")).unwrap();
    siv.load_theme_file("res/style.toml").unwrap();
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
                    .child(TextView::new("CTRL-N: Next "))
                    .child(TextView::new("CTRL-P: Previous "))
                    .child(TextView::new("CTRL-Y: Copy "))
                    .full_width()
            )
    );
    siv.run();
}
