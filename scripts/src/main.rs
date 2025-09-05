use std::{error::Error, io};
use std::ops::Index;

#[derive(Debug)]
struct Day {
    date: u32,
    price: f32,
    tag: String,
    comment: String,
}

fn parse() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v = vec![];
    let mut current = 1;
    for result in rdr.records() {
        let record = result?;
        let d = Day{
            date: record.index(0).trim().parse::<u32>().unwrap_or(current),
            price: record.index(1).trim().replace(',', ".").parse::<f32>().unwrap_or(0.0),
            tag: record.index(2).trim().to_owned(),
            comment: record.index(3).trim().to_owned(),
        };
        if d.tag == "" { continue; }
        current = d.date;
        v.push(d);
    }
    println!("{:?}", v);
    Ok(())
}

fn main() {
    let _ = parse();
}