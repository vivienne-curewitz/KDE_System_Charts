use std::fs::File;
use std::io::{Read, Seek};
use zbus::zvariant::Type;

#[derive(Clone, Type, Debug, serde::Serialize, serde::Deserialize)]
pub struct SimpleCpuInfo {
    pub user: i64,
    pub system: i64,
}

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
    pub fn delta(&self, other: &CpuInfo) -> CpuInfo {
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

    pub fn simple_delta(&self, other: &CpuInfo) -> SimpleCpuInfo {
        let delta = self.delta(other);
        let total = delta.user
            + delta.nice
            + delta.system
            + delta.idle
            + delta.iowait
            + delta.irq
            + delta.softirq;
        SimpleCpuInfo {
            user: (100.0 * (delta.user + delta.nice) as f64 / total as f64).round() as i64,
            system: (100.0 * delta.system as f64 / total as f64).round() as i64,
        }
    }
}

#[derive(Clone)]
pub struct CpuProcBuffer {
    position: usize,
    size: usize,
    ring: Vec<CpuInfo>,
}

impl CpuProcBuffer {
    fn new(isize: usize) -> CpuProcBuffer {
        CpuProcBuffer {
            position: isize - 1,
            size: isize,
            ring: vec![CpuInfo::empty(); isize],
        }
    }

    fn insert(&mut self, ci: CpuInfo) {
        self.position = (self.position + 1) % self.size;
        self.ring[self.position] = ci;
    }

    fn delta(&self) -> SimpleCpuInfo {
        let current = &self.ring[self.position];
        let wrap = &self.ring[(self.position + 1) % self.size];
        current.simple_delta(wrap)
    }
}

pub struct ProcFileManager {
    cpu_file: File,
    cpu_rings: Vec<CpuProcBuffer>,
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
    pub fn new(frequency: i32, num_cpus: usize) -> ProcFileManager {
        let cpu_file = File::open("/proc/stat").expect("Could not open stat");
        ProcFileManager {
            cpu_file,
            cpu_rings: vec![CpuProcBuffer::new(1000 / frequency as usize); num_cpus + 1],
        }
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

    pub fn read_step(&mut self) {
        for (proc_num, step_data) in self
            .get_cpu_step()
            .into_iter()
            .map(string_to_cpu_info)
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to read a cpu line")
            .into_iter()
            .enumerate()
        {
            self.cpu_rings[proc_num].insert(step_data);
        }
    }

    pub fn get_cpu_info(&mut self) -> Vec<SimpleCpuInfo> {
        let cpu_info = self.cpu_rings.iter().map(|x| x.delta()).collect();
        cpu_info
    }

    pub fn read_and_print_circle_buffer(&mut self, num_cpus: usize, frequency: i32) {
        let mut step_diff: Vec<CpuProcBuffer> =
            vec![CpuProcBuffer::new(frequency as usize); num_cpus + 1];
        let mut proc_num = 0;
        for init_cpu in self.get_cpu_step().into_iter().map(string_to_cpu_info) {
            let v2 = init_cpu.expect("Failed to read cpu line in init");
            step_diff[proc_num].insert(v2);
            proc_num += 1;
        }
        let mut step_count = 0;
        let sleep_time = std::time::Duration::from_millis(1000 / frequency as u64);
        loop {
            std::thread::sleep(sleep_time);
            proc_num = 0;
            for step_data in self
                .get_cpu_step()
                .into_iter()
                .map(string_to_cpu_info)
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to read a cpu line")
                .into_iter()
            {
                step_diff[proc_num].insert(step_data);
                proc_num += 1;
            }
            step_count += 1;
            if step_count >= frequency {
                let delta_ett = step_diff[0].delta();
                let ut: i64 = delta_ett.user;
                let st: i64 = delta_ett.system;
                // just print cpu 1 for now
                println!("CPU 1: User: {}% System: {}%", ut, st);
            }
        }
    }
}
