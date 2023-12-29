use std::fs;
use std::io;
use std::path;

pub fn read_file<P: AsRef<path::Path>>(file_name: P) -> io::Lines<io::BufReader<fs::File>>
{
    let file: fs::File = match fs::File::open(file_name)
    {
        Ok(file) => file,
        Err(_) => panic!("Error: Failed to open input file"),
    };

    return io::BufRead::lines(io::BufReader::new(file));
}
