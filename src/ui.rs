use anyhow::anyhow;
use crossterm::{cursor, style, terminal, QueueableCommand};
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

pub struct UI {
    stdout: Stdout,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = crate::tty::get()?;
        Ok(UI { stdout })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let res = self.run_inner();
        if res.is_ok() {
            self.stdout.write_all(GOODBYE.as_bytes())?;
        }
        res
    }

    pub fn run_inner(&mut self) -> anyhow::Result<()> {
        use crate::cleanup::CleanupWith;

        self.stdout.write_all(WELCOME.as_bytes())?;
        self.setup()?;
        self.read_execute_loop().cleanup_with(self.exit())?;
        Ok(())
    }

    fn setup(&mut self) -> anyhow::Result<()> {
        use terminal::{Clear, ClearType::All, EnterAlternateScreen};

        terminal::enable_raw_mode()?;

        self.stdout
            .queue(EnterAlternateScreen)?
            .queue(Clear(All))?
            .queue(cursor::SetCursorStyle::BlinkingBlock)?
            .flush()?;
        Ok(())
    }

    fn exit(&mut self) -> anyhow::Result<()> {
        self.stdout
            .queue(cursor::SetCursorStyle::DefaultUserShape)?
            .queue(terminal::LeaveAlternateScreen)?
            .flush()?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn read_execute_loop(&mut self) -> anyhow::Result<()> {
        while self.read_and_execute()? {}
        Ok(())
    }

    fn read_and_execute(&mut self) -> anyhow::Result<bool> {
        use crate::prompt;

        let (columns, rows) = terminal::size()?;
        self.stdout
            .queue(cursor::MoveTo(0, rows - 2))?
            .queue(style::SetBackgroundColor(style::Color::DarkGreen))?;

        for _ in 0..columns {
            self.stdout.write(b"-")?;
        }

        let command = prompt::read(&mut self.stdout, "$ ")?;
        if command == "exit" {
            Ok(false)
        } else {
            self.execute_command(command)?;
            Ok(true)
        }
    }

    fn execute_command(&self, command: String) -> anyhow::Result<()> {
        Err(anyhow!("execute_command({command:?}) not yet implemented"))
    }
}
