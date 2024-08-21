use std::thread;
use crossbeam_channel::unbounded;

fn main() {
    // Create senders and receivers for necessary channels
    let (s1, r1) = unbounded::<Vec<u32>>();
    let (s2, r2) = (s1.clone(), r1.clone());
    let (_s3, r3) = (s2.clone(), r2.clone());

    // Get data from the sensor continuously
    let data: Vec <u32> = Vec::new();

    // Commented out since it will run indefinitely
    // and will make code below unreachable
    // Send data to the second receiver
    // loop { //Note: don't do this. 
        s1.send(data).expect("Unable to get process data!");
    // }

    // Spawn a thread that receives a message and then sends one.
    thread::spawn(move || {
        let euler_angles: Vec<u32> = r2.recv().expect("Unable to receive data!");
        // Access the Euler angles
        let _x = &euler_angles[..8];
        let _y = &euler_angles[..16];
        let _z = &euler_angles[..24];

        let res: u32 = 0; // TODO: Use x, y, z to get the result of the conversion

        s2.send(vec![res, res, res]).expect("Unable to get correct conversion from Euler angles.");

        println!("Simulation concluded with value {:#?}!", ());
    });

    // Spawn a thread that receives and displays a message.
    thread::spawn(move || {
        let euler_angles: Vec<u32> = r3.recv().expect("Unable to receive data!");
        // Access the Euler angles
        // TODO: Use the Euler angles to get the result (floating point values)
        let x = &euler_angles[..8];
        let y = &euler_angles[..16];
        let z = &euler_angles[..24];

        // let res: u32 = 0; // TODO: Use x, y, z to get the result

        println!("Simulation concluded with the observed Euler angles of x = {:#?}, y = {:#?}, and z = {:#?}.!", x, y, z);
    });

}
