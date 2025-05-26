/////////////////////////////////////////////////////////////////
// File ini berisi parser untuk membaca data graf dari file txt. 
/////////////////////////////////////////////////////////////////

use crate::graph::Graph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse_txt(file_path: &str) -> io::Result<Graph> {
    // validasi path
    if !Path::new(file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File tidak ditemukan"));
    }
    if !file_path.ends_with(".txt") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"File harus berformat .txt",));
    }

    // buka dan baca file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // baca baris pertama (N)
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;
    if first_line.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Baris pertama tidak boleh kosong"));
    }

    // parsing jumlah kota
    let num_cities: usize = first_line.trim().parse().unwrap();
    if num_cities < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Jumlah kota minimal 2"));
    }

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

        // sedikit validasi
        if distances.len() != num_cities {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Baris {}: Jumlah jarak tidak sesuai dengan jumlah kota", i + 2),
            ));
        }
        if i >= num_cities {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Jumlah baris tidak sesuai dengan jumlah kota",
            ));
        }

        // tambahkan jarak ke graf
        graph.add_distances(i, distances);
    }

    Ok(graph)
}