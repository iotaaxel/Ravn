pub mod simulation {

    use crossbeam_channel::unbounded;
    use std::thread;

    use crate::conversions::conversion::{
        fixed_points_triplet_from_bits_euler_angles, reconstructed_euler_angles_from_fixed_points,
    };

    pub fn run_simulation(queue: crossbeam_queue::SegQueue<Vec<u32>>) {
        println!("Running simulation...");

        // Create senders and receivers for necessary channels
        let (s1, r1) = unbounded();
        let (s2, r2) = unbounded();

        // Spawn a thread that sends sensor data to the first receiver
        let thread1 = thread::spawn(move || {
            // Get data from the sensor continuously
            while let Some(sensor_data) = queue.pop() {
                s1.send(Some(sensor_data))
                    .expect("Unable to send sensor data!");
            }
            // Signal the end of the queue
            drop(s1);
        });

        // Spawn a thread that receives a message, processes it, and then sends to the next receiver
        let thread2 = thread::spawn(move || {
            while let Ok(sensor_data) = r1.recv() {
                if let Some(euler_angles) = sensor_data {
                    // Access the Euler angles (convert bits to three floating points stored as u32)
                    let fixed_representation: Vec<u32> =
                        fixed_points_triplet_from_bits_euler_angles(euler_angles);
                    s2.send(Some(fixed_representation))
                        .expect("Unable to get correct conversion from Euler angles.");
                } else {
                    // Forward the end signal to thread3
                    s2.send(None).expect("Unable to send final message!");
                    break;
                }
            }
        });

        // Spawn a thread that displays the received message
        let thread3 = thread::spawn(move || {
            while let Ok(sensor_data) = r2.recv() {
                if let Some(euler_angles_fixed_representation) = sensor_data {
                    // Access the Euler angles (three u32 values used to represent the floating point values)
                    let (roll, pitch, yaw): (f32, f32, f32) =
                        reconstructed_euler_angles_from_fixed_points(
                            euler_angles_fixed_representation,
                        );
                    println!(
                        "Observed Euler angles of {:#?} (roll), {:#?} (pitch), and {:#?} (yaw).",
                        roll, pitch, yaw
                    );
                } else {
                    // End signal received
                    break;
                }
            }
        });

        // Join threads to ensure they complete
        thread1.join().expect("Unable to join thread1!");
        thread2.join().expect("Unable to join thread2!");
        thread3.join().expect("Unable to join thread3!");

        println!("Simulation concluded.");
    }
}
#[cfg(test)]
mod tests {
    use super::simulation::*;
    use crossbeam_queue::SegQueue;

    #[test]
    fn test_run_simulation_empty_queue() {
        // Create an empty queue
        let queue = SegQueue::new();

        // Run the simulation
        run_simulation(queue);
        // No assertions needed, just checking if the simulation completes without errors
    }

    #[test]
    fn test_run_simulation_large_queue() {
        // Create a large sample queue with sensor data
        let queue = SegQueue::new();
        for _ in 0..1000 {
            let bits = vec![
                1, 0, 1, 1, 0, 0, 1, 0, // x: 178
                0, 0, 0, 0, 1, 0, 1, 1, // y: 11
                1, 1, 1, 1, 0, 0, 0, 1, // z: 241
                1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
            ];
            queue.push(bits);
        }

        // Run the simulation
        run_simulation(queue);
        // No assertions needed, just checking if the simulation completes without errors
    }

    #[test]
    fn test_run_simulation_multiple_threads() {
        // Sample vectors of bits representing sensor data
        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2 = vec![
            0, 1, 0, 1, 0, 1, 0, 1, // x: 85
            1, 0, 1, 0, 1, 0, 1, 0, // y: 170
            0, 0, 1, 1, 1, 1, 0, 0, // z: 60
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits3 = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits1_clone = bits1.clone();
        let bits2_clone = bits2.clone();
        let bits3_clone = bits3.clone();

        // Create sample queues with sensor data
        let queue = SegQueue::new();
        queue.push(bits1);
        queue.push(bits2);
        queue.push(bits3);

        let queue2 = SegQueue::new();
        queue2.push(bits1_clone.clone());
        queue2.push(bits2_clone.clone());
        queue2.push(bits3_clone.clone());

        // Run the simulation with multiple threads
        let thread1 = std::thread::spawn(move || {
            run_simulation(queue);
        });
        let thread2 = std::thread::spawn(move || {
            run_simulation(queue2);
        });

        // Wait for the threads to complete
        thread1.join().expect("Unable to join thread1!");
        thread2.join().expect("Unable to join thread2!");
        // No assertions needed, just checking if the simulation completes without errors
    }
}
