pub mod thread_test1 {
    use std::{thread, time::Duration};

    pub fn test_panic() -> thread::JoinHandle<()> {

        // let handle2 = thread::spawn(|| {
        //     for i in 0..10 {
        //         println!("Count in a MAIN: {}", i);
        //         thread::sleep(Duration::from_millis(200));
        //     }
        // });

        // panic_a_thread().join().expect("panicked");
        // handle2.join();

        return panic_a_thread();
    }

    fn panic_a_thread() -> thread::JoinHandle<()> {
        // println!("INTO PANIC THREAD!!");
        let handler = thread::spawn(|| {
            for i in 0..10 {
                println!("Count in a Thread: {}", i);
                thread::sleep(Duration::from_millis(200));
                if i == 5 {
                    panic!();
                }
            }
        });

        return handler;

        // return handler;
    }
}