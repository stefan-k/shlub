pub struct History {
    history: Vec<String>,
    pos: usize,
    length: usize,
    max_length: usize,
}

impl History {
    pub fn new() -> Self {
        History {
            history: vec![],
            pos: 0,
            length: 0,
            max_length: 5,
        }
    }

    pub fn backwards(&mut self) -> Option<String> {
        if self.pos < self.length {
            self.pos += 1;
            Some(self.history[self.length - self.pos].clone())
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<String> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(self.history[self.length - self.pos - 1].clone())
        } else {
            None
        }
    }

    pub fn push(&mut self, cmd: &str) -> &mut Self {
        // TODO: Don't push if previous command is equivalent
        self.history.push(cmd.to_owned());
        self.pos = 0;
        self.length += 1;
        if self.length > self.max_length {
            self.history.remove(0);
            self.length = self.max_length;
        }
        self
    }

    pub fn get_all(&self) -> String {
        self.history.join("\n")
    }
}
