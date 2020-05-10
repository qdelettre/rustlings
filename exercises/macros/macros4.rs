// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

// I AM DONE

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

#[macro_use]
fn main() {
    my_macro!();
    my_macro!(7777);
}
