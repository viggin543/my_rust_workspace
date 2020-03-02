mod channels;
mod mutex;
mod threads;

fn main() {
    threads::two_threads();
    threads::move_closure();
    channels::simple_blocking_channel();
}
