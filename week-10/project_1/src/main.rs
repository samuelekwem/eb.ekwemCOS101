// Define a struct for laptop information
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate cost for N units
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Create laptop entries
    let hp = Laptop { brand: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000 };

    // Quantity to be purchased from each brand
    let qty = 3;

    // Calculate total
    let total_cost =
        hp.cost(qty) +
        ibm.cost(qty) +
        toshiba.cost(qty) +
        dell.cost(qty);

    println!("Total cost for buying 3 laptops from each brand is: ₦{}", total_cost);
}
