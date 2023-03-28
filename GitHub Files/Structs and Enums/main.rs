#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions (kinda like static functions)
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    } 
}


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Ardnys"),
        email: String::from("ardnys@ardnysmail.com"),
        sign_in_count: 1,
    };

    println!("username: {0}", user1.username);
    // user1.username = String::from("Sydnra");
    // println!("username: {0}", user1.username);

    user1 = build_user(String::from("greenteamails.com"), String::from("Cupholder"));
    println!("name: {0}\nmail: {1}\nactive: {2}\ncount: {3}\n", user1.username, user1.email, user1.active, user1.sign_in_count);

    // struct update syntax
    // this moves data so be careful with it
    // fields that don't implement copy trait are no longer valid
    let user2 = User {
        username: String::from("jasmine (it's a tea reference)"),
        ..user1
    };
    // this is hard to write
    println!("name: {0}\nmail: {1}\nactive: {2}\ncount: {3}\n", user2.username, user2.email, user1.active, user1.sign_in_count);
    println!("user2 is {:?}", user2);
    // takes ownership and prints to stderr
    // dbg!(&user2);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("r of black {0}\nx of origin {1}", black.0, origin.0);

    let rect1 = Rectangle {
        width: 12,
        height: 23,
    };
    println!("the area of rectangle is {} square pixels",
        rect1.area()
    );

    let square1 = Rectangle::square(15);
    println!("Area of square is {}", square1.area());


}

fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}
