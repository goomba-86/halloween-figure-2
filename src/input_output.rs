pub trait InputOutput {
    fn write(&self) -> ();
    fn read(&self) -> String;
}
