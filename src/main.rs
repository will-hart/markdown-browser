// #![windows_subsystem = "windows"]
// #![allow(deprecated)]

extern crate web_view;

// use std::thread::{spawn, sleep_ms};
// use std::sync::{Arc, Mutex};
use web_view::*;

const HTML: &'static str = include_str!("../www/index.html");
// const JS: &'static str = include_str!("../www/index.js");

fn main() {
    println!("LOADING APPLICATION");

    let size = (800, 800);
    let resizable = true;
    let debug = true;

    run("Markdown Viewer",
        Content::Html(HTML),
        Some(size),
        resizable,
        debug,
        move |_| { },
        move |_, arg, _| {
            match arg {
                _ => {
                    println!("Unimplemented");
                    unimplemented!();
                }
            }
        },
        ());

    // tether::builder()
    //     .html(html)
    //     .handler(|_, msg: &str| {
    //         println!("Received: {}", msg);
    //     })
    //     .start();
}

// fn render<'a, T>(webview: &mut WebView<'a, T>, counter: u32, userdata: i32) {
//     println!("counter: {}, userdata: {}", counter, userdata);
//     webview.eval(&format!("updateTicks({}, {})", counter, userdata));
// }