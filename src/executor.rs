use crate::buffer::CFRBuffer;
use crate::painter::CFRPainter;

#[derive(Debug, Clone)]
pub struct CommandExecutorState {
    pub commands: String,
    pub index: usize,
    pub block_starts: Vec<usize>,
}

/// The `CommandExecutor` struct represents an executor for a set of commands.
/// It keeps track of the current state, buffer, and painter.
#[derive(Debug)]
pub struct CommandExecutor<'a> {
    pub state: CommandExecutorState,
    pub buffer: &'a mut CFRBuffer,
    pub painter: CFRPainter,
}

impl<'a> CommandExecutor<'a> {
    /// Creates a new `CommandExecutor` instance.
    ///
    /// # Arguments
    ///
    /// * `commands` - A string containing the commands to be executed.
    /// * `buffer` - A mutable reference to the `CFRBuffer` instance.
    ///
    /// # Returns
    ///
    /// A new `CommandExecutor` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::{CFRBuffer, CommandExecutor};
    ///
    /// let mut buffer = CFRBuffer::new(256, 256);
    /// let mut executor = CommandExecutor::new("[CFRS]".to_string(), &mut buffer);
    /// if let Ok(_) = executor.run() {
    ///     println!("Commands executed successfully");
    ///     // Do something with the buffer
    ///     // ...
    /// }
    /// ```
    pub fn new(commands: String, buffer: &'a mut CFRBuffer) -> Self {
        let mut painter = CFRPainter::new();
        painter.x = (buffer.width - 1) / 2;
        painter.y = (buffer.height - 1) / 2;

        Self {
            state: CommandExecutorState {
                commands,
                index: 0,
                block_starts: Vec::new(),
            },
            buffer,
            painter,
        }
    }

    /// Returns the current position of the painter.
    ///
    /// # Returns
    ///
    /// A tuple containing the x and y coordinates of the painter.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::{CFRBuffer, CommandExecutor};
    ///
    /// let mut buffer = CFRBuffer::new(256, 256);
    /// let mut executor = CommandExecutor::new("[CFRS]".to_string(), &mut buffer);
    /// let position = executor.position();
    /// assert_eq!(position, (127, 127));
    /// ```
    pub fn position(&self) -> (u32, u32) {
        (self.painter.x, self.painter.y)
    }

    /// Executes the next step in the command sequence.
    ///
    /// # Returns
    ///
    /// - `Ok((bool, &CFRBuffer))` if the step was executed successfully. The boolean value indicates whether the executor should sleep after the step, and the reference to the `CFRBuffer` is returned.
    /// - `Err(&'static str)` if an error occurred during execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::{CFRBuffer, CommandExecutor};
    ///
    /// let mut buffer = CFRBuffer::new(256, 256);
    /// let mut executor = CommandExecutor::new("[CFRS]".to_string(), &mut buffer);
    /// while let Ok((sleep, buffer)) = executor.step() {
    ///     println!("Step executed successfully");
    ///     if sleep {
    ///         std::thread::sleep(std::time::Duration::from_millis(20));
    ///     }
    ///     // Do something with the buffer
    ///     // ...
    /// }
    ///
    /// // Check if the end of the commands was reached
    /// if let Err(e) = executor.step() {
    ///     assert_eq!(e, "End of commands");
    /// }
    /// ```
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
                self.state.block_starts.push(self.state.index + 1);
            }
            ']' => {
                if let Some(block_start) = self.state.block_starts.pop() {
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

    /// Executes all the steps in the command sequence.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all steps were executed successfully.
    /// - `Err(&'static str)` if an error occurred during execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::{CFRBuffer, CommandExecutor};
    ///
    /// let mut buffer = CFRBuffer::new(256, 256);
    /// let mut executor = CommandExecutor::new("[CFRS]".to_string(), &mut buffer);
    /// if let Ok(_) = executor.run() {
    ///     println!("Commands executed successfully");
    ///     // Do something with the buffer
    ///     // ...
    /// } else {
    ///     println!("Error executing commands");
    /// }
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
