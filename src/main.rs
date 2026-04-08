mod cpu_reader;
mod mem_reader;
use zbus::object_server::SignalEmitter;
use zbus::{connection::Builder, interface, zvariant::Type};

pub struct SysManager;

#[interface(name = "org.vivicado.SysInfo")]
impl SysManager {
    #[zbus(signal)]
    async fn stats_updated(
        signal_ctxt: &SignalEmitter<'_>,
        user: Vec<i64>,
        system: Vec<i64>,
    ) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting CPU Proc Daemon");
    let cpu_frequency = 50;
    let mut mem_ticker = tokio::time::interval(std::time::Duration::from_millis(200));
    let mut cpu_ticker = tokio::time::interval(std::time::Duration::from_millis(cpu_frequency));
    let mut mem_reader = mem_reader::MemInfo::new();
    let mut cpu_reader = cpu_reader::ProcFileManager::new(cpu_frequency as i32, 8);
    // setup connection
    // 1. Create the backend manager
    let sys_manager = SysManager;

    let conn = Builder::session()?
        .name("org.vivicado.Daemon")?
        .serve_at("/org/vivicado/SysInfo", sys_manager)?
        .build()
        .await?;

    let interface_ref = conn
        .object_server()
        .interface::<_, SysManager>("/org/vivicado/SysInfo")
        .await?;

    let signal_emitter = interface_ref.signal_emitter();

    loop {
        tokio::select! {
            _ = mem_ticker.tick() => {
                mem_reader.read_mem_file();
                mem_reader = tokio::task::spawn_blocking(move || {
                    mem_reader.read_mem_file();
                    mem_reader
                }).await?;
            },
            _ = cpu_ticker.tick() => {
                cpu_reader = tokio::task::spawn_blocking(move || {
                    cpu_reader.read_step();
                    cpu_reader
                }).await?;
                let mut user: Vec<i64> = Vec::new();
                let mut system: Vec<i64> = Vec::new();
                for cpu in cpu_reader.get_cpu_info().into_iter() {
                    user.push(cpu.user);
                    system.push(cpu.system);
                }

                SysManager::stats_updated(signal_emitter, user, system).await?;
            }
        }
    }
}
