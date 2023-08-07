fn main() {
    println!("Hello, world!");

    loop {
        println!("again!");
        println!("yipee!");
        //needed to get commented since it starts an infinite loop
        //continue;
        //println!("redundant 😠");
        break;
    }

    let mut counter = 0;

    //declaring using loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };
    println!("result:{result}");

    //Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //advanced
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    //1..4 is a Range, included in the standard libraryđ

    let a = [5; 10];
}
