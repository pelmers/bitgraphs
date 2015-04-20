use std::io;
use std::fs::File;
use bitgraphs::BitGraph;
use bitgraphs::graph::read_csv;

#[test]
fn test_read_csv() {
    let path = "tests/data/petersen.csv";
    if let Ok(file) = File::open(path) {
        if let Some(g) = read_csv(&mut io::BufReader::new(file)) {
            println!("{}", g.serialize_dot(None, None));
        } else {
            panic!("Could not read csv to graph.");
        }
    } else {
        panic!("Could not open file.");
    }
}

