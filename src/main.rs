
use std::error::Error;
use std::sync::mpsc;
use std::{io, thread};
use std::time::{Duration, Instant};
use crossterm::{event};
use crossterm::event::{Event, KeyCode};

use invaders::audio::{clear_audio, setup_audio};
use invaders::{frame, render};
use invaders::frame::Drawable;
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render::{clear_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    //audio
    let mut audio = setup_audio();
    audio.play("startup");

    //terminal
    let mut stdout = setup_terminal()?;

    //render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameLoop: loop{
        //per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameLoop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    _ => {}
                }
            }
        }

        //updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }

        //draw and render
        let drawables:Vec<&dyn Drawable> = vec![&player,&invaders];
        for drawable in drawables.iter() {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //win or loose
        if invaders.exterminated(){
            audio.play("win");
            break 'gameLoop;
        }
        if invaders.reached_ship(){
            audio.play("lose");
            break 'gameLoop;
        }

    }


    //cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    clear_audio(&audio);
    clear_terminal(&mut stdout)?;

    Ok(())
}
