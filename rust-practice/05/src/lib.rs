#[test]
fn hello_works() {
    assert_eq!("Hello", "Hello");
}


pub fn add_two(a: i32) -> i32{
    a + 2
}


#[cfg(test)]
mod tests{
    use super::add_two;

    #[test]
    fn it_works(){
        assert_eq!(4, add_two(2));
    }
}
