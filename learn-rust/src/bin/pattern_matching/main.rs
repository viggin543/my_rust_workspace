fn main() {
    while_let();
    for_loop();
    method_params();
    let_destruction();
    value_matching();
    exact_composite_value_matching();
    multipale_pattern();
    range_pattern();
    destruct_nested_Struct_and_enum();
    the_at_operator();
}

fn mathc_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn remaining_parts_ignore() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn destruct_nested_Struct_and_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn struct_destrucation_matching() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 }; // destruction

    let Point { x: a, y: b } = p;
    let Point { x, y } = p; // destruction even shorter!
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(7, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn exact_composite_value_matching() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}

fn value_matching() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn let_destruction() {
    // let destruction
    let (x, y, z) = (1, 2, 3);
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
}

fn while_let() {
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("poped {}", x);
    }
}

fn for_loop() {
    // for loop destruction
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    for (index, _) in v.iter().enumerate() {
        // ignore _
        println!("is at index {}", index);
    }
}

fn the_at_operator() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn multipale_pattern() {
    let x = 1;
    match x {
        //multiple patterns
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 4;
    let y = false;

    match x {
        // multi with guard
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn range_pattern() {
    let x = 5;
    match x {
        //range pattern
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn destructing_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn method_params() {
    let point = (3, 5);
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    print_coordinates(&point);
}
