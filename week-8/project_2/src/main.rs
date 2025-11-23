#[derive(Debug)]
struct Developer {
    name: String,
    years_experience: u32,
}

fn main() {
    // Compound data type: Vector of Structs
    let candidates = vec![
        Developer { name: "John Doe".to_string(), years_experience: 3 },
        Developer { name: "Amara Johnson".to_string(), years_experience: 7 },
        Developer { name: "Chinedu Okeke".to_string(), years_experience: 5 },
        Developer { name: "Fatima Musa".to_string(), years_experience: 10 },
        Developer { name: "Michael Smith".to_string(), years_experience: 6 },
    ];

    // Find developer with highest years of experience
    let most_experienced = candidates
        .iter()
        .max_by_key(|dev| dev.years_experience)
        .expect("No candidates available");

    println!("Most experienced developer:");
    println!("Name: {}", most_experienced.name);
    println!("Years of Experience: {}", most_experienced.years_experience);
}
