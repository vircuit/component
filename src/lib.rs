pub trait Component {
    fn get_name(&self) -> &str;
    fn run(&mut self) -> anyhow::Result<()>;
    fn stop(&mut self) -> anyhow::Result<()>;
}
