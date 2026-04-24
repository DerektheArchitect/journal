use std::io::{self, Write};
use std::fs::OpenOptions; 
use chrono::Local;

fn main() -> std::io::Result<()> {
    println!("Append to journal.");
    let mut reply = String::new();
    io::stdin().read_line(&mut reply).expect("Could not read stdin.");

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut log = OpenOptions::new().append(true).create(true).open("journal.txt")?;
    writeln!(log, "[{}] {}", timestamp, reply.trim_end())?;
    
    Ok(())
}
