use std::cmp::Ordering;
use std::time::SystemTime;

mod elevator;
use elevator::*;

mod experession_evaluation;
use experession_evaluation::*;

mod standard_library;
use standard_library::*;

mod read_write;
use read_write::*;

mod rot13;
use rot13::*;

mod exercises;
use exercises::*;

mod methods_and_traits;
use methods_and_traits::*;
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    // thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("Count in a Thread: {}", i);
    //         thread::sleep(Duration::from_millis(200));
    //     }
    // });

    // let handler = thread_test::thread_test1::test_panic();
    // // handler.join();

    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));

            if i == 5 {
                panic!();
            }
        }
    });
    for i in 1..10 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}

fn duplicate_1<T: Clone>(x: T) -> T {
    x.clone()
}

fn duplicate_2<T>(x: T) -> T
where
    T: Clone,
{
    x.clone()
}

fn min<T: Ord>(x: &T, y: &T) -> bool {
    match &x.cmp(&y) {
        Ordering::Less => true,
        _ => false,
    }
}

struct Coordinate {
    x: i32,
    y: i32,
}

fn destruct_coord() {
    let c = Coordinate { x: 1, y: 2 };

    let Coordinate { x: a, y: b } = c;

    println!("a = {}, b = {}", a, b);

    match c {
        Coordinate { x: 1, y: 2 } => println!("Found 1, 2"),
        Coordinate { x, y } => println!("Found {}, {}", x, y),
    }
}

fn pattern_matching(x: &char) {
    match x {
        'A' => println!("Found an A!"),
        'B' => println!("Found a B!"),
        _ => println!("Found something else!"),
    }
}

fn print_elevator_action() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_button_pressed(0, elevator::Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match self {
            Direction::Left => "1",
            Direction::Right => "2",
        }
    }
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn magnitude(geo: [f64; 3]) -> f64 {
    return (geo[0].powi(2) + geo[1].powi(2) + geo[2].powi(2)).sqrt();
}

fn normalize(mut n_geo: [f64; 3]) {
    let mag = magnitude(n_geo);

    for i in 0..3 {
        n_geo[i] = n_geo[i] / mag;
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut t_matrix: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            t_matrix[i][j] = matrix[j][i];
        }
    }

    return t_matrix;
}

fn collatz_sequence(mut n: i32) -> i32 {
    let mut l = 1;

    while n != 1 {
        println!("{}", n);

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }

        l += 1;
    }

    l
}

fn compare_3_fibs(n: u32) {
    let t1 = SystemTime::now();

    println!("fib({}) = {}", n, fib(n));

    let t2 = SystemTime::now();

    println!("fib takes: {}", t2.duration_since(t1).unwrap().as_nanos());

    println!("fib_iter({}) = {}", n, fib_iter(n));

    let t3 = SystemTime::now();

    println!(
        "fib_iter takes: {}",
        t3.duration_since(t2).unwrap().as_nanos()
    );

    println!("fib_memorize({}) = {}", n, fib_memorize(n));

    let t4 = SystemTime::now();

    println!(
        "fib_memorize takes: {}",
        t4.duration_since(t3).unwrap().as_nanos()
    );
}

fn interproduct(x: i32, y: i32, z: i32) -> i32 {
    return (x * y * z) as i32;
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn fib_iter(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    for _ in 0..n - 1 {
        c = a + b;
        a = b;
        b = c;
    }

    return c;
}

fn fib_memorize(n: u32) -> u32 {
    let mut memo = vec![0; n as usize + 1];
    return fib_memorize_sub(n, &mut memo);
}

fn fib_memorize_sub(n: u32, memo: &mut Vec<u32>) -> u32 {
    if n < 2 {
        return n;
    } else if memo[n as usize] != 0 {
        return memo[n as usize];
    } else {
        memo[n as usize] = fib_memorize_sub(n - 1, memo) + fib_memorize_sub(n - 2, memo);
        return memo[n as usize];
    }
}
