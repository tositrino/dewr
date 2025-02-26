//! This module contains functions for the data race lab
//! # Examples:
//! ````
//! use module_02::datarace::*;
//! datarace01();
//! datarace02();
//! ````

use std::sync::Mutex;
//use std::thread;

/// datarace test function 01
/// # Examples:
/// ````
/// use module_02::datarace::datarace01;
/// datarace();
/// ````

// Mutex that protects the data vector, and then we spawn three threads 
//that each acquire a lock on the mutex and modify an element of the vector.

pub fn datarace01()->i32 {
    let result:i32=0;
    println!("datarace01() - setup multhreaded data:");
    let data = Mutex::new(vec![1, 2, 3]);
    /*
    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.unwrap().clone(); //data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    */
    println!("datarace01(): data={:?}", data);
    result
}

/// datarace test function 01
/// # Examples:
/// ````
/// use module_02::datarace::datarace02;
/// datarace02();
/// ````

pub fn datarace02()->i32 {
    let result:i32=0;
    println!("datarace02() - setup multhreaded data:");
    let mut data = vec![1, 2, 3];
    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        //thread::spawn(move || {
        //    data[i] += 1;
        //});
        data[i] += 1;
    }
    // No data race can occur, this will not compile.
    println!("datarace02(): data={:?}", data);
    result
}
