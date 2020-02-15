use std::time::{Instant};

mod handle_json_file;

fn main() -> std::io::Result<()> {
//    let start = Instant::now();
//    write_json_file::write_json_file()?;
//    let duration = start.elapsed();
//    println!("duration:{:?}", duration);

//    let start = Instant::now();
//    handle_json_file::read_file_serde();
//    let duration = start.elapsed();
//    println!("duration:{:?}", duration);

//    let start = Instant::now();
//    handle_json_file::regex();
//    let duration = start.elapsed();
//    println!("duration:{:?}", duration);

    let time = Instant::now();
    handle_json_file::read_csv();
    println!("all read_csv: {:?} secs", time.elapsed());
    Ok(())
}
