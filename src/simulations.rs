pub mod simulation {

    use crossbeam_channel::{unbounded, Receiver, Sender};
    use nalgebra::Rotation3;
    use std::thread;

    use crate::conversions::conversion::{
        bits_to_u32_triplet, convert_fixed32_to_float, convert_float_to_fixed32,
    };

    pub fn run_simulation() {
        // Create senders and receivers for necessary channels
        let (s1, r1): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = unbounded::<Vec<u32>>();
        let (s2, r2): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = (s1.clone(), r1.clone());
        let (_s3, r3): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = (s2.clone(), r2.clone());

        // Get data from the sensor continuously
        // Note: This is a vector of bits that represent the Euler angles
        let data: Vec<u32> = Vec::new();

        // Commented out since it will run indefinitely
        // and will make code below unreachable
        // Send data to the second receiver
        // loop { //Note: don't do this.
        s1.send(data).expect("Unable to get process data!");
        // }

        // Spawn a thread that receives a message and then sends one.
        thread::spawn(move || {
            let euler_angles: Vec<u32> = r2.recv().expect("Unable to receive data!");

            // Access the Euler angles (convert bits to three floating points stored as u32)
            let (x, y, z): (u32, u32, u32) = bits_to_u32_triplet(&euler_angles)
                .expect("Unable to get correct conversion from Euler angles.");

            // Convert the Euler angles to floating point values
            let fractional_bits = 16;
            let fixed_values_triplet: (u32, u32, u32) = (x, y, z);
            let result_x: f32 = convert_fixed32_to_float(fixed_values_triplet.0, fractional_bits);
            let result_y: f32 = convert_fixed32_to_float(fixed_values_triplet.1, fractional_bits);
            let result_z: f32 = convert_fixed32_to_float(fixed_values_triplet.2, fractional_bits);

            // Convert the floating point values to fixed-point representation
            let fixed_x: u32 = convert_float_to_fixed32(result_x, fractional_bits)
                .expect("Unable to get correct conversion from Euler angles.");
            let fixed_y: u32 = convert_float_to_fixed32(result_y, fractional_bits)
                .expect("Unable to get correct conversion from Euler angles.");
            let fixed_z: u32 = convert_float_to_fixed32(result_z, fractional_bits)
                .expect("Unable to get correct conversion from Euler angles.");

            // Send the result to the third receiver
            s2.send(vec![fixed_x, fixed_y, fixed_z])
                .expect("Unable to get correct conversion from Euler angles.");
        });

        // Spawn a thread that receives and displays a message.
        thread::spawn(move || {
            let euler_angles: Vec<u32> = r3.recv().expect("Unable to receive data!");

            // Access the Euler angles (three u32 values used to represent the floating point values)
            let x: u32 = euler_angles[0]; // Assuming this represents the roll
            let y: u32 = euler_angles[1]; // Assuming this represents the pitch
            let z: u32 = euler_angles[2]; // Assuming this represents the yaw

            // Convert the Euler angles to floating point values
            let fractional_bits = 16;
            let fixed_values_triplet: (u32, u32, u32) = (x, y, z);
            let result_x: f32 = convert_fixed32_to_float(fixed_values_triplet.0, fractional_bits);
            let result_y: f32 = convert_fixed32_to_float(fixed_values_triplet.1, fractional_bits);
            let result_z: f32 = convert_fixed32_to_float(fixed_values_triplet.2, fractional_bits);

            // Creates a new rotation from the given Euler angles (in order roll, pitch, yaw)
            let rotation = Rotation3::from_euler_angles(result_x, result_y, result_z);
            let (roll, pitch, yaw) = rotation.euler_angles();
            println!("Simulation concluded with the observed Euler angles of {:#?} (roll), {:#?} (pitch), and {:#?} (yaw).", roll, pitch, yaw);
        });
    }
}
