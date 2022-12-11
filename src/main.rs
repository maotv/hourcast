use std::net::UdpSocket;

use chrono::{Local, DateTime, TimeZone, NaiveTime, Timelike, Duration};
use std::thread;

fn main() -> std::io::Result<()> {

    let socket = UdpSocket::bind("0.0.0.0:12412")?;
    socket.set_broadcast(true)?;


    loop {

        let time = Local::now().time();
        let next_min = time + Duration::seconds(61 - time.second() as i64);
        let next_opt = NaiveTime::from_hms_opt(next_min.hour(), next_min.minute(), 0);
        
        if let Some(next) = next_opt {

            let one_minute_after = next + Duration::seconds(60);

            println!("Next: {}" , next.format("%H:%M:%S %f"));

            let ms = (next-time).num_milliseconds();
            if ms > 0 {
                println!("Sleep {}", ms);
                thread::sleep(std::time::Duration::from_millis(ms as u64));
            }
            println!("Time: {}" , Local::now().format("%d/%m/%Y %H:%M:%S %f"));

            let nm = next.minute();
            let opt_buf: Option<[u8;4]> = match nm {

                59 => Some([ 'Q' as u8, 0, one_minute_after.hour12().1 as u8, 0] ),
                14 => Some([ 'Q' as u8, 1, one_minute_after.hour12().1 as u8, 0] ),
                29 => Some([ 'Q' as u8, 2, one_minute_after.hour12().1 as u8, 0] ),
                44 => Some([ 'Q' as u8, 3, one_minute_after.hour12().1 as u8, 0] ),
                _ => None

            };
    
            if let Some(buf) = opt_buf {
                socket.send_to(&buf, "255.255.255.255:2412")?;
            }

            socket.send_to(&[ 'T' as u8, next.hour() as u8, next.minute() as u8, 0], "255.255.255.255:2412")?;
            thread::sleep(std::time::Duration::from_millis(1000));

        } else {
            println!("Oooops");
            thread::sleep(std::time::Duration::from_millis(1000));
        }



        


    }



        // // the message, it will be cut off.
        // let mut buf: [u8;10] = [0; 10];
        // socket.send_to(buf, "255.255.255.255:2412")?;

    Ok(())

}
