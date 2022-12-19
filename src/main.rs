mod mouse_event_handler;
mod screen_capture;
mod image_recognition;

fn main() {
    println!("Program Launched.");

    mouse_event_handler::switch();
    screen_capture::run();
    mouse_event_handler::run();
    image_recognition::run();
    println!("Program Closed.");
}
