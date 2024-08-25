pub mod input {
    use crossbeam_queue::SegQueue;
    use std::io::{self, Write};

    pub fn read_queue_from_user(mode: &str, test_input: &str) -> SegQueue<Vec<u32>> {
        let q: SegQueue<Vec<u32>> = SegQueue::new();
        if mode != "TEST" {
            let mut input = String::new();
            print!("Enter the queue as a newline-separated list of integers: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let lines: Vec<&str> = input.lines().collect();
            for line in lines {
                let numbers: Vec<u32> =
                    line.split(',').map(|s| s.trim().parse().unwrap()).collect();
                q.push(numbers);
            }
        } else {
            // TEST mode for tests
            let test_input_lines: Vec<&str> = test_input.lines().collect();
            for line in test_input_lines {
                let numbers: Vec<u32> =
                    line.split(',').map(|s| s.trim().parse().unwrap()).collect();
                q.push(numbers);
            }
        }
        q
    }
}

#[cfg(test)]
mod tests {
    use super::input::read_queue_from_user;
    use crossbeam_queue::SegQueue;

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_empty() {
        // Test empty input
        let empty_input = "\n";
        let expected_empty: SegQueue<Vec<u32>> = SegQueue::new();
        let actual_queue = read_queue_from_user("TEST", empty_input);
        let expected_queue = expected_empty;
        let actual_vec: Vec<Vec<u32>> = actual_queue
            .into_iter()
            .map(|seg_queue| seg_queue.into_iter().collect())
            .collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_single_input() {
        // Test single line input
        let single_line_input = "1, 2, 3\n";
        let expected_single_line: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", single_line_input);
        let expected_queue = expected_single_line;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_multiple_lines() {
        // Test multiple line input
        let multiple_line_input = "4, 5, 6\n7, 8, 9\n";
        let expected_multiple_line: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_line_input);
        let expected_queue = expected_multiple_line;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }
    #[test]
    fn test_read_queue_from_user_multiple_inputs() {
        // Test multiple inputs
        let multiple_inputs = "1, 2, 3\n4, 5, 6\n7, 8, 9\n";
        let expected_multiple_inputs: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_inputs);
        let expected_queue = expected_multiple_inputs;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_mixed_inputs() {
        // Test mixed inputs (empty line, single line, multiple lines)
        let mixed_inputs = "\n1, 2, 3\n\n4, 5, 6\n\n7, 8, 9\n";
        let expected_mixed_inputs: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", mixed_inputs);
        let expected_queue = expected_mixed_inputs;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_invalid_input() {
        // Test invalid input (non-integer values)
        let invalid_input = "1, 2, 3\na, b, c\n4, 5, 6\n";
        let expected_invalid_input: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", invalid_input);
        let expected_queue = expected_invalid_input;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }
    #[test]
    fn test_read_queue_from_user_no_input() {
        // Test no input
        let no_input = "";
        let expected_no_input: SegQueue<Vec<u32>> = SegQueue::new();
        let actual_queue = read_queue_from_user("TEST", no_input);
        let expected_queue = expected_no_input;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_whitespace_input() {
        // Test whitespace input
        let whitespace_input = "   \n\n   ";
        let expected_whitespace: SegQueue<Vec<u32>> = SegQueue::new();
        let actual_queue = read_queue_from_user("TEST", whitespace_input);
        let expected_queue = expected_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_duplicate_inputs() {
        // Test duplicate inputs
        let duplicate_inputs = "1, 2, 3\n1, 2, 3\n";
        let expected_duplicate_inputs: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![1, 2, 3]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", duplicate_inputs);
        let expected_queue = expected_duplicate_inputs;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_large_input() {
        // Test large input
        let large_input = "1, 2, 3\n4, 5, 6\n7, 8, 9\n".repeat(1000);
        let expected_large_input: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            for _ in 0..1000 {
                q.push(vec![1, 2, 3]);
                q.push(vec![4, 5, 6]);
                q.push(vec![7, 8, 9]);
            }
            q
        };
        let actual_queue = read_queue_from_user("TEST", &large_input);
        let expected_queue = expected_large_input;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }
    #[test]
    #[should_panic]
    fn test_read_queue_from_user_negative_numbers() {
        // Test input with negative numbers
        let negative_input = "-1, -2, -3\n4, 5, 6\n";
        let expected_negative: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![4294967295, 4294967294, 4294967293]);
            q.push(vec![4, 5, 6]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", negative_input);
        let expected_queue = expected_negative;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_large_numbers() {
        // Test input with large numbers
        let large_numbers_input = "4294967295, 4294967294, 4294967293\n4, 5, 6\n";
        let expected_large_numbers: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![4294967295, 4294967294, 4294967293]);
            q.push(vec![4, 5, 6]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", large_numbers_input);
        let expected_queue = expected_large_numbers;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_whitespace_between_numbers() {
        // Test input with whitespace between numbers
        let whitespace_between_numbers_input = "1,   2,   3\n4,   5,   6\n";
        let expected_whitespace_between_numbers: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", whitespace_between_numbers_input);
        let expected_queue = expected_whitespace_between_numbers;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_trailing_comma() {
        // Test input with trailing comma
        let trailing_comma_input = "1, 2, 3,\n4, 5, 6,\n";
        let expected_trailing_comma: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", trailing_comma_input);
        let expected_queue = expected_trailing_comma;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_mixed_whitespace() {
        // Test input with mixed whitespace
        let mixed_whitespace_input = "1, 2, 3\n4, 5, 6\n\n\n7, 8, 9\n";
        let expected_mixed_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", mixed_whitespace_input);
        let expected_queue = expected_mixed_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }
    #[test]
    fn test_read_queue_from_user_custom_input() {
        // Test custom input
        let custom_input = "10, 20, 30\n40, 50, 60\n70, 80, 90\n";
        let expected_custom: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![10, 20, 30]);
            q.push(vec![40, 50, 60]);
            q.push(vec![70, 80, 90]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", custom_input);
        let expected_queue = expected_custom;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_large_custom_input() {
        // Test large custom input
        let large_custom_input = "1, 2, 3\n4, 5, 6\n7, 8, 9\n".repeat(10000);
        let expected_large_custom: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            for _ in 0..10000 {
                q.push(vec![1, 2, 3]);
                q.push(vec![4, 5, 6]);
                q.push(vec![7, 8, 9]);
            }
            q
        };
        let actual_queue = read_queue_from_user("TEST", &large_custom_input);
        let expected_queue = expected_large_custom;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_empty_input() {
        // Test empty input
        let empty_input = "";
        let expected_empty: SegQueue<Vec<u32>> = SegQueue::new();
        let actual_queue = read_queue_from_user("TEST", empty_input);
        let expected_queue = expected_empty;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_single_line_input() {
        // Test single line input
        let single_line_input = "1, 2, 3\n";
        let expected_single_line: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", single_line_input);
        let expected_queue = expected_single_line;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_multiple_inputs_with_empty_lines() {
        // Test multiple inputs with empty lines
        let multiple_inputs_with_empty_lines = "1, 2, 3\n\n4, 5, 6\n\n\n7, 8, 9\n";
        let expected_multiple_inputs_with_empty_lines: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_inputs_with_empty_lines);
        let expected_queue = expected_multiple_inputs_with_empty_lines;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_duplicate_inputs_with_whitespace() {
        // Test duplicate inputs with whitespace
        let duplicate_inputs_with_whitespace = "1, 2, 3\n\n1, 2, 3\n";
        let expected_duplicate_inputs_with_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![1, 2, 3]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", duplicate_inputs_with_whitespace);
        let expected_queue = expected_duplicate_inputs_with_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_large_custom_input_with_whitespace() {
        // Test large custom input with whitespace
        let large_custom_input_with_whitespace = "1, 2, 3\n\n4, 5, 6\n\n\n7, 8, 9\n".repeat(10000);
        let expected_large_custom_with_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            for _ in 0..10000 {
                q.push(vec![1, 2, 3]);
                q.push(vec![4, 5, 6]);
                q.push(vec![7, 8, 9]);
            }
            q
        };
        let actual_queue = read_queue_from_user("TEST", &large_custom_input_with_whitespace);
        let expected_queue = expected_large_custom_with_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]

    fn test_read_queue_from_user_single_input_with_trailing_whitespace() {
        // Test single input with trailing whitespace
        let single_input_with_trailing_whitespace = "1, 2, 3   \n";
        let expected_single_input_with_trailing_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", single_input_with_trailing_whitespace);
        let expected_queue = expected_single_input_with_trailing_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_multiple_inputs_with_trailing_whitespace() {
        // Test multiple inputs with trailing whitespace
        let multiple_inputs_with_trailing_whitespace = "1, 2, 3   \n4, 5, 6   \n7, 8, 9   \n";
        let expected_multiple_inputs_with_trailing_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_inputs_with_trailing_whitespace);
        let expected_queue = expected_multiple_inputs_with_trailing_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_read_queue_from_user_multiple_inputs_with_leading_whitespace() {
        // Test multiple inputs with leading whitespace
        let multiple_inputs_with_leading_whitespace = "   1, 2, 3\n   4, 5, 6\n   7, 8, 9\n";
        let expected_multiple_inputs_with_leading_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_inputs_with_leading_whitespace);
        let expected_queue = expected_multiple_inputs_with_leading_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_multiple_inputs_with_whitespace_between_inputs() {
        // Test multiple inputs with whitespace between inputs
        let multiple_inputs_with_whitespace_between_inputs = "1, 2, 3\n\n\n4, 5, 6\n\n\n7, 8, 9\n";
        let expected_multiple_inputs_with_whitespace_between_inputs: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue =
            read_queue_from_user("TEST", multiple_inputs_with_whitespace_between_inputs);
        let expected_queue = expected_multiple_inputs_with_whitespace_between_inputs;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    #[should_panic]
    fn test_read_queue_from_user_multiple_inputs_with_mixed_whitespace() {
        // Test multiple inputs with mixed whitespace
        let multiple_inputs_with_mixed_whitespace = "1, 2, 3\n4, 5, 6\n\n\n7, 8, 9\n";
        let expected_multiple_inputs_with_mixed_whitespace: SegQueue<Vec<u32>> = {
            let q = SegQueue::new();
            q.push(vec![1, 2, 3]);
            q.push(vec![4, 5, 6]);
            q.push(vec![7, 8, 9]);
            q
        };
        let actual_queue = read_queue_from_user("TEST", multiple_inputs_with_mixed_whitespace);
        let expected_queue = expected_multiple_inputs_with_mixed_whitespace;
        let actual_vec: Vec<Vec<u32>> = actual_queue.into_iter().collect();
        let expected_vec: Vec<Vec<u32>> = expected_queue.into_iter().collect();
        assert_eq!(actual_vec, expected_vec);
    }
}
