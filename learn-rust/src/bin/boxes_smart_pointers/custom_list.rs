use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn print_a_list() {
    if let List::Cons(first, tail) = make_a_list() {
        println!("first list element! {}", first);
        print_tail(&tail); // pass &Box<List>
    };
}

fn print_tail(tail: &List) {
    // receive just &L
    if let List::Cons(first, tail) = tail {
        println!("printing tail ! {} {:?}", first, *tail);
    }
}
fn make_a_list() -> List {
    List::Cons(
        3,
        Box::new(List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))),
    )
}

fn int_on_heap() {
    let b = Box::new(5);
    println!("an int stored on the heap -> b = {}", b);
}

pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::RcList::*;
    use super::*;

    #[test]
    fn pribt_list() {
        print_a_list()
    }

    #[test]
    fn reference_counting() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a)); // instead of depp copy this only increaces the reference counting
        let c = Cons(4, Rc::clone(&a));
    }

    #[test]
    fn reference_counting_complex() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        assert_ne!(0, 0);
    }
}
