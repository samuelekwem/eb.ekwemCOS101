use std::io;

fn main() {
    println!("=================MENU==============");
    println!("P = Poundo Yam / Edinkaiko Soup  - N3200");
    println!("F = Fried rice & Chicken         - N3000");
    println!("A = Amala & Ewedu Soup           - N2500");
    println!("E = Eba & Egusi Soup             - N2000");
    println!("W = White rice & Stew            - N2500");
    println!("===================================");
    
    let mut input = String::new();
    println!("Enter type of food (P, F, A, E, W): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let food_type = input.trim().to_uppercase();

    input.clear();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let quantity: i32 = input.trim().parse().expect("Please enter a valid number");

    let price_per_unit = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
           _ => {
            println!("Invalid food type entered.");
            return;
           }
    };
    let mut total = price_per_unit * quantity;

    if total > 10000 {
        let discount = total as f64 * 0.05;
        total = (total as f64 - discount) as i32;
        println!("You got a 5% discount!");

    }
    println!("Total amount to pay: N{}", total);
    println!("Thank you for your order!");

    
}