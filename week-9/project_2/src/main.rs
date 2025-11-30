use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_no: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Vector containing student records as shown in the slide
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_no: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_no: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_no: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_no: "EEE10202002".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_no: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Display header in console
    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level");

    // Display student records
    for s in &students {
        println!("{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric_no, s.department, s.level);
    }

    // Write to file
    let mut file = File::create("students.txt")?;

    writeln!(file, "PAU SMIS\n")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level")?;

    for s in &students {
        writeln!(file, "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric_no, s.department, s.level)?;
    }

    println!("\nData successfully saved to 'students.txt'!");
    Ok(())
}
