mod lexer;

fn main() {
    // raw json string
    let json = r#"
    {
        "name": "Alice",
        "age": 30,
        "height": 5.5,
        "address": {
            "city": "Wonderland",
            "zip": 12345
        },
        "is_student": false,
        "extra": null
    }
    "#;

    let tokens = lexer::lexer(String::from(json)).unwrap();
    println!("{:?}", tokens);
    println!("{}", json);
}
