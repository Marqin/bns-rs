pub mod players;

#[cfg(test)]
mod tests {
    use players;

    #[test]
    fn test_ok() {
        let x = players::Character::new(
            String::from("PUT_DOMAIN_NAME_HERE"),
            String::from("eu"),
            String::from("Marqin")
        ).unwrap();
        println!("{:?}", x);
    }

    #[test]
    fn test_not_found() {
        let x = players::Character::new(
            String::from("PUT_DOMAIN_NAME_HERE"),
            String::from("eu"),
            String::from("not existent character")
        );
        assert_eq!(x.err(), Some("CHARACTER_NOT_FOUND".to_string()));
    }

    #[test]
    fn test_invalid_region() {
        let x = players::Character::new(
            String::from("PUT_DOMAIN_NAME_HERE"),
            String::from("invalid"),
            String::from("Marqin")
        );
        assert_eq!(x.err(), Some("INVALID_REGION".to_string()));
    }
}
