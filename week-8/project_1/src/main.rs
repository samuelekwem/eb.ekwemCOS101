use std::collections::HashMap;
use std::io;

// Enum for career groups
#[derive(Debug, PartialEq, Eq, Hash)]
enum Career {
    OfficeAdmin,
    Academic,
    Lawyer,
    Teacher,
}

// Struct to represent a job → APS level entry
#[derive(Debug)]
struct ApsEntry {
    job_title: &'static str,
    aps_level: &'static str,
}

fn main() {
    // APS mapping table based on your project image
    let mut aps_table: HashMap<Career, Vec<ApsEntry>> = HashMap::new();

    aps_table.insert(
        Career::OfficeAdmin,
        vec![
            ApsEntry { job_title: "Intern", aps_level: "APS 1-2" },
            ApsEntry { job_title: "Administrator", aps_level: "APS 3-5" },
            ApsEntry { job_title: "Senior Administrator", aps_level: "APS 5-8" },
            ApsEntry { job_title: "Office Manager", aps_level: "EL 1-2" },
            ApsEntry { job_title: "Director", aps_level: "EL 2 10-13" },
            ApsEntry { job_title: "CEO", aps_level: "SES" },
        ],
    );

    aps_table.insert(
        Career::Academic,
        vec![
            ApsEntry { job_title: "Research Assistant", aps_level: "APS 3-5" },
            ApsEntry { job_title: "PhD Candidate", aps_level: "APS 5-8" },
            ApsEntry { job_title: "Post-Doc Researcher", aps_level: "EL 1-2" },
            ApsEntry { job_title: "Senior Lecturer", aps_level: "EL 2 10-13" },
            ApsEntry { job_title: "Dean", aps_level: "SES" },
        ],
    );

    aps_table.insert(
        Career::Lawyer,
        vec![
            ApsEntry { job_title: "Paralegal", aps_level: "APS 1-2" },
            ApsEntry { job_title: "Junior Associate", aps_level: "APS 3-5" },
            ApsEntry { job_title: "Associate", aps_level: "APS 5-8" },
            ApsEntry { job_title: "Senior Associate 1-2", aps_level: "EL 1-2" },
            ApsEntry { job_title: "Senior Associate 3-4", aps_level: "EL 2 10-13" },
            ApsEntry { job_title: "Partner", aps_level: "SES" },
        ],
    );

    aps_table.insert(
        Career::Teacher,
        vec![
            ApsEntry { job_title: "Placement", aps_level: "APS 1-2" },
            ApsEntry { job_title: "Classroom Teacher", aps_level: "APS 3-5" },
            ApsEntry { job_title: "Snr Teacher", aps_level: "APS 5-8" },
            ApsEntry { job_title: "Leading Teacher", aps_level: "EL 1-2" },
            ApsEntry { job_title: "Deputy Principal", aps_level: "EL 2 10-13" },
            ApsEntry { job_title: "Principal", aps_level: "SES" },
        ],
    );

    // User interaction
    println!("Public Service APS Level Checker");
    println!("Enter career (office, academic, lawyer, teacher):");

    let mut career_input = String::new();
    io::stdin().read_line(&mut career_input).expect("Failed to read input");
    let career_input = career_input.trim().to_lowercase();

    let career = match career_input.as_str() {
        "office" => Career::OfficeAdmin,
        "academic" => Career::Academic,
        "lawyer" => Career::Lawyer,
        "teacher" => Career::Teacher,
        _ => {
            println!("Unknown career type.");
            return;
        }
    };

    println!("Enter job title:");
    let mut job_input = String::new();
    io::stdin().read_line(&mut job_input).expect("Failed to read input");
    let job_input = job_input.trim().to_lowercase();

    // Search for the job in the APS table
    if let Some(entries) = aps_table.get(&career) {
        for entry in entries {
            if entry.job_title.to_lowercase() == job_input {
                println!("APS Level: {}", entry.aps_level);
                return;
            }
        }
    }

    println!("Job title not found for this career.");
}
