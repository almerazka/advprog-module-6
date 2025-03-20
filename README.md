**Tutorial 6 Pemrograman Lanjut (Advanced Programming) 2024/2025 Genap**
* Nama    : Muhammad Almerazka Yocendra
* NPM     : 2306241745
* Kelas   : Pemrograman Lanjut - A

<details>
    <summary><strong> Basic Knowledge about Web Servers 💡 </strong></summary> 
    
Web server adalah program yang menerima permintaan (_request_) dari client (_browser_) dan mengirimkan respon (_response_) sesuai permintaan tersebut.

Sebuah web server biasanya menggunakan dua protokol utama untuk berkomunikasi dengan browser :

**1. TCP (Transmission Control Protocol)**

> **TCP** adalah protokol dasar yang menangani bagaimana data dikirim antara dua komputer. **TCP** memastikan data yang sampai dalam urutan yang benar dan tidak ada yang hilang. Bisa dibilang **TCP** seperti pengantar paket yang memastikan barang sampai ke tujuan tanpa rusak.

**2. HTTP (Hypertext Transfer Protocol)**

> Sedangkan **HTTP** berjalan di atas **TCP** dan menentukan format permintaan dan balasan antara browser dan server. Browser menggunakan HTTP untuk meminta halaman web, sementara server membalas dengan data yang sesuai.

Jadi kesimpulannya **TCP** berfokus pada bagaimana data dikirim sedangkan **HTTP** berfokus pada apa yang dikirim (apa isinya, misal permintaan halaman web).
</details>

### Milestone 1: Single-Threaded Web Server
---
**1. Membuat Server yang Mendengarkan Koneksi TCP**

Hal pertama yang kita lakukan adalah _Server_ perlu mendengarkan koneksi yang masuk agar bisa menerima permintaan dari _browser_. Disini saya menggunakan `TcpListener` untuk menangani masalah ini

```rust
TcpListener::bind("127.0.0.1:7878")
```
  - Membuat server yang mendengarkan koneksi di alamat `127.0.0.1` pada port `7878`.
  - `unwrap()` digunakan agar program langsung berhenti jika terjadi error saat memulai server.

```rust   
for stream in listener.incoming()
```
  - Menunggu koneksi dari client.
  - `incoming()` mengembalikan iterator yang menghasilkan `TcpStream` setiap kali ada koneksi masuk.

```rust
let stream = stream.unwrap();
```    
  - Mengambil data dari koneksi.
  - `unwrap()` memastikan kita mendapatkan koneksi yang valid.
    
```rust
println!("Connection established!");
```     
  - Mencetak pesan di terminal setiap kali ada koneksi baru.


**2. Memproses dan Membaca _Request_ dari _Browser_**

Fungsi `handle_connection(stream);` digunakan untuk memproses isi dari koneksi yang masuk. Tanpa fungsi ini, server hanya tahu bahwa ada koneksi masuk tetapi tidak tahu apa yang diminta oleh browser seperti sebelumnya dimana kita hanya tahu ada koneksi masuk dengan mencetak _"Connection established!"_.
```rust
fn handle_connection(mut stream: TcpStream) {
```
  - Fungsi ini menerima **TcpStream** yang berisi data dari koneksi browser, _stream_ disini yaitu koneksi aktif antara _server_ dan _client_
    
```rust
let buf_reader = BufReader::new(&stream);
```
  - Membungkus _stream_ dengan **BufReader** agar kita bisa membaca data per baris dengan lebih mudah

```rust
let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
```
  - `.lines()` : Mengubah data menjadi iterator yang membaca per baris.
  - `.map(|result| result.unwrap())` : Mengambil teks dari setiap baris (mengabaikan error).
  - `.take_while(|line| !line.is_empty())` : Menghentikan pembacaan ketika menemukan baris kosong (menandakan akhir dari permintaan **HTTP**).
  - `.collect()` : Mengumpulkan semua baris ke dalam Vec<String>.

```rust
println!("Request: {:#?}", http_request);
```
  - Menampilkan isi permintaan **HTTP** di terminal dalam format yang mudah dibaca.
    
***Output detail request*** :
```rust
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "Connection: keep-alive",
    "Cache-Control: max-age=0",
    "sec-ch-ua: \"Chromium\";v=\"134\", \"Not:A-Brand\";v=\"24\", \"Google Chrome\";v=\"134\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",  
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: en-GB,en;q=0.9,id-ID;q=0.8,id;q=0.7,en-US;q=0.6",     
]
```