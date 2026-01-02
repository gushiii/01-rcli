fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        // The assert_eq! macro checks if two values are equal
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        // This test will fail because panic! is called
        // panic!("Make this test fail");
    }
}
