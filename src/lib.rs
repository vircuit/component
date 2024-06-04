pub trait Component {
    fn get_name(&self) -> &str;
    fn run(&mut self) -> anyhow::Result<()>;
    fn stop(&mut self) -> anyhow::Result<()>;
}

pub fn connect<SIGNAL, SLOT, ARGS>(signal: SIGNAL, slot: SLOT) -> std::thread::JoinHandle<()>
where
    SIGNAL: FnOnce(std::sync::mpsc::SyncSender<ARGS>) + Send + 'static,
    SLOT: Fn(&ARGS) -> anyhow::Result<()> + Send + 'static,
    ARGS: Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(100);
    signal(tx);

    std::thread::spawn(move || {
        for args in rx {
            if let Err(e) = slot(&args) {
                eprintln!("Error: {:?}", e);
            }
        }
    })
}
