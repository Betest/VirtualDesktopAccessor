use std::thread;
use winvd::{get_desktops, get_event_receiver, helpers::get_desktop_count, VirtualDesktopEvent};

fn main() {
    // Desktop count
    let desktops = get_desktop_count().unwrap();
    println!("Desktops {:?}", desktops);

    // Desktops are:
    println!("Desktops are: {:?}", get_desktops().unwrap());

    thread::spawn(|| {
        get_event_receiver().iter().for_each(|msg| match msg {
            VirtualDesktopEvent::DesktopChanged(old, new) => {
                println!(
                    "<- Desktop changed from {:?} to {:?} {:?}",
                    old,
                    new,
                    thread::current().id()
                );
            }
            VirtualDesktopEvent::DesktopCreated(desk) => {
                println!("<- New desktop created {:?}", desk);
            }
            VirtualDesktopEvent::DesktopDestroyed(desk) => {
                println!("<- Desktop destroyed {:?}", desk);
            }
            VirtualDesktopEvent::WindowChanged(hwnd) => {
                println!("<- Window changed {:?}", hwnd);
            }
        })
    });

    println!("Press enter key to close...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
