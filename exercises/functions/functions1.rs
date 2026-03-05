// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(5);
}

fn call_me(num: i32) {
    for _ in 0..num {
        println!("Ring! Call me!");
    }
}
