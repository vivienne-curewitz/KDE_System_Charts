use std::fs::File;
use std::io::{Read, Seek};

pub struct MemInfo {
    total: i64,
    free: i64,
    available: i64,
    mem_file: File,
}

impl MemInfo {
    pub fn new() -> MemInfo {
        MemInfo {
            total: 0,
            free: 0,
            available: 0,
            mem_file: File::open("/proc/meminfo").expect("Failed to open /proc/meminfo"),
        }
    }

    pub fn read_mem_file(&mut self) {
        if self.mem_file.rewind().is_err() {
            self.mem_file = File::open("/proc/meminfo").expect("Failed to reopen /proc/meminfo");
        }
        let mut data: String = String::new();
        self.mem_file
            .read_to_string(&mut data)
            .expect("Failed to read file /proc/meminfo");
        let lines: Vec<Vec<String>> = data
            .lines()
            .map(|line| line.split_whitespace().map(String::from).collect())
            .collect();
        self.total = lines[0][1].parse().unwrap();
        self.free = lines[1][1].parse().unwrap();
        self.available = lines[2][1].parse().unwrap();
    }

    pub fn read_loop(&mut self) {
        loop {
            self.read_mem_file();
        }
    }

    pub fn print_data(&mut self) {
        println!(
            "Available: {:.2} Total: {:.2} :: {:.2}%",
            self.available as f64 / 10e5,
            self.total as f64 / 10e6,
            self.available as f64 / self.total as f64 * 100.0
        );
    }
}
