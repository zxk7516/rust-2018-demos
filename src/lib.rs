use log::{error, warn};

#[macro_export]
macro_rules! baz {
    () => {
        println!("baz!");
    };
}

pub fn test() {
    warn!("log::warn!");
    error!("log::error!");
    println!("test");
}
