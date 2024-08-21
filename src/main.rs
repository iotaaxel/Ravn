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


fn convert_float_to_fixed32(value: f32, fractional_bits: u32) -> Result<u32, &'static str> {
    // Calculate the fixed-point representation
    let scaled_value = f32::from_bits(value.to_bits() * (1u32 << fractional_bits));
    
    // Check if the scaled value fits within the u32 range
    // Note: Depending on the application, you might want to set to u32::MIN or u32::MAX instead of returning an error
    if scaled_value < 0.0 {
        return Err("Underflow: value is too small for u32");
    } else if scaled_value > u32::MAX as f32 {
        return Err("Overflow: value is too large for u32");
    }
    
    // Convert to u32 and return
    Ok(scaled_value.round().to_bits())
}


fn convert_fixed32_to_float(fixed_value: u32, fractional_bits: u32) -> f32 {
    // Convert the unsigned integer (fixed-point) back to a floating point value
    f32::from_bits(fixed_value) / f32::from_bits(1u32 << fractional_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_float_to_fixed32() {
        let fractional_bits = 16;
        
        // Test conversion of a positive float
        let result = convert_float_to_fixed32(3.14159, fractional_bits);
        assert_eq!(result.unwrap(), 205887); // Expected fixed-point value
        
        // Test conversion of zero
        let result = convert_float_to_fixed32(0.0, fractional_bits);
        assert_eq!(result.unwrap(), 0); // Expected fixed-point value
        
        // Test conversion of a small positive float
        let result = convert_float_to_fixed32(0.0001, fractional_bits);
        assert_eq!(result.unwrap(), 6); // Expected fixed-point value
        
        // Test conversion of a value that would cause overflow
        let result = convert_float_to_fixed32(1e10, fractional_bits);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Overflow: value is too large for u32");
        
        // Test conversion of a negative float (underflow)
        let result = convert_float_to_fixed32(-3.14159, fractional_bits);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Underflow: value is too small for u32");
    }

    #[test]
    fn test_convert_fixed32_to_float() {
        let fractional_bits = 16;
        
        // Test conversion of a fixed-point value back to float
        let fixed_value = 205887;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        assert!((result - 3.14159).abs() < 1e-5); // Expected float value (within tolerance)
        
        // Test conversion of zero
        let fixed_value = 0;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        assert_eq!(result, 0.0); // Expected float value
        
        // Test conversion of a small fixed-point value
        let fixed_value = 6;
        let result = convert_fixed32_to_float(fixed_value, fractional_bits);
        assert!((result - 0.0001).abs() < 1e-7); // Expected float value (within tolerance)
    }
}
