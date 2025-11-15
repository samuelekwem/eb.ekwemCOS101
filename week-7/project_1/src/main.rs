use std::io;

// Utility function to read a floating-number from the user
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().parse::<f64>().expect("Invalid number entered.")

}

// Shape calculation functions
fn area_trapezium() {
    let height = read_input("Enter the height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2");
    let area = height / 2.0 * (base1 + base2);
    println!("Area of the trapezium = {}", area);

}

fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of the rhombus = {}", area);

}

fn area_parallelogram() {
    let base = read_input("Enter the base:");
    let altitude = read_input("Enter the altitude:");
    let area = base * altitude;
    println!("Area of the parallelogram = {}", area);
}

fn area_cube() {
    let side = read_input("Enter the length of the side:");
    let area = 6.0 * side.powi(2);
    println!("Area of the cube = {}", area);

}

fn volume_cylinder() {
    let radius = read_input("Enter the radius:");
    let height = read_input("Enter the height:");
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("Volume of the cylinder = {}", volume);

}

fn main() {
    println!("Select a calculation:");
    println!("1. Area of trapezium");
    println!("2. Area of rhombus");
    println!("3. Area of parallelogram");
    println!("4. Area of cube");
    println!("5. Volume of cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input.");

    match choice.trim() {
        "1" => area_trapezium(),
        "2" => area_rhombus(),
        "3" => area_parallelogram(),
        "4" => area_cube(),
        "5" => volume_cylinder(),
       _ => println!("Invalid choice."),
    }
    
}