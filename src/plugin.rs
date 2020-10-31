pub trait Plugin {
    fn setup(&self);
    fn run(&self);
    fn teardown(&self);
}