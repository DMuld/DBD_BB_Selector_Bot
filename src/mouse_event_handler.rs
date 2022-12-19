use enigo::*;
use std::{thread, time};

pub fn switch(){
    //Switches from the program to the DBD page.
    println!("You have 5 seconds to open DBD Bloodweb!");
    thread::sleep(time::Duration::from_secs(5));
}

pub fn mouse_click(pos_x: i32, pos_y: i32) {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(pos_x, pos_y);
    enigo.mouse_down(MouseButton::Left);
    thread::sleep(time::Duration::from_secs(1));
    enigo.mouse_up(MouseButton::Left);
}