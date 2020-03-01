use std::collections::HashMap;

fn main(){
    play_with_collects();
}

pub fn play_with_collects() {
    concatinating();
}

fn accessing() {
    let v: Vec<u128> = Vec::new();
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("vec is vec {:?}, da {}", v, *third); //same
    println!("vec is vec {:?}, da {}", v, third); //same
    println!("vec is vec {:?}, da {}", v, v[2]); //same
}

fn no_mutable_and_imutable_in_same_scope() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); cannot borrow as mutable since its borrowed as imutable
    println!("The first element is: {}", first);
}

fn edit_in_place() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // every variable is also a pointer.... ?
    }
    println!("da {:?} v", v);
}

fn enum_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in row {
        match i {
            SpreadsheetCell::Int(x) => println!("x {}", x),
            SpreadsheetCell::Text(x) => println!("x {}", x),
            SpreadsheetCell::Float(x) => println!("x {}", x),
        }
    }

    let mut da = vec![1, 2, 3];
    for i in &mut da {
        *i += 100;
    }
}

fn stringss() {
    let data = "initial contents";
    let s = data.to_string();
    print!("{}", s);
}

fn concatinating() {
    let s1 = String::from("tic");
    let mut s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    s2 = String::from("xxx");
    println!("{}", s);
}

fn vec_of_stracts() {
    #[derive(Debug)]
    struct SSS {
        Id: String,
        name: u32,
    }

    let mut sss = SSS {
        Id: "da".to_string(),
        name: 2,
    };

    fn rename_ss(v: &Vec<&SSS>) {
        println!("{:?}", v);
    };

    let v: Vec<&SSS> = vec![&sss];

    rename_ss(&v);
    println!("{:?}", v);
}

fn update_entry_in_a_map() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
