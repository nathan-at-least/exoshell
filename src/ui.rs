use anyhow::anyhow;
use crossterm::event::EventStream;
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

pub struct UI {
    stdout: Stdout,
    events: EventStream,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = crate::tty::get()?;
        let events = EventStream::new();
        Ok(UI { stdout, events })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let res = self.run_inner().await;
        if res.is_ok() {
            self.stdout.write_all(GOODBYE.as_bytes())?;
        }
        res
    }

    async fn run_inner(&mut self) -> anyhow::Result<()> {
        use crate::cleanup::CleanupWith;
        use crate::screen;

        self.stdout.write_all(WELCOME.as_bytes())?;
        screen::setup(&mut self.stdout)?;
        self.read_execute_loop()
            .await
            .cleanup_with(screen::exit(&mut self.stdout))?;
        Ok(())
    }

    async fn read_execute_loop(&mut self) -> anyhow::Result<()> {
        while self.read_and_execute().await? {}
        Ok(())
    }

    async fn read_and_execute(&mut self) -> anyhow::Result<bool> {
        use crate::{prompt, status};

        status::display(&mut self.stdout)?;
        let command = prompt::read(&mut self.events, &mut self.stdout, "$ ").await?;
        if command == "exit" {
            Ok(false)
        } else {
            self.execute_command(command).await?;
            Ok(true)
        }
    }

    async fn execute_command(&self, command: String) -> anyhow::Result<()> {
        Err(anyhow!("execute_command({command:?}) not yet implemented"))
    }
}
