mod image_recognition;
mod mouse_event_handler;

fn main() {
    println!("Program Launched.");

    mouse_event_handler::switch();
    image_recognition::run();
    mouse_event_handler::run();

    println!("Program Finished.");
}
