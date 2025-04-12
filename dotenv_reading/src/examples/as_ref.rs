pub struct MyStruct;

impl AsRef<str> for MyStruct {
    fn as_ref(&self) -> &str {
        "my_struct"
    }
}

pub fn hello<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref())
}
