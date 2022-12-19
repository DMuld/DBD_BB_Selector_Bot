pub fn run(){
    //Normal Print
    println!("Hello from the print.rs file");
    
    //Print Formatting
    println!("Number: {}", 1);
    
    //Postional Arguments
    println!("{0} is {1} is {0}", "Devin", "Jacob");

    //Named arguments
    println!("{name} likes to play {activity}", name = "Devin", activity = "football");

    //Place hodler traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Place holder for DEBUG
    println!("{:?}", (12, true, "Hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}