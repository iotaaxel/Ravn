use crossbeam_channel::{unbounded, Receiver, Sender};
use nalgebra::Rotation3;
use std::thread;

fn main() {
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

fn convert_float_to_fixed32(value: f32, fractional_bits: u32) -> Result<u32, &'static str> {
    // Calculate the fixed-point representation
    let scaled_value = value * (1u32 << fractional_bits) as f32;

    // Check if the scaled value fits within the u32 range
    // Note: Depending on the application, you might want to set to u32::MIN or u32::MAX instead of returning an error
    if scaled_value < 0.0 {
        return Err("Underflow: value is too small for u32");
    } else if scaled_value > u32::MAX as f32 {
        return Err("Overflow: value is too large for u32");
    }

    // Convert to u32 and return
    Ok(scaled_value.round() as u32)
}

fn convert_fixed32_to_float(fixed_value: u32, fractional_bits: u32) -> f32 {
    // Convert the unsigned integer (fixed-point) back to a floating point value
    fixed_value as f32 / (1u32 << fractional_bits) as f32
}

fn bits_to_u32_triplet(bits: &[u32]) -> Result<(u32, u32, u32), &'static str> {
    if bits.len() < 24 {
        return Err("Not enough bits to create u32 values for x, y, and z.");
    }

    let x: u32 = bits_to_u32(&bits[0..8])?;
    let y: u32 = bits_to_u32(&bits[8..16])?;
    let z: u32 = bits_to_u32(&bits[16..24])?;

    Ok((x, y, z))
}

fn bits_to_u32(bits: &[u32]) -> Result<u32, &'static str> {
    if bits.len() < 8 {
        return Err("Not enough bits to create a u32 from the first 8 bits.");
    }

    // Initialize x as u32 from the first 8 bits
    let mut x: u32 = 0;

    for (i, item) in bits.iter().enumerate().take(8) {
        if *item != 0 && *item != 1 {
            return Err("Invalid bit value; bits must be 0 or 1.");
        }
        x |= *item << (7 - i);
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_bits_to_u32_triplet_valid() {
        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let result = bits_to_u32_triplet(&bits);
        assert_eq!(result.unwrap(), (178, 11, 241));
    }

    #[test]
    fn test_bits_to_u32_triplet_insufficient_bits() {
        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11, z is missing
        ];

        let result = bits_to_u32_triplet(&bits);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Not enough bits to create u32 values for x, y, and z."
        );
    }

    #[test]
    fn test_bits_to_u32_triplet_invalid_bits() {
        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 2, // x: Invalid bit value (2)
            0, 0, 0, 0, 1, 0, 1, 1, // y
            1, 1, 1, 1, 0, 0, 0, 1, // z
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let result = bits_to_u32_triplet(&bits);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Invalid bit value; bits must be 0 or 1."
        );
    }

    #[test]
    fn test_bits_to_u32_valid() {
        // Test with a valid 8-bit input
        let bits: Vec<u32> = vec![1, 0, 1, 1, 0, 0, 1, 0];
        let result: Result<u32, &str> = bits_to_u32(&bits);
        assert_eq!(result.unwrap(), 178); // Expected output: 178 (0b10110010)

        // Test with another valid 8-bit input
        let bits: Vec<u32> = vec![0, 0, 0, 0, 1, 0, 1, 1];
        let result: Result<u32, &str> = bits_to_u32(&bits);
        assert_eq!(result.unwrap(), 11); // Expected output: 11 (0b00001011)

        // Test with a vector longer than 8 bits (only first 8 should be used)
        let bits: Vec<u32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0];
        let result: Result<u32, &str> = bits_to_u32(&bits);
        assert_eq!(result.unwrap(), 255); // Expected output: 255 (0b11111111)
    }

    #[test]
    fn test_bits_to_u32_insufficient_bits() {
        // Test with fewer than 8 bits
        let bits: Vec<u32> = vec![1, 0, 1, 0, 1];
        let result: Result<u32, &str> = bits_to_u32(&bits);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Not enough bits to create a u32 from the first 8 bits."
        );
    }

    #[test]
    fn test_bits_to_u32_invalid_bits() {
        // Test with invalid bit values (not 0 or 1)
        let bits: Vec<u32> = vec![1, 0, 2, 1, 0, 0, 1, 0];
        let result: Result<u32, &str> = bits_to_u32(&bits);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Invalid bit value; bits must be 0 or 1."
        );
    }

    #[test]
    fn test_convert_fixed32_to_float() {
        let fractional_bits = 16;

        // Test conversion of a fixed-point value back to float
        let fixed_value = 205887;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        println!("res 1: {:?}", result);
        assert!((result - PI).abs() < 1e-5); // Expected float value (within tolerance)

        // Test conversion of zero
        let fixed_value = 0;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        println!("res 2: {:?}", result);
        assert_eq!(result, 0.0); // Expected float value

        // Test conversion of a small fixed-point value
        let fixed_value = 7;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        println!("res 3: {:?}", result);
        assert!((result - 0.0001).abs() < 1e-5); // Expected float value (within tolerance)
    }

    #[test]
    fn test_convert_fixed32_to_float_large_value() {
        let fractional_bits = 16;
        let fixed_value = u32::MAX; // Maximum value for u32
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        let expected = (u32::MAX as f32) / ((1u32 << fractional_bits) as f32);
        assert!((result - expected).abs() < 1e-5);
    }

    #[test]
    fn test_convert_fixed32_to_float_negative() {
        // Since u32 cannot represent negative numbers, this test is not applicable.
        // We skip the negative case because u32 does not support negative values.
        let value = -PI;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_fixed32_to_float_small_value() {
        let fractional_bits = 16;
        let fixed_value = 1; // Smallest possible value for a non-zero fixed-point number
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        let expected = 1.0 / (1u32 << fractional_bits) as f32;
        assert!((result - expected).abs() < 1e-7);
    }

    #[test]
    fn test_convert_float_to_fixed32_zero() {
        let value = 0.0;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_convert_float_to_fixed32_positive() {
        let value = PI;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        assert_eq!(result.unwrap(), 205887);
    }

    #[test]
    fn test_convert_float_to_fixed32_negative() {
        let value = -PI;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        // The behavior is not defined here since we don't handle negative values in this example.
        // Skipping this test.
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_float_to_fixed32_small_value() {
        let value = 0.0001;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn test_convert_float_to_fixed32_large_value() {
        let value = 1e10;
        let fractional_bits = 16;
        let result = convert_float_to_fixed32(value, fractional_bits);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Overflow: value is too large for u32"
        );
    }
}
