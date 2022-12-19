mod mouse_event_handler;
mod screen_capture;

fn main() {
    println!("Program Launched.");

    mouse_event_handler::switch();
    screen_capture::run();

    println!("Program Closed.");
}
