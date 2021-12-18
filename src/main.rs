use std::u8;

fn main() {
    println!("Hello, world!");

    /* Variables */
    // basic numeric variable
    let un8: u8 = 1;
    let sum = 5 + 10;
    let x = 2.0;
    let y: f32 = 3.0;

    // string variable
    let heart_eyes_cat = "☺️";
    println!("{}", heart_eyes_cat);

    // tuple variable
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y의 값: {}", y);

    // array variable
    let a = [1, 2, 3, 4, 5];
    let triple_five = [5; 3];
    // access to element
    let first = a[0];
    println!("First element of array: {}", first);

    println!("------------ end of variables ------------");

    /* Function */
    // call function
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("y의 값: {}", y);

    // function to return value
    let x = five();
    println!("value of x: {}", x);

    println!("------------- end of function ------------");

    let number = 3;
    if number < 5 {
        // 동적 언어들과는 다르게 정확하게 boolean 의 값만 사용해야함
        println!("조건이 일치합니다");
    } else {
        println!("조건이 일치하지 않습니다");
    }

    let condition = true;
    let number = if condition { 5 } else { 22 }; // rust는 삼항연산자가 없다

    let mut counter = 0;
    // loop can return a value
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("value of result variable: {}", result);

    let mut number = 3;
    // while
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("발사!!!");

    let a = [10, 20, 30, 40, 50];

    // for
    for element in a.iter() {
        println!("value of element: {}", element);
    }

    // for with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!!!");

    println!("----------- end of flow control ----------");
}

// void function
fn another_function(x: i32, y: i32) {
    println!("value of x: {}", x);
    println!("value of y: {}", y);
}

// function to return value
fn five() -> i32 {
    5 // return line must not exist semi colon.
}
