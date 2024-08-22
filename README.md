## Overview
We are simulating using multiple threads to process data from a sensor, perform basic sensor fusion, and display the information. 

The three main threads have the following roles:
* `Thread 1`: Get data from the sensor
  * Sensor input is pooled continuously as Euler angles (use `Vec<u32>`)
    * Angles `x`, `y`, `z`
    * Each vector is divided into `| x (8 bits) | y (8 bits) | z (8 bits) | (unused 8 bits) |`
* `Thread 2`: Perform basic sensor fusion
  * Floating point data is stored as `u32`
  * Remember to convert data back to a floating point (without truncation of significant figures)  
* `Thread 3`: Display the resulting data
  * We can start with simple output.  

## Data Sharing between threads
* Flow of information:
  * Thread 1 (sender only)
  * Thread 2 (sender + receiver)
  * Thread 3 (receiver only)

## Priorities and Optimizations
* We prioritize ensuring proper threading (syncing and data passing between threads).
* Optimizations would be great considering potentially large amounts of data.

## Future Work
* Implement queueing
* Use Command-line arguments to take in user input (will probably be converted to `Vec<Vec<u32>>`)
* Support alternative input formats (JSON, comma-separated, etc.)
* Support alternative output formats (JSON, CSV, etc.)
* Support input and output compression types (Snappy, etc.)
