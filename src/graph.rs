//////////////////////////////////////////////////////
// File ini berisi definisi dari struktur data Graph 
// yang merepresentasikan graf berbobot.
// saya mengasumsikan bahwa graf ini tidak berarah
// dan tidak memiliki bobot negatif
//////////////////////////////////////////////////////

pub struct Graph {
    adj_matrix: Vec<Vec<Option<u32>>>, 
    num_cities: usize,
}

impl Graph {
    // konstruktor
    pub fn new(num_cities: usize) -> Self {
        let adj_matrix = vec![vec![None; num_cities]; num_cities];
        Graph { adj_matrix, num_cities }
    }

    // menambahkan sisi antara dua kota dengan bobot tertentu
    pub fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
        if from < self.num_cities && to < self.num_cities {
            self.adj_matrix[from][to] = Some(weight);
            self.adj_matrix[to][from] = Some(weight); 
        }
    }

    // mendapatkan bobot sisi antara dua kota
    pub fn get_weight(&self, from: usize, to: usize) -> Option<u32> {
        if from < self.num_cities && to < self.num_cities {
            self.adj_matrix[from][to]
        } else {
            None
        }
    }

    pub fn get_num_cities(&self) -> usize {
        self.num_cities
    }
}

