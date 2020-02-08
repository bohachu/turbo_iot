use std::time::{Instant};

mod write_json_file;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    write_json_file::write_json_file()?;
    let duration = start.elapsed();
    println!("duration:{:?}", duration);
    Ok(())
}
