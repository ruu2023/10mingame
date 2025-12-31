fn main() {
    let x = 5;
    let mut y = 10;
    y = 15;
    
    const MAX_POINTS: u32 = 100_000;
    println!("{} {} {}", MAX_POINTS, x, y);
    println!("{}", add(1, 2));
    let status = if x > y { "greater" } else { "less" };
    println!("{status}");
    let list = vec![1, 2, 3];

    println!("中身は: {:?}", list); 
    println!("整形表示: {:#?}", list);
    let user = User {
        id: 1,
        name: String::from("Taro"),
        active: true,
        point: Point { x: 1, y: 2 },
    };

    println!("{:?}", user);
    println!("{:#?}", user);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
    point: Point,
}