/**
* Practice with structs and impl(s)
*/

fn main() {
    let rect1 = Rectangle {
        height: 3,
        width: 5,
    };

    // pretty print an object that has Debug trait w/ {:#?}
    println!("Rectangle {:#?}", rect1);

    // requires reference (&) to rect1
    println!("Area via static method: {}", Rectangle::area(&rect1));

    // area() can be called directly as a method on rect1 instance
    println!("Area via regular method: {}", rect1.area());

    let box1 = Box {
        height: 2.3,
        width: 2.3,
        depth: 2.3
    };

    let box2 = Box {
        height: 2.3,
        ..box1
    };

    println!("Is box1 a cube? {}", box1.is_cube());
    println!("Is box2 a cube? {}", box2.is_cube());

    println!("box1 volume: {}", box1.volume());
    println!("box2 volume: {}", box2.volume());

    let app_is_loading = AppState::Loading(String::from("App is loading..."));
    let app_has_data = AppState::Data(String::from("App has data and is ready to render!"));
 
    run_app(&app_is_loading);
    run_app(&app_has_data);

    let safe_val = match safe_add(Some(1), None) {
        Some(value) => println!("safe_val has data {:#?}", value),
        None => println!("safe_val has no data")
    };
}

fn safe_add(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    Some(a.unwrap_or(0) + b.unwrap_or(0))
}

#[derive(Debug, Clone, Copy)] // so that we can print this struct to the console
struct Box<T> {
    height: T,
    width: T,
    depth: T,
}

impl<T: std::cmp::PartialEq + std::ops::Mul<Output = T>> Box<T> {
    fn is_cube(&self) -> bool {
        self.height == self.width && self.height == self.depth
    }
    fn volume(&self) -> T {
        self.height * self.width * self.depth
    }
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

// Rust Enums, like algebraic data types, are custom data types with different
// variants and each enum instance can only be in one variant at a time

// Enums are kind of like how a finite state machine (FSM) can only 
// be in one state out of a set of predefined states
#[derive(Debug)]
enum AppState {
    Loading(String),
    Data(String),
}

fn run_app(app_state: &AppState) {
    match app_state {
        AppState::Loading(state) => println!("{}", state),
        AppState::Data(state) => println!("{}", state)
    }
}