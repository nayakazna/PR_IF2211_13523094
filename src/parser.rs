/////////////////////////////////////////////////////////////////
// File ini berisi parser untuk membaca data graf dari file txt. 
/////////////////////////////////////////////////////////////////

use crate::graph::Graph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse_txt(file_path: &str) -> io::Result<Graph> {
    // buka dan baca file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // baca baris pertama (N)
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;

    // parsing jumlah kota
    let num_cities: usize = first_line.trim().parse().unwrap();

    // baca graf
    let mut graph = Graph::new(num_cities);
    for (i, line) in lines.enumerate() {
        // baca baris
        let line = line?;
        
        // parsing jarak antar kota
        let distances: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // tambahkan jarak ke graf
        graph.add_distances(i, distances);
    }
    Ok(graph)
}