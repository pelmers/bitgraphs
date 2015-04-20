use std::io;
use std::fs::File;
use bitgraphs::BitGraph;
use bitgraphs::graph::read_csv;
use bitgraphs::utils;
use std::collections::HashMap;

#[test]
fn test_read_csv() {
    let path = "tests/data/petersen.csv";
    if let Ok(file) = File::open(path) {
        if let Some(g) = read_csv(&mut io::BufReader::new(file)) {
            let colors = utils::greedy_color(&g, (0..g.len()).collect());
            let mut color_map = HashMap::new();
            for i in 0..g.len() {
                color_map.insert(i, HashMap::new());
            }
            for (c,s) in colors.iter().enumerate() {
                for n in s.iter() {
                    color_map.get_mut(&n).unwrap().insert("class".to_string(), format!("{}", c));
                }
            }
            println!("{}", g.serialize_dot(Some(&color_map), None));
        } else {
            panic!("Could not read csv to graph.");
        }
    } else {
        panic!("Could not open file.");
    }
}

