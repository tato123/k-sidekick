use std::collections::HashMap;

pub trait Source {
    properties: HashMap;

    fn set(&self) -> String {
        String::from("hello world")
    }
}
