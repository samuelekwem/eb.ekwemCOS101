use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Dataset 1: Names of Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Validate that vectors have equal lengths
    let len = commissioners.len();
    if ministries.len() != len || zones.len() != len {
        panic!("Dataset lengths do not match!");
    }

    // Print header in console
    println!("{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Commissioner", "Ministry", "Geo Zone");

    // Print merged records
    for i in 0..len {
        println!("{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    // Save results to file
    let mut file = File::create("merged_ministers.txt")?;

    writeln!(file, "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Commissioner", "Ministry", "Geo Zone")?;

    for i in 0..len {
        writeln!(file, "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )?;
    }

    println!("\nMerged file saved as 'merged_ministers.txt'!");
    Ok(())
}
