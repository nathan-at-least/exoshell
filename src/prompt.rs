use crossterm::event::EventStream;
use std::io::{Stdout, Write};

pub(crate) async fn read(
    events: &mut EventStream,
    stdout: &mut Stdout,
    prompt: &str,
) -> anyhow::Result<String> {
    use crossterm::event::{Event::Key, KeyEvent, KeyEventKind};
    use futures::StreamExt;

    display_prompt(stdout, prompt)?;

    let mut response = String::new();
    while let Some(Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    })) = events.next().await.transpose()?
    {
        use crossterm::event::KeyCode::{Char, Enter};

        match code {
            Enter => {
                break;
            }
            Char(c) => {
                response.push(c);

                // Display it on screen:
                let mut bytes = [0u8; 4];
                c.encode_utf8(&mut bytes);
                stdout.write_all(&bytes[..c.len_utf8()])?;
                stdout.flush()?;
            }
            _ => {}
        }
    }

    Ok(response)
}

pub(crate) fn display_prompt(stdout: &mut Stdout, prompt: &str) -> anyhow::Result<()> {
    use crossterm::{cursor, style, terminal, QueueableCommand};

    let (_, rows) = terminal::size()?;
    stdout
        .queue(style::SetBackgroundColor(style::Color::Reset))?
        .queue(cursor::MoveTo(0, rows - 1))?
        .write_all(prompt.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
