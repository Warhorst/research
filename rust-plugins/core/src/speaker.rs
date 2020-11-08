/// Public trait to be implemented in plugins. Just returns some string.
pub trait Speaker {
    fn speak(&self) -> String;
}