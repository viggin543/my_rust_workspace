#[macro_export]
macro_rules! DA {
//https://doc.rust-lang.org/book/ch19-06-macros.html
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v: Vec<i8> = DA!(1, 2, 3);
}
