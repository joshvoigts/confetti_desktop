#[macro_export]
macro_rules! p {
    ($($opt:expr),*) => {
        {
            $(
                print!("{:?} ", $opt);
             )*
        }
        println!();
    };
}
