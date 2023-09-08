// ==================================task01 start
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait LightTime {
    fn time(&self) -> u32;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 5,
        }
    }
}

fn test01(){
    // task 01
    let red_light: TrafficLight = TrafficLight::Red;
    let green_light: TrafficLight = TrafficLight::Green;

    println!("light time:{}!", red_light.time());
    println!("light time:{}!", green_light.time());
}
// ==================================task01 end

// ==================================task02 start

fn sum_with_overflow_check(arr: &[u32]) -> Option<u32> {
    let mut total = 0u32;

    for &i in arr {
        match total.checked_add(i) {
            Some(result) => {
                total = result;
            }
            None => {
                return None;
            }
        }
    }

    return Some(total);
}

fn test02(){
    // task02
    let testarr: [u32; 3] = [4000000000, 4000000000, 4000000000];
    let sum = sum_with_overflow_check(&testarr);
    match sum {
        Some(result) => {
            println!("testarr sum:{}!", result);
        }
        None => {
            println!("overflow happend!");
        }
    }
}

// ==================================task02 end

// ==================================task03 start

trait Shape {
    fn area(&self)->f32;
}

struct Triangle{}
struct Circle{}
struct Square{}

impl Shape for Triangle{
    fn area(&self)->f32 {
        // todo
        return 1.0;
    }
}

impl Shape for Circle{
    fn area(&self)->f32 {
        // todo
        return 2.0;
    }
}

impl Shape for Square{
    fn area(&self)->f32 {
        // todo
        return 3.0;
    }
}

fn print_area<T: Shape>(i: T){
    println!("the area is: {}", i.area());
}

fn test03(){
    let square = Square{};
    print_area(square);
}

// ==================================task03 end

fn main() {
    test01();
    test02();
    test03();
}
