mod cpu_reader;

fn main() {
    println!("Starting CPU Proc Daemon");
    let mut procman = cpu_reader::ProcFileManager::new();
    procman.read_and_print();
}
