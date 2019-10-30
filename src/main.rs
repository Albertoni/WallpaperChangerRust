#![windows_subsystem = "windows"]

extern crate wallpaper;
extern crate systray;

use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        loop {
            // TODO: https://doc.rust-lang.org/std/thread/fn.sleep.html
            // On Unix platforms, the underlying syscall may be interrupted by a spurious wakeup or signal handler.
            // To ensure the sleep occurs for at least the specified duration, this function may invoke that system call multiple times.\

            println!("HENLO TIMER");
            wallpaper::set_from_url("https://cdn.star.nesdis.noaa.gov/GOES16/ABI/SECTOR/ssa/GEOCOLOR/3600x2160.jpg").unwrap();
            thread::sleep(Duration::from_secs(1200));
        }
    });

    register_tray_icon();
}

fn register_tray_icon() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!")
    }

    let mut path = std::env::current_dir().ok().unwrap();
    path.push("icon.ico");
    let result = app.set_icon_from_file(&path.to_str().unwrap().to_string());
    match result {
        Err(x) => println!("Couldn't load icon: {}", x),
        Ok(_) => (),
    }

    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
        std::process::exit(0);
    }).ok();

    println!("Waiting on message!");
    app.wait_for_message();
}