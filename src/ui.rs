use crossterm::event::EventStream;
use futures::stream::SelectAll;
use std::io::{Stdout, Write};
use tokio_childstream::ChildStream;

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

pub struct UI {
    stdout: Stdout,
    events: EventStream,
    childstreams: SelectAll<ChildStream>,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = crate::tty::get()?;
        let events = EventStream::new();
        let childstreams = SelectAll::default();
        Ok(UI {
            stdout,
            events,
            childstreams,
        })
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
        if command.trim().is_empty() {
            Ok(true)
        } else if command == "exit" {
            Ok(false)
        } else {
            self.execute_command(command).await?;
            Ok(true)
        }
    }

    async fn execute_command(&mut self, command: String) -> anyhow::Result<()> {
        use crate::{prompt, Command};

        match command.parse::<Command>() {
            Ok(cmd) => {
                let stream = cmd.spawn()?;
                self.childstreams.push(stream);
            }
            Err(e) => {
                prompt::display_prompt(&mut self.stdout, &format!("Error: {e}"))?;
            }
        }
        Ok(())
    }
}
