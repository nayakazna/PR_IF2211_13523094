# Travelling Salesman Problem (TSP) Solver in Rust
[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/nayakazna/PR_IF2211_13523094/blob/master/README.en.md)
[![id](https://img.shields.io/badge/lang-id-red.svg)](https://github.com/nayakazna/PR_IF2211_13523094/blob/master/README.md)

## Deskripsi
Repositori ini berisi implementasi algoritma untuk menyelesaikan masalah Travelling Salesman Problem (TSP) menggunakan bahasa pemrograman Rust dengan pendekatan pemrograman dinamis (*dynamic programming*). TSP adalah masalah klasik di bidang keinformatikaan yang bertujuan menemukan rute terpendek yang mengunjungi setiap kota tepat satu kali dan kembali ke kota asal sehingga dapat kita terjemahkan sebagai persoalan optimasi kombinatorial. 
- Tiap kota dipandang sebagai simpul dalam graf, dan rute antar kota sebagai sisi dengan bobot yang mewakili jarak antar kota tersebut. 
- Dalam program ini, graf direpresentasikan sebagai matriks ketetanggaan (*adjacency matrix*), dengan setiap elemen matriks `dist[i][j]` menunjukkan jarak antara kota `i` dan kota `j`. 
- Program ini menggunakan pendekatan pemrograman dinamis untuk menghitung jarak terpendek yang diperlukan untuk mengunjungi semua kota, sedikit terinspirasi dari algoritma *Held-Karp*.

## Konfigurasi
Input dari program ini, seperti tucil dan tubes pada mata kuliah ini, adalah berupa file `.txt` yang berisi matriks ketetanggaan. Format file input adalah sebagai berikut:

```
<jumlah_kota>
<matriks_ketetanggaan>
```

Contoh:

```
4
0 10 15 20
10 0 35 25
15 35 0 30
20 25 30 0
```

Konfigurasi `.txt` ini disimpan dalam direktori `test`.

## Kompilasi dan Eksekusi
0. Pastikan Rust sudah terinstal.
1. Clone repositori ini:
   ```bash
   git clone https://github.com/nayakazna/PR_IF2211_13523094.git
   ```
2. Masuk ke direktori utama:
   ```bash
    cd PR_IF2211_13523094
    ```
3. Compile dan jalankan program:
   ```bash
   cargo run
   ```
4. Enjoy :D