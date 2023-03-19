
fn main() {
    println!("a loop with labels");
    label_loop();

    println!("simple while loop");

    let mut x = 12;

    while x != 0 {
        println!("{x}");
        x -= 2;
    }

    println!("a nice for loop");
    // let a = [10,20,30,40,50];

    for element in (10..=20).rev() {
        println!("{element}");
    }


}

fn label_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaning = {remaining}");
            if remaining == 9 {
                break;
            }
            else if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("count = {count}");
}