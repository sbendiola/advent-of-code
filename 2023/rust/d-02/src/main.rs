use std::process::Command;

pub(crate) fn main() {
    ["part_01", "part_02"].iter().for_each(|bin| {
        let output = Command::new("cargo")
            .args(&["run", "--package", "d-02", "--bin", bin])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
}
