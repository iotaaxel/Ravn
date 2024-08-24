use crate::simulations::simulation::run_simulation;
mod conversions;
mod simulations;

use crossbeam_queue::SegQueue;

fn main() {
    let q: SegQueue<Vec<u32>> = SegQueue::new();
    run_simulation(q);
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
}
