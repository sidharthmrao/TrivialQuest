#[cfg(test)]
mod tests {
    use trivial_quest;

    #[test]
    fn it_works() {
        let result = trivial_quest::add(2, 2);
        assert_eq!(result, 4);
    }
}
