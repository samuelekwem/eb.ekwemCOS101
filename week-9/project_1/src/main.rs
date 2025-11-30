use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Drink categories from the table
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Malta",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create / overwrite the file
    let mut file = File::create("drinks.txt")?;

    // Write the categories to the file
    writeln!(file, "Nigerian Breweries Drink Categories\n")?;

    writeln!(file, "Lager:")?;
    for item in &lager {
        writeln!(file, " - {}", item)?;
    }

    writeln!(file, "\nStout:")?;
    for item in &stout {
        writeln!(file, " - {}", item)?;
    }

    writeln!(file, "\nNon-Alcoholic:")?;
    for item in &non_alcoholic {
        writeln!(file, " - {}", item)?;
    }

    println!("File 'drinks.txt' successfully created!");
    Ok(())
}
