use std::fs::File;
use std::io::{Read, Seek};

#[derive(Clone)]
pub struct CpuInfo {
    user: i64,
    nice: i64,
    system: i64,
    idle: i64,
    iowait: i64,
    irq: i64,
    softirq: i64,
}

impl CpuInfo {
    pub fn empty() -> CpuInfo {
        CpuInfo {
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
        }
    }
    pub fn simple_delta(&self, other: &CpuInfo) -> CpuInfo {
        CpuInfo {
            user: (self.user - other.user).abs(),
            nice: (self.nice - other.nice).abs(),
            system: (self.system - other.system).abs(),
            idle: (self.idle - other.idle).abs(),
            iowait: (self.iowait - other.iowait).abs(),
            irq: (self.irq - other.irq).abs(),
            softirq: (self.softirq - other.softirq).abs(),
        }
    }
}

pub struct CpuProcBuffer {
    position: usize,
    size: usize,
    ring: Vec<CpuInfo>,
}

impl CpuProcBuffer {
    fn new(isize: usize) -> CpuProcBuffer {
        CpuProcBuffer {
            position: 0,
            size: isize,
            ring: vec![CpuInfo::empty(); isize],
        }
    }

    fn insert(&mut self, ci: CpuInfo) {
        self.ring[self.position] = ci;
        self.position = (self.position + 1) & self.size;
    }

    fn delta(&mut self) -> CpuInfo {
        let current = &self.ring[self.position];
        let wrap = &self.ring[(self.position + 1) % self.size];
        current.simple_delta(wrap)
    }
}

pub struct ProcFileManager {
    cpu_file: File,
}
pub fn string_to_cpu_info(input: String) -> Result<CpuInfo, &'static str> {
    let cpu_vals: Vec<&str> = input.split_whitespace().collect();
    if cpu_vals.len() != 11 {
        println!("Line: {}", input);
        println!("Size: {}", cpu_vals.len());
        Err("Line returned Wrong size {}")
    } else {
        Ok(CpuInfo {
            user: cpu_vals[1].parse::<i64>().unwrap(),
            nice: cpu_vals[2].parse().unwrap(),
            system: cpu_vals[3].parse().unwrap(),
            idle: cpu_vals[4].parse().unwrap(),
            iowait: cpu_vals[5].parse().unwrap(),
            irq: cpu_vals[6].parse().unwrap(),
            softirq: cpu_vals[7].parse().unwrap(),
        })
    }
}
impl ProcFileManager {
    pub fn new() -> ProcFileManager {
        let cpu_file = File::open("/proc/stat").expect("Could not open stat");
        ProcFileManager { cpu_file }
    }
    pub fn get_cpu_step(&mut self) -> Vec<String> {
        //match set.cpu_file.seek(0) {
        if self.cpu_file.rewind().is_err() {
            self.cpu_file = File::open("/proc/stat").expect("Proc File could not be opened");
        }
        let mut data = String::new();
        match self.cpu_file.read_to_string(&mut data) {
            Ok(_) => {}
            Err(e) => panic!("Error {}", e),
        }
        let output: Vec<String> = data
            .lines() // 1. Split into an iterator of lines (&str)
            .filter(|line| line.starts_with("cpu")) // 2. Only keep lines starting with "cpu"
            .map(String::from) // 3. Convert each &str into a String
            .collect();
        output
    }

    pub fn read_and_print(&mut self) {
        let mut step_diff: Vec<CpuInfo> = vec![CpuInfo::empty(); 9];
        let mut prev_step: Vec<CpuInfo> = self
            .get_cpu_step()
            .into_iter()
            .map(string_to_cpu_info)
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to read a cpu line");
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let mut step_data: Vec<CpuInfo> = self
                .get_cpu_step()
                .into_iter()
                .map(string_to_cpu_info)
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to read a cpu line");
            for i in 0..step_data.len() {
                step_diff[i] = step_data[i].simple_delta(&prev_step[i]);
            }
            prev_step = step_data;
            let ut: i64 = step_diff[1].user + step_diff[1].nice;
            let st: i64 = step_diff[1].system;
            // just print cpu 1 for now
            println!("CPU 1: User: {} System: {}", ut, st);
        }
    }
}
