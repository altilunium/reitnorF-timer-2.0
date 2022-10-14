use std::thread;
use std::time::{Duration};
use std::io;
use chrono;
use std::sync::mpsc::{self, TryRecvError};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {


    let date = chrono::Local::now().format("%-d %B %Y").to_string();
    let start_time = chrono::Local::now().format("%-H:%M").to_string();
    println!("reitnorF timer 2.0");
    println!("{}, {}", date,start_time);
    println!("");



    let (tx, rx) = mpsc::channel();
    let (txa, rxa) = mpsc::channel();
    let mut i = 1;
    let mut m = 0;
    let mut h = 0;
    
    thread::spawn(move || {
        
        loop {
            //Update current elapsed time
            let term = console::Term::stdout();
            term.clear_last_lines(1).expect("");
            let stamp = format!("{} hour, {} minute, {} second ",h,m,i);
            term.write_line(&stamp).expect("");
           
            //Second to hour-minute-second conversion
            thread::sleep(Duration::from_millis(1000));
            i = i + 1;
            if i == 60 {i = 0;m = m +1;}
            if m == 60{m = 0;h = h +1;}

            //Wait for quit signal from main thread
            match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Timer terminated.");
                let ticker_dur = format!("({} hour {} minute {} second)",h,m,i);
                // Send back the time data to main thread
                txa.send(ticker_dur).unwrap();
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
        }
    });
    

    //Wait for quit signal from user
    io::stdin().read_line(&mut String::new()).unwrap();
    
    //Send quit signal to timer thread
    let _ = tx.send(());

    //Receive elapsed time data
    let received = rxa.recv().unwrap();

    //Get end time
    let end_time = chrono::Local::now().format("%-H:%M").to_string();

    //Generate report
    let ticker = format!("[{}] {} - {} {}", date,start_time, end_time,received);

    //Show report
    println!("{}",ticker);

    //Copy report to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(ticker.to_owned()).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

    


    


}
