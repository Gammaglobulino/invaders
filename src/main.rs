use invaders::player::Player;
use invaders::invaders::Invaders;
use invaders::render;
use rusty_audio::Audio;
use std::error::Error;
use std::io;
use crossterm::ExecutableCommand;
use crossterm::terminal;
use crossterm::event::{Event,KeyCode};
use crossterm::event;
use crate::terminal::{EnterAlternateScreen,LeaveAlternateScreen};
use crossterm::cursor::{Hide,Show};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::thread;
use invaders::frame;
use invaders::frame::Drawable;



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

    //Render loop in a separate thread

    let (render_tx,render_rx) =mpsc::channel();
    let render_handle=thread::spawn(move || {
       let mut last_frame=frame::new_frame();
       let mut stdout=io::stdout(); 
       render::render(&mut stdout, &last_frame, &last_frame, true);
       loop{
           
           let cur_frame=match render_rx.recv(){
               Ok(x) => x,
               Err(_) => break,
           };
           render::render(&mut stdout, &last_frame, &cur_frame, false);
           last_frame=cur_frame;
       }
    });

    // Game Lopp
    let mut player=Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop{
        // Per-frame init
        let delta=instant.elapsed();
        instant=Instant::now();
        let mut cur_frame=frame::new_frame();
        //input
        while event::poll(Duration::default())?{
            
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter =>{
                        if player.shoot(){
                            audio.play("pew");
                        }
                    }
                    _=> {}
                }

            }
        }
        // game updates
        player.update(delta);
        if invaders.update(delta){
            audio.play("move");
        }

        // Draw & render
        let drawables:Vec<&dyn Drawable> =vec![&player,&invaders];
        for drawable in drawables{
            drawable.draw(&mut cur_frame);
        }

        let _ = render_tx.send(cur_frame);
        thread::sleep(Duration::from_micros(1));
    }

    //Cleanup

    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}
