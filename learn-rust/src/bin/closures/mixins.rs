//https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

trait Wizard {
    fn fly(&self);
    fn name() -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
    fn name() -> String {
        String::from("pilot")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
    fn name() -> String {
        String::from("wizarrd")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fly_human_fly() {
        let man = Human;
        man.fly(); // Hman method
        Pilot::fly(&man); // choosing the correct trait
        Wizard::fly(&man);

        assert!(false);
    }

    #[test]
    fn fqn() {
        println!("A baby dog is called a {}", Dog::baby_name());
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
