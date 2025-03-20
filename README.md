**Tutorial 6 Pemrograman Lanjut (Advanced Programming) 2024/2025 Genap**
* Nama    : Muhammad Almerazka Yocendra
* NPM     : 2306241745
* Kelas   : Pemrograman Lanjut - A

<details>
    <summary><strong> Basic Knowledge 💡 </strong></summary> 
    
Web server adalah program yang menerima permintaan (_request_) dari client (_browser_) dan mengirimkan respon (_response_) sesuai permintaan tersebut.

Sebuah web server biasanya menggunakan dua protokol utama untuk berkomunikasi dengan browser :

**1. TCP (Transmission Control Protocol)**

> **TCP** adalah protokol dasar yang menangani bagaimana data dikirim antara dua komputer. **TCP** memastikan data yang sampai dalam urutan yang benar dan tidak ada yang hilang. Bisa dibilang **TCP** seperti pengantar paket yang memastikan barang sampai ke tujuan tanpa rusak.

**2. HTTP (Hypertext Transfer Protocol)**

> Sedangkan **HTTP** berjalan di atas **TCP** dan menentukan format permintaan dan balasan antara browser dan server. Browser menggunakan HTTP untuk meminta halaman web, sementara server membalas dengan data yang sesuai.

Jadi kesimpulannya **TCP** berfokus pada bagaimana data dikirim sedangkan **HTTP** berfokus pada apa yang dikirim (apa isinya, misal permintaan halaman web).

**Format HTTP Response** :
```css
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
- **HTTP-Version** : Versi protokol HTTP yang digunakan (misalnya, HTTP/1.1).
- **Status-Code** : Angka yang menunjukkan hasil request (200 OK jika sukses).
- **Reason-Phrase** : Penjelasan singkat tentang status _request_.
- **Headers** : Informasi tambahan seperti _Content-Length_, Content-Type__, dll.
- **Message-Body** : Isi dari halaman HTML yang dikirim ke browser.
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
  - Menerima dan menangani koneksi dari client.
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

Fungsi `handle_connection(stream);` digunakan untuk memproses isi dari koneksi yang masuk. Tanpa fungsi ini, server hanya mendeteksi bahwa ada koneksi masuk, tetapi tidak memahami isi permintaan **HTTP** yang dikirim oleh browser. 

Seperti sebelumnya, server hanya mencetak _"Connection established!"_ tanpa mengetahui apa yang sebenarnya diminta oleh klien. Dengan `handle_connection()`, server dapat membaca detail permintaan, seperti metode **HTTP** _(GET, POST)_, URL yang diminta, serta informasi tambahan seperti jenis browser dan format data yang diinginkan.

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
  - `.lines()` : Mengembalikan iterator yang menghasilkan setiap baris dalam input sebagai `Result<String, io::Error>`.
  - `.map(|result| result.unwrap())` : Mengambil teks dari setiap baris atau mengekstrak string dari **Result**      (mengabaikan error).
  - `.take_while(|line| !line.is_empty())` : Menghentikan pembacaan ketika menemukan baris kosong (menandakan akhir header dari permintaan **HTTP**).
  - `.collect()` : Memengumpulkan semua baris hasil pembacaan dari iterator menjadi satu koleksi dalam bentuk `Vec<String>`

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

### Milestone 2: Returning HTML
---
![Commit 2 screen capture](assets/images/commit2.png)
Sebelumnya, web server hanya menerima koneksi tetapi tidak mengirimkan respons yang bisa ditampilkan oleh browser. Sekarang, saya akan mengirimkan halaman HTML sebagai respons HTTP yang valid, sehingga browser dapat merendernya dengan benar.

Pada tahap ini, saya mengubah fungsi `handle_connection` kembali agar bisa mengirimkan halaman HTML.

**1. Membaca File **HTML** dan Menyiapkan Respons**
```rust
let status_line = "HTTP/1.1 200 OK";
let contents = fs::read_to_string("hello.html").unwrap();
let length = contents.len();
```
Disini
- `status_line`: Menentukan status **HTTP** sebagai 200 OK (berhasil).
- `fs::read_to_string("hello.html")`: Saya menggunakan `fs::read_to_string` karena saya ingin mengirimkan file HTML yakni `hello.html` sebagai bagian dari respons **HTTP**, dimana `fs::read_to_string` akan membaca file tersebut ke dalam string dan kemudian dikirimkan ke browser.
- `length`: Menghitung panjang isi HTML untuk dikirimkan dalam header _Content-Length_.

**2. Membentuk Respons HTTP**
```rust
let response = format!(
    "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
)
```
Format respon yang dibuat
```html
HTTP/1.1 200 OK
Content-Length: <panjang-konten>
\r\n
<konten-HTML>
```
- `status_line`: Menandakan bahwa request berhasil (HTTP/1.1 200 OK).
- `Content-Length`: Memberitahu browser <panjang-konten> yang dikirim.
- `{contents}`: Isi dari file HTML yang akan ditampilkan di browser.
- `\r\n\r\n` : Memisahkan header dengan body dalam format **HTTP**. Tanpa ini, browser tidak akan mengenali akhir dari header **HTTP**, sehingga respons bisa dianggap tidak valid.

**3. Mengirim Respons ke Browser**
```rust
stream.write_all(response.as_bytes()).unwrap();
```
Disini
- `.as_bytes()` : Mengubah string menjadi byte agar bisa dikirim melalui **TCP**.
- `write_all()` : Mengirim seluruh data ke klien (browser).
- `unwrap()` : Jika terjadi error, program akan berhenti dengan _panic_.

**4. Menambahkan File HTML untuk Respons yakni [hello.html](https://github.com/almerazka/advprog-module-6/blob/main/hello.html)