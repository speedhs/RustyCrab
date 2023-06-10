
use clap::Parser;
use std::io::BufRead;

#[derive(Parser)]
struct Cli{
    pattern : String,
    path : std::path::PathBuf,
} 

fn main() {
    let args =  Cli::parse();
    println!("{}",&args.pattern);
    let file = std::fs::File::open(&args.path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines(){
        if let Ok(line) = line {
            if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    
    }
}
