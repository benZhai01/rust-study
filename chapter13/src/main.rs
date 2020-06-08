use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout3(simulated_user_specified_value, simulated_random_number);
    test_environment();
    iterator_sum();
    iterator_filter(20);
    iterator_custom();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32){
    if intensity < 25 {
        println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
        println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
        }
    }
}

pub fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

pub fn generate_workout3(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn test_environment() {
    let x = vec![1,2,3];
    let equal_x = |z| -> bool { z == x };
    //let equal_x = move |z| -> bool { z == x };
    let y = vec![1,2,3];
    println!("{}", x[0]);
    println!("{}",equal_x(y));

}

fn iterator_sum() {
    let v1 = vec![1,2,3,4];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("{}", sum);

    let v2 = vec![1,2,3,4];
    let v2_iter = v2.iter().map(|x| x + 1);
    let v2_c: Vec<_> = v2_iter.collect();
    println!("{}", v2_c[0]);
}

fn iterator_filter(std: u32) {
    let v1 = vec![10, 20,30, 20, 20, 20, 02];
    let v1_filter = v1.iter().filter(|s| *s == &std);
    let v1_res: Vec<_> = v1_filter.collect();
    println!("{}", v1_res.len());
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else {
            None
        }
    }
}

fn iterator_custom() {
    
    let mut counter = Counter::new();
    println!("{}", counter.next().is_some());
    println!("{}", counter.next().is_some());
    println!("{}", counter.next().is_some());
    println!("{}", counter.next().is_some());
    println!("{}", counter.next().is_some());
    println!("{}", counter.next().is_some());

    let counter2 = Counter::new();
    let c_iter = counter2.filter(|i| i%2 != 0);
    let res: Vec<_> = c_iter.collect();
    println!("{}", res.len());
}

