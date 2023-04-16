use anyhow::anyhow;
use crossterm::tty::IsTty;
use crossterm::{cursor, style, terminal, QueueableCommand};
use std::io::{Stdin, Stdout, Write};

pub struct UI {
    stdout: Stdout,
    stdin: Stdin,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = std::io::stdout();

        if stdout.is_tty() {
            Ok(UI {
                stdout,
                stdin: std::io::stdin(),
            })
        } else {
            Err(anyhow!("not a tty"))
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        use terminal::{Clear, ClearType::All, EnterAlternateScreen};

        self.stdout
            .queue(EnterAlternateScreen)?
            .queue(Clear(All))?
            .queue(cursor::SetCursorStyle::BlinkingBlock)?
            .flush()?;

        let inner_res = self.read_execute_loop();
        let outer_res = self.exit();

        match (inner_res, outer_res) {
            (Ok(()), Ok(())) => Ok(()),
            (inner, Ok(())) => inner,
            (Ok(()), outer) => outer,
            (Err(inner), Err(outer)) => Err(outer.context(format!("Original error: {inner:#}"))),
        }
    }

    fn exit(&mut self) -> anyhow::Result<()> {
        self.stdout
            .queue(cursor::SetCursorStyle::DefaultUserShape)?
            .queue(terminal::LeaveAlternateScreen)?
            .flush()?;
        self.stdout.write_all(b"Until next time!")?;
        Ok(())
    }

    fn read_execute_loop(&mut self) -> anyhow::Result<()> {
        while self.read_and_execute()? {}
        Ok(())
    }

    fn read_and_execute(&mut self) -> anyhow::Result<bool> {
        let (columns, rows) = terminal::size()?;
        self.stdout
            .queue(cursor::MoveTo(0, rows - 2))?
            .queue(style::SetBackgroundColor(style::Color::DarkGreen))?;

        for _ in 0..columns {
            self.stdout.write(b"-")?;
        }

        self.stdout
            .queue(style::SetBackgroundColor(style::Color::Reset))?
            .queue(cursor::MoveTo(0, rows - 1))?
            .write_all(b"prompt> ")?;
        self.stdout.flush()?;

        let command = self.read_line()?;
        if command == "exit" {
            Ok(false)
        } else {
            self.execute_command(command)?;
            Ok(true)
        }
    }

    fn read_line(&self) -> anyhow::Result<String> {
        let mut response = String::new();
        self.stdin.read_line(&mut response)?;
        Ok(response.trim().to_string())
    }

    fn execute_command(&self, command: String) -> anyhow::Result<()> {
        todo!("{:?}", command);
    }
}
