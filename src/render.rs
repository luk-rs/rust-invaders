use std::error::Error;
use std::io;
use std::io::{Stdout, Write};
use crossterm::{ExecutableCommand, QueueableCommand, terminal};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crate::frame::Frame;

pub fn setup_terminal() -> Result<Stdout,Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(stdout)
}

pub fn clear_terminal(stdout: &mut Stdout) -> Result<(),Box<dyn Error>>{
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}


pub fn render(stdout: &mut Stdout, last_frame:&Frame, current_frame:&Frame, force:bool){
    if force {
        stdout.queue(SetBackgroundColor(Color::Green)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x,col) in current_frame.iter().enumerate(){
        for (y,c) in col.iter().enumerate(){
            if *c != last_frame[x][y] || force{
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *c);
            }
        }
    }

    stdout.flush().unwrap();

}
