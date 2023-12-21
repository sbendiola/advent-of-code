use std::process::Command;

fn main() {
    let current_file = "day-03";
    ["part_01", "part_02"].iter().for_each(|bin| {
        let output = Command::new("cargo")
            .args(&["run", "--package", "d-03", "--bin", bin])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!(
                        "Output: {} {}",
                        current_file,
                        String::from_utf8_lossy(&output.stdout)
                    );
                } else {
                    println!(
                        "Error: {} {}",
                        current_file,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                println!("Error: {} {}", current_file, e);
            }
        }
    });
}
