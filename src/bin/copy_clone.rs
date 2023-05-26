// #[derive(Copy, Clone, Debug)]
// struct Point(i32, i32);
#[derive(Clone, Debug)]
struct Point(i32, i32, String);

fn main() {
    // let p1 = Point(3, 4);
    // let p2 = p1;
    let p1 = Point(3, 4, String::from("Hello"));
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
