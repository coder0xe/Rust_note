use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 20;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2)); // 模拟计算时长
//     intensity
// }

struct Cache<T> 
    where T: Fn(u32) -> u32 // 闭包类型
{
    calculation: T,
    value_map: HashMap<u32, u32>,
}

impl<T> Cache<T> 
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value_map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cache::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2)); // 模拟计算时长
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure.value(intensity));
    }
}

#[cfg(test)]
mod tests {
    use crate::Cache;

    #[test]
    fn test01() {
        let mut cacher = Cache::new(|x| x);
        let one = cacher.value(1);
        let two = cacher.value(2);
        assert_eq!(one, 1);
        assert_eq!(two, 2);
    }
}