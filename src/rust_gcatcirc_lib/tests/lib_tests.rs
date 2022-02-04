#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use rust_gcatcirc_lib::say_some;
        println!("{}", u32::MAX);
        assert_eq!(say_some(), "no id (['A'])) { A }");
    }
}