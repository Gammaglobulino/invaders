use rusty_audio::Audio;
use std::error::Error;
use std::io;
use crossterm::ExecutableCommand;
use crossterm::terminal;
use crossterm::event::{Event,KeyCode};
use crossterm::event;
use crate::terminal::{EnterAlternateScreen,LeaveAlternateScreen};
use crossterm::cursor::{Hide,Show};
use std::time::Duration;

fn main() -> Result<(),Box<dyn Error>>{
    let mut audio=Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout =io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Lopp
    'gameloop: loop{
        //input
        while event::poll(Duration::default())?{
            
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _=> {}
                }

            }
        }
    }

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    
    Ok(())
}
