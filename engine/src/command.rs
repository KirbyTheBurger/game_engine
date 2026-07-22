use crate::Shared;

#[derive(Clone)]
pub enum Command {

}

#[derive(Clone)]
pub struct CommandQueue(Shared<Vec<Command>>);

impl CommandQueue {
    pub fn new() -> Self {
        Self(Shared::new(vec![]))
    }

    pub fn push(&self, cmd: Command) {
        self.0.get().push(cmd);
    }

    pub fn drain(&self) -> Vec<Command> {
        std::mem::take(&mut *self.0.get())
    }
}
