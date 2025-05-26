//////////////////////////////////////////////////////
// File ini berisi implementasi algoritma TSP
// yang menggunakan pendekatan DP.
//////////////////////////////////////////////////////

use crate::graph::Graph;

const INFINITY: u32 = u32::MAX;

pub fn solve_tsp(graph: &Graph, start_node: usize) -> (Option<Vec<usize>>, Option<u32>) {
    let n = graph.get_num_cities();

    // beberapa edge case
    if n == 0 {
        return (None, None); // graf kosong
    }
    if start_node >= n {
        return (None, None); // start_node invalid
    }
    if n == 2 {
        if let Some(weight) = graph.get_weight(0, 1) {
            return (Some(vec![0, 1, 0]), Some(weight * 2)); 
        } else {
            return (None, None); // Tidak ada jalur antara kedua kota
        }
    }

    // inisialisasi tabel DP
    // dp[mask][i] = ongkos dari rute terpendek yang dimulai dari `start_node` kemudian mengunjungi semua kota dalam `mask` tepat sekali, dan berakhir di kota `i`
    // `mask` adalah bitmask di mana bit ke-k disetel jadi 1 kalau kota ke-k telah dikunjungi.
    let mut dp = vec![vec![INFINITY; n]; 1 << n];

    // inisialisasi tabel parent
    // parent[mask][i] = kota sebelumnya sebelum kota `i` dalam rute terpendek yang berakhir di kota `i` untuk subset `mask`.
    // Ini digunakan untuk merekonstruksi rute setelah DP selesai.
    let mut parent = vec![vec![0_usize; n]; 1 << n];

    // Base case: rute yang hanya mengunjungi start_node.
    for j in 0..n {
        if j == start_node {
            continue;
        }
        let mask = (1 << start_node) | (1 << j);
        if let Some(weight) = graph.get_weight(start_node, j) {
            dp[mask][j] = weight;
            parent[mask][j] = start_node;
        }
    }

    // Iterasi untuk setiap ukuran subset dari 3 hingga n (ukuran subset)
    // Kita mulai dari 3 karena kita sudah menangani kasus n=2 di awal
    for subset_size in 3..=n {
        for mask in 1_usize..(1 << n) {
            if mask.count_ones() != subset_size as u32 {
                continue; 
            }

            if (mask >> start_node) & 1 == 0 {
                continue;
            }

            for j in 0..n {
                if j == start_node {
                    continue;
                }
                if (mask >> j) & 1 == 0 {
                    continue;
                }

                // `prev_mask_for_j`= S - {j} di algoritma Held-Karp
                let prev_mask_for_j = mask ^ (1 << j);
                
                // `k` adalah kota sebelumnya yang dikunjungi sebelum `j` dalam rute terpendek.
                for k in 0..n {
                    if k == j {
                        continue;
                    }
                    if (prev_mask_for_j >> k) & 1 == 0 {
                        continue;
                    }

                    if dp[prev_mask_for_j][k] != INFINITY {
                        if let Some(weight_kj) = graph.get_weight(k, j) {
                            let new_cost = dp[prev_mask_for_j][k].saturating_add(weight_kj);
                            if new_cost < dp[mask][j] {
                                dp[mask][j] = new_cost;
                                parent[mask][j] = k;
                            }
                        }
                    }
                }
            }
        }
    }

    // mencari tur terpendek yang mengunjungi semua kota dan kembali ke start_node
    let final_mask = (1 << n) - 1; // mask yang menunjukkan semua kota telah dikunjungi
    let mut min_tour_cost = INFINITY;
    let mut last_node_of_tour = usize::MAX; 

    for j in 0..n {
        if j == start_node {
            continue;
        }
        if dp[final_mask][j] != INFINITY {
            if let Some(weight_j_start) = graph.get_weight(j, start_node) {
                let current_tour_cost = dp[final_mask][j].saturating_add(weight_j_start);
                if current_tour_cost < min_tour_cost {
                    min_tour_cost = current_tour_cost;
                    last_node_of_tour = j;
                }
            }
        }
    }

    // merekonstruksi rute jika ditemukan tur terpendek
    if min_tour_cost == INFINITY {
        (None, None) 
    } else {
        let mut path = Vec::with_capacity(n);
        let mut current_node = last_node_of_tour;
        let mut current_mask = final_mask;

        for _ in 0..(n - 1) {
            path.push(current_node);
            let prev_node = parent[current_mask][current_node];
            current_mask ^= 1 << current_node; // Remove current_node from mask.
            current_node = prev_node;
        }
        
        path.push(start_node); 
        path.reverse(); 

        (Some(path), Some(min_tour_cost))
    }
}