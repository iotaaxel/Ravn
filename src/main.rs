use crate::inputs::input::read_queue_from_user;
use crate::simulations::simulation::run_simulation;
mod conversions;
mod inputs;
mod simulations;

use crossbeam_queue::SegQueue;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter 'y' to use the default queue or 'n' to enter a queue: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let use_default_queue = input.trim().eq_ignore_ascii_case("y");

    if use_default_queue {
        let q: SegQueue<Vec<u32>> = SegQueue::new();
        run_simulation(q);
    } else {
        let q: SegQueue<Vec<u32>> = read_queue_from_user("", ""); // Replace actual_arg1 and actual_arg2 with the actual arguments
        run_simulation(q);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simulation() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2 = bits.clone();

        q.push(bits);
        q.push(bits2);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    fn test_run_simulation_empty_queue() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    fn test_run_simulation_single_item_queue() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        q.push(bits);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    fn test_run_simulation_multiple_items_queue() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(bits2);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    fn test_run_simulation_large_queue() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits3: Vec<u32> = vec![
            1, 1, 0, 0, 1, 0, 1, 1, // x: 203
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits4: Vec<u32> = vec![
            1, 0, 1, 0, 1, 1, 0, 1, // x: 173
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(bits2);
        q.push(bits3);
        q.push(bits4);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_invalid_queue() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        // Push an invalid bit sequence to the queue
        q.push(bits);
        q.push(vec![0, 1, 2, 3]); // Invalid bits

        run_simulation(q);

        // Add your assertions here
    }
    #[test]
    #[should_panic]
    fn test_run_simulation_large_queue_with_empty_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits3: Vec<u32> = vec![]; // Empty item

        let bits4: Vec<u32> = vec![
            1, 0, 1, 0, 1, 1, 0, 1, // x: 173
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(bits2);
        q.push(bits3);
        q.push(bits4);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_invalid_queue_with_empty_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![]; // Empty item

        // Push an invalid bit sequence to the queue
        q.push(bits1);
        q.push(bits2);
        q.push(vec![0, 1, 2, 3]); // Invalid bits

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_multiple_items_queue_with_empty_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits3: Vec<u32> = vec![]; // Empty item

        let bits4: Vec<u32> = vec![
            1, 1, 0, 0, 1, 0, 1, 1, // x: 203
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits5: Vec<u32> = vec![]; // Empty item

        q.push(bits1);
        q.push(bits2);
        q.push(bits3);
        q.push(bits4);
        q.push(bits5);

        run_simulation(q);

        // Add your assertions here
    }
    #[test]
    fn test_run_simulation_large_queue_with_multiple_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits3: Vec<u32> = vec![
            1, 1, 0, 0, 1, 0, 1, 1, // x: 203
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits4: Vec<u32> = vec![
            1, 0, 1, 0, 1, 1, 0, 1, // x: 173
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits5: Vec<u32> = vec![
            0, 0, 1, 1, 0, 1, 0, 1, // x: 45
            1, 0, 1, 0, 1, 1, 0, 1, // y: 173
            0, 1, 1, 0, 1, 0, 1, 0, // z: 106
            1, 1, 0, 0, 0, 1, 0, 1, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(bits2);
        q.push(bits3);
        q.push(bits4);
        q.push(bits5);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_invalid_queue_with_multiple_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits3: Vec<u32> = vec![
            1, 1, 0, 0, 1, 0, 1, 1, // x: 203
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits4: Vec<u32> = vec![
            1, 0, 1, 0, 1, 1, 0, 1, // x: 173
            0, 1, 1, 0, 0, 1, 0, 1, // y: 177
            1, 0, 0, 1, 1, 0, 1, 0, // z: 218
            0, 1, 0, 1, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let bits5: Vec<u32> = vec![
            0, 0, 1, 1, 0, 1, 0, 1, // x: 45
            1, 0, 1, 0, 1, 1, 0, 1, // y: 173
            0, 1, 1, 0, 1, 0, 1, 0, // z: 106
            1, 1, 0, 0, 0, 1, 0, 1, // Last 8 bits unused
        ];

        // Push an invalid bit sequence to the queue
        q.push(bits1);
        q.push(bits2);
        q.push(bits3);
        q.push(bits4);
        q.push(bits5);
        q.push(vec![0, 1, 2, 3]); // Invalid bits

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    fn test_run_simulation_empty_queue_with_multiple_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(bits2);
        q.pop();
        q.pop();
        run_simulation(q);
    }
    #[test]
    #[should_panic]
    fn test_run_simulation_single_item_queue_with_empty_items() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let empty_bits: Vec<u32> = vec![]; // Empty item

        q.push(bits);
        q.push(empty_bits);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_multiple_items_queue_with_empty_items_at_end() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        let empty_bits: Vec<u32> = vec![]; // Empty item

        q.push(bits1);
        q.push(bits2);
        q.push(empty_bits);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_multiple_items_queue_with_empty_items_in_between() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let empty_bits: Vec<u32> = vec![]; // Empty item

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        q.push(bits1);
        q.push(empty_bits);
        q.push(bits2);

        run_simulation(q);

        // Add your assertions here
    }

    #[test]
    #[should_panic]
    fn test_run_simulation_invalid_queue_with_empty_items_in_between() {
        let q: SegQueue<Vec<u32>> = SegQueue::new();

        let bits1: Vec<u32> = vec![
            1, 0, 1, 1, 0, 0, 1, 0, // x: 178
            0, 0, 0, 0, 1, 0, 1, 1, // y: 11
            1, 1, 1, 1, 0, 0, 0, 1, // z: 241
            1, 1, 1, 0, 1, 0, 0, 0, // Last 8 bits unused
        ];

        let empty_bits: Vec<u32> = vec![]; // Empty item

        let bits2: Vec<u32> = vec![
            0, 1, 0, 1, 1, 0, 1, 0, // x: 90
            1, 1, 0, 1, 0, 1, 0, 0, // y: 212
            0, 0, 1, 0, 1, 1, 1, 1, // z: 47
            1, 0, 1, 0, 0, 1, 1, 0, // Last 8 bits unused
        ];

        // Push an invalid bit sequence to the queue
        q.push(bits1);
        q.push(empty_bits);
        q.push(vec![0, 1, 2, 3]); // Invalid bits
        q.push(bits2);

        run_simulation(q);

        // Add your assertions here
    }
}
