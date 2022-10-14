use std::thread;
use std::time::{Duration};
use std::io;
use chrono;
use std::sync::mpsc::{self, TryRecvError};
use cli_clipboard::{ClipboardContext, ClipboardProvider};




fn main(){

println!("reitnorF Timer 2.0");
    let date = chrono::Local::now().format("%-d %B %Y").to_string();
    let start_time = chrono::Local::now().format("%-H:%M").to_string();
    let (tx, rx) = mpsc::channel();
    let (txa, rxa) = mpsc::channel();
    let mut i = 1;
    let mut m = 0;
    let mut h = 0;
    
    thread::spawn(move || {
        
        let ctn:bool = true;
        
        while ctn {
            
            let term = console::Term::stdout();
            term.clear_screen().expect("failed clearing screen");
            println!("reitnorF timer 2.0");
            //println!("{:?}", chrono::offset::Local::now());

            let now = chrono::Local::now();
            println!("{}", now.format("%-d %B %Y, %-H:%M").to_string());

            println!("{} hour, {} minute, {} second ",h,m,i);
           
            thread::sleep(Duration::from_millis(1000));
            i = i + 1;
            if i == 60 {
                i = 0;
                m = m +1;
            }
            if m == 60{
                m = 0;  
                h = h +1;
            }

            match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Timer terminated.");
                let ticker_dur = format!("({} hour {} minute {} second)",h,m,i);
                txa.send(ticker_dur).unwrap();
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
        }
    });
    
    let end_time = chrono::Local::now().format("%-H:%M").to_string();
    
    //let start = Instant::now();

    io::stdin().read_line(&mut String::new()).unwrap();
    //let duration = start.elapsed();

    let _ = tx.send(());
    let received = rxa.recv().unwrap();

    let ticker = format!("[{}] {} - {} {}", date,start_time, end_time,received);


    println!("{}",ticker);
    //println!("{:?}", duration);

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(ticker.to_owned()).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();


    /*
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input);
    */

    


    


}
