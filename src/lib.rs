use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Job>, // Menambahkan sender untuk mengirim tugas ke Worker
}

type Job = Box<dyn FnOnce() + Send + 'static>; // Job akan menyimpan tugas (closure)

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("Number of threads must be greater than zero!");
        }

        let (sender, receiver) = mpsc::channel(); // Membuat channel untuk komunikasi
        
        let receiver = Arc::new(Mutex::new(receiver)); // Menggunakan Arc + Mutex agar bisa dibagi ke banyak Worker

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Mengirim receiver ke Worker dan mengclone Arc agar bisa digunakan oleh semua Worker
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f); // Membungkus closure dalam Box<Job>
        
        self.sender.send(job).unwrap(); // Mengirim tugas ke channel
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // Setiap Worker memiliki thread sendiri
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap(); // Mengambil tugas dari channel

            println!("Worker {id} got a job; executing."); // Log untuk melihat Worker yang menangani tugas

            job(); // Menjalankan tugas
        });

        Worker { id, thread }
    }
}
