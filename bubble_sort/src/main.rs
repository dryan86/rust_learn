use std::{cmp::Ordering, fmt::Debug};

// 冒泡排序
fn bubble_sort<T:PartialOrd>(arr:&mut Vec<T>){
    let len = arr.len();

    for i in 0 .. len {
        for j in 0 .. len - i - 1 {
            if arr[j] > arr[j+1]{
                arr.swap(j, j + 1);
            }
        }
    }
}

// 自定义类
struct Player{
    age: i32,
    name: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.age.eq(&other.age);
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.age.partial_cmp(&other.age) {
            Some(Ordering::Equal)=>{
                self.name.partial_cmp(&other.name)
            }
            other => other,
        }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("name:{}, age:{}", self.name, self.age))
    }
}

fn main() {
    println!("普通类型的排序:");
    let mut test1 = vec![3,1,5,7,9,23,11,23,1,51];
    println!("befort sort:{:?}", test1);
    bubble_sort(&mut test1);
    println!("after sort:{:?}", test1);

    // test custom type
    println!("自定义类型的排序:");
    let p1: Player = Player{age:32, name:String::from("player1")};
    let p2: Player = Player{age:23, name:String::from("player2")};
    let p3: Player = Player{age:42, name:String::from("player3")};
    let p4: Player = Player{age:62, name:String::from("player4")};
    let p5: Player = Player{age:29, name:String::from("player5")};


    let mut test_obj: Vec<Player> = vec![p1, p2, p3, p4, p5];
    println!("befort sort player:\n{:#?}", test_obj);
    bubble_sort(&mut test_obj);
    println!("after sort player:\n{:#?}", test_obj);
}
