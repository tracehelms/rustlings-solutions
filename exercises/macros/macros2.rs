// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// Using the below code will let it work _anywhere_ in the file
// #[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
