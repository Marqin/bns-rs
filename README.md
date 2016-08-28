# bns-rs

Rust library that web scraps character information from certain MMO.

## Dependencies

```
curl = "^0.3.5"
scraper = "^0.4.0"
```

## Example usage

```rust
let chara = players::Character::new(
    String::from("PUT_API_DOMAIN_NAME_HERE"),
    String::from("eu"),
    String::from("SOME_CHARACTER_NAME")
).unwrap();
println!("{:?}", chara);
```

You have to guess what to put instead of `PUT_API_DOMAIN_NAME_HERE`, because
I don't want to be sued for misusing trademark.

## Additional info

`Character::new` returns `Result<Character, String>`, where two of possible errors are:
* INVALID_REGION
* CHARACTER_NOT_FOUND

This may change in future versions to some custom Error structs.
