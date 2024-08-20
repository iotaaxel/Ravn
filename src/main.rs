extern crate crossbeam_channel;
use crossbeam_channel::{bounded, unbounded};

fn main() {
    // Create senders and receivers for necessary channels
    let (s1, r1) = unbounded();
    let (s2, r2) = (s1.clone(), r1.clone());
    let (s3, r3) = (s2.clone(), r2.clone());

    // Get data from the sensor continuously
    let mut data: Vec <u32> = Vec::new();

    loop { //Note: don't do this. 
        s1.send(&data).expect("Unable to get process data!");
    }

    // Spawn a thread that receives a message and then sends one.
    thread::spawn(move || {
        let euler_angles = r2.recv().expect("Unable to receive data!");
        // Access the Euler angles
        let x = &euler_angles[..8];
        let y = &euler_angles[..16];
        let z = euler_angles[..24];

        // TODO: Convert from floating point stored as u32
        // TODO: Look into crate https://docs.rs/fast-float/latest/fast_float/
        let res: u32 = x + y + z; //TODO: Use correct calculation based on use case

        let val = s2.send(res).expect("Unable to get correct conversion from Euler angles.");

        println!("Simulation concluded with value {}!", val);
    });

}
