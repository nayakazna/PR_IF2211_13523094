// src/main.rs
mod graph;
mod parser;
mod tsp;
mod writer;

use crate::parser::parse_txt;
use crate::tsp::solve_tsp;
use crate::writer::write_output_to_file; 

use std::env;
use std::fmt::Write as FmtWrite;
use std::path::Path;
use std::process;
use std::time::Instant;

fn main() {
    // Ambil argumen dari command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Format: {} <nama_file_graf.txt>", args[0]);
        eprintln!("Contoh: {} graph1.txt (file disimpannya di ./test/graph1.txt)", args[0]);
        process::exit(1);
    }
    let file_name_arg = &args[1];
    let input_file_path = format!("./test/{}", file_name_arg);

    // Parsing file
    println!("Membaca graf dari: {}", input_file_path);
    let graph = match parse_txt(&input_file_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error saat parsing file '{}': {}", input_file_path, e);
            process::exit(1);
        }
    };
    println!("Graf berhasil dibaca. Jumlah kota: {}", graph.get_num_cities());

    // print matriks
    println!("Matriks adjacency:");
    let adj_matrix = graph.get_adjacency_matrix();
    for row in adj_matrix {
        // write u32::max as infinity
        let row_str = row.iter()
            .map(|&weight| match weight {
                Some(u32::MAX) => "∞".to_string(), 
                None => "∞".to_string(), 
                Some(w) => w.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\t");
        println!("{}", row_str);
    }

    // Menjalankan TSP
    let start_node = 0; //node awal, tapi sebenarnya karena tur yang dihasilkan adalah tur Hamiltonian, kita bisa mulai dari node mana saja
    println!("Menjalankan TSP dengan node awal: {}", start_node);

    let start_time = Instant::now();
    let (path_option, cost_option) = solve_tsp(&graph, start_node);
    let duration = start_time.elapsed();

    // Menyiapkan string output
    let mut output_string = String::new();
    writeln!(output_string, "=========================================").unwrap();
    writeln!(output_string, "            HASIL TSP SOLVER             ").unwrap();
    writeln!(output_string, "=========================================").unwrap();
    writeln!(output_string, "Input file: {}", input_file_path).unwrap();
    writeln!(output_string, "Jumlah kota: {}", graph.get_num_cities()).unwrap();
    writeln!(output_string, "Node awal: {}", start_node).unwrap();
    writeln!(output_string, "-----------------------------------------").unwrap();

    match (path_option, cost_option) {
        (Some(path), Some(cost)) => {
            let path_str = path.iter().map(|p_node| p_node.to_string()).collect::<Vec<String>>().join(" -> ");
            let full_path_display = if path.len() > 1 {
                format!("{} -> {}", path_str, start_node)
            } else if !path.is_empty() { 
                format!("{}", path_str)
            } else {
                "Path tidak valid".to_string()
            };

            writeln!(output_string, "Tur terpendek ditemukan!").unwrap();
            writeln!(output_string, "Biaya (cost): {}", cost).unwrap();
            writeln!(output_string, "Urutan kunjungan: {}", full_path_display).unwrap();
        }
        _ => {
            writeln!(output_string, "Tidak ditemukan tur TSP yang valid.").unwrap();
        }
    }
    writeln!(output_string, "-----------------------------------------").unwrap();
    writeln!(output_string, "Waktu eksekusi: {:.2?}", duration).unwrap();
    writeln!(output_string, "=========================================").unwrap();

    print!("{}", output_string);

    // Menulis output ke file
    let input_path_obj = Path::new(file_name_arg);
    let input_file_stem = input_path_obj.file_stem().unwrap_or_else(|| std::ffi::OsStr::new(file_name_arg)) .to_str().unwrap_or("unknown_graph");

    let output_dir = "./test/output";
    // Membuat direktori output jika belum ada
    if !Path::new(output_dir).exists() {
        if let Err(e) = std::fs::create_dir_all(output_dir) {
            eprintln!(
                "\nPeringatan: Gagal membuat direktori output '{}': {}",
                output_dir, e
            );
        }
    }

    let output_file_name = format!("{}/{}_solution.txt", output_dir, input_file_stem);

    match write_output_to_file(&output_file_name, &output_string) {
        Ok(()) => {
            println!(
                "\nSolusi juga telah ditulis ke file: {}",
                output_file_name
            );
        }
        Err(e) => {
            eprintln!(
                "\nError saat menulis solusi ke file '{}': {}",
                output_file_name, e
            );
        }
    }
}