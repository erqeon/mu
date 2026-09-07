use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind, read};
use crossterm::execute;
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, size};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

    execute!(stdout, EnableMouseCapture)?;
    execute!(stdout, Hide)?;

    println!("good luck, to exit, use: q.\r");
    stdout.flush()?;

    let mut alphabet: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", 
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", 
        " ", "{", "}", "(", ")", "'", "'", ",", ".", ":", "!", "?", "/", "*", ";", "=", "#", "[BACKSPACE]", "[ENTER]"
    ];

    let mut current_index: usize = 0;
    let mut code_buffer = String::new();

    let (_width, height) = size()?;
    loop {
        match read()? {
            Event::Key(key_event) => {
                stdout.flush()?;
                    
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }

            Event::Mouse(mouse_event) => {
                match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        current_index = (current_index + alphabet.len() - 1) % alphabet.len();
                        stdout.flush()?;
                    }
                    MouseEventKind::ScrollDown => {
                        current_index = (current_index + 1) % alphabet.len();
                        stdout.flush()?;
                    }
                    MouseEventKind::Down(MouseButton::Right) => {
                        let selected = alphabet[current_index];

                        if selected == "[ENTER]" {
                            code_buffer.push('\n');
                            code_buffer.push('\r');
                        } else if selected == "[BACKSPACE]" {
                            code_buffer.pop();
                        } else {
                            code_buffer.push_str(selected);
                        }

                        stdout.flush()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        print!("{}", code_buffer);

        execute!(stdout, MoveTo(0, height - 1))?;

        for (i, &c) in alphabet.iter().enumerate() {
            if i == current_index {
                print!("[{}]", c);
            } else {
                print!("{}", c);
            }
        }

        stdout.flush()?;
    }

    execute!(stdout, DisableMouseCapture)?;
    execute!(stdout, Show)?;
    disable_raw_mode()?;
    println!("Good bye! exit\r");
    Ok(())
}
