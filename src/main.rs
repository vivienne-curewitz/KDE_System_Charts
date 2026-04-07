mod cpu_reader;
mod mem_reader;

fn main() {
    println!("Starting CPU Proc Daemon");
    // let mut procman = cpu_reader::ProcFileManager::new();
    // procman.read_and_print_circle_buffer(8, 20);
    let mut procmem = mem_reader::MemInfo::new();
    loop {
        procmem.read_mem_file();
        procmem.print_data();
    }
}
