pub struct History {
    history: Vec<String>,
    pos: u32,
}

impl History {
    pub fn new() -> Self {
        History {
            history: vec![],
            pos: 0,
        }
    }

    pub fn push(&mut self, cmd: &str) -> &mut Self {
        self.history.push(cmd.to_owned());
        self
    }

    pub fn get_all(&self) -> String {
        self.history.join("\n")
    }
}
