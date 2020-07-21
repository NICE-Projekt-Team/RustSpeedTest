// imports
use std::time::{Duration, Instant};
use std::fmt;
use std::fmt::Debug;
use byte_array::ByteArray;
use std::thread;
use tokio::runtime;
//use tokio::timer::Delay;

fn main() {
    // ---------- INITIALISING
    // the size of the array - 1024
    let mut size = 5;
    // the duration in minutes
    let time = 1;
    // the counter
    let count = 0;
    // remembering the start time
    let start = Instant::now();

    //creating the 3 arrays
    let mut ba1 = ByteArray::new();
    let mut ba2 = ByteArray::new();
    let mut ba3 = ByteArray::new();

    // ---------- WRITING & TRANSFERING
    // writing the array
    // ! on diverent threads
    ba1 = write_array(size);
    ba2 = write_array(size);
    ba3 = write_array(size);
    // ???? println!("{:?}", ba1);


    let mut rt = runtime::Builder::new()
        .core_threads(3).build().unwrap();

    //let task =

    //rt.spawn(task);
    //rt.shutdown_on_idle().wait.unwrap();

    // ---------- TIMING
    // duration
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    //divide by the duration and not 1
    let speed = count / 1;
    println!("Counter: {}, Speed: {}", count, speed);
}

fn write_array(size: u32) -> ByteArray{
    println!("Starting to write Array");

    let mut byteArray = ByteArray::new();
    let a: u32 = 1;
    byteArray.write(&a);

    for i in 0..size {
        byteArray <<= &a;
        //println!("{:?}", byteArray[i]);
    }


    println!("{:?}", byteArray.len());
    println!("Done writing the array");
    byteArray
}

fn transfer() {

}