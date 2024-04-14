use crate::buffer::CFRBuffer;
use crate::painter::CFRPainter;

#[derive(Debug, Clone)]
pub struct CommandExecutorState {
    pub commands: String,
    pub index: usize,
    pub level: usize,
    pub block_starts: Vec<usize>,
}

#[derive(Debug)]
pub struct CommandExecutor<'a> {
    pub state: CommandExecutorState,
    pub buffer: &'a mut CFRBuffer,
    pub painter: CFRPainter,
}

impl<'a> CommandExecutor<'a> {
    pub fn new(commands: String, buffer: &'a mut CFRBuffer) -> Self {
        let mut painter = CFRPainter::new();
        painter.x = buffer.width / 2;
        painter.y = buffer.height / 2;

        Self {
            state: CommandExecutorState {
                commands,
                index: 0,
                level: 0,
                block_starts: Vec::new(),
            },
            buffer,
            painter,
        }
    }

    pub fn position(&self) -> (u32, u32) {
        (self.painter.x, self.painter.y)
    }

    pub fn step(&mut self) -> Result<(bool, &CFRBuffer), &'static str> {
        if self.state.index >= self.state.commands.len() {
            return Err("End of commands");
        }

        let mut sleep = false;
        let c = self.state.commands.chars().nth(self.state.index).unwrap();
        match c {
            'C' => {
                self.painter.change_color();
            }
            'F' => {
                self.painter.move_forward_and_draw(self.buffer);
            }
            'R' => {
                self.painter.rotate();
            }
            'S' => {
                sleep = true;
            }
            '[' => {
                self.state.block_starts.push(self.state.index);
                self.state.level += 1;
            }
            ']' => {
                if let Some(block_start) = self.state.block_starts.pop() {
                    self.state.level -= 1;
                    self.state
                        .commands
                        .replace_range(self.state.index..=self.state.index, "|");
                    self.state.index = block_start;
                    return Ok((sleep, self.buffer));
                } else {
                    return Err("Unmatched ]");
                }
            }
            '|' => {
                self.state
                    .commands
                    .replace_range(self.state.index..=self.state.index, "]");
            }
            _ => {}
        }

        self.state.index += 1;
        Ok((sleep, self.buffer))
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            match self.step() {
                Ok(_) => {}
                Err(e) => {
                    if e == "End of commands" {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}
