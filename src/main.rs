extern crate serial;

use std::env;
use std::io;
use std::time::Duration;
use std::process;
use std::io::prelude::*;
use serial::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    let data = Arc::new(Mutex::new(0));
    let run = Arc::new(Mutex::new(true));
    let data_log = data.clone();
    let data_sc = data.clone();
    let run_log = run.clone();
    let run_sc = run.clone();
    let run_main = run.clone();

    let sc = thread::spawn(||serial_comms(data_sc, run_sc));
    let log = thread::spawn(||logging(data_log, run_log));



    sc.join().unwrap();
    log.join().unwrap();

}

fn serial_comms(data: Arc<Mutex<i32>>, run: Arc<Mutex<bool>>) -> io::Result<()> {
    let mut port = serial::open("/dev/ttyACM0").unwrap();
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud115200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));
    try!(port.set_timeout(Duration::from_millis(1000)));

    let mut running = true;
    while running {
        {
            let mut data = data.lock().unwrap();
            *data += 1;

            let hello = "Hello World".to_string();
            let mut recv: Vec<u8> = (0..50).collect();
            // let mut recv: Vec<u8> = Vec::with_capacity(100);
            // let mut buf: Vec<u8> = (0..5).collect();
            // let mut recv: Vec<u8> = (10..15).collect();
            println!("Sending {}", hello);
            try!(port.write(&hello.into_bytes()));
            try!(port.read(&mut recv));
            println!("Received {:?}", String::from_utf8(recv).unwrap());
            // println!("Sending {:?}", buf);
            // try!(port.write(&buf[..]));
            // try!(port.read(&mut recv));
            // println!("Received {:?}", recv);
        }
        thread::sleep(Duration::from_millis(500));
        {
            let run = run.lock().unwrap();
            running = *run;
        }
    }
    Ok(())
}
fn logging(data: Arc<Mutex<i32>>, run: Arc<Mutex<bool>>){
    let mut running = true;
    while running {
        {
            let mut data = data.lock().unwrap();
            *data += 1;
            // println!("{} Logging",*data);
        }
        thread::sleep(Duration::from_millis(500));
        {
            let run = run.lock().unwrap();
            running = *run;
        }
    }
}
