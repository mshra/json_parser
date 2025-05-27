mod lexer;

fn main() {
    // raw json string
    let json = r#"
    {
        "string": "hello \"world\" \\ / \b \f \n \r \t \u1234",
        "number_integer": 123,
        "number_negative": -456,
        "number_float": 78.9,
        "negative_number_float": -85.545,
        "number_exp": 6.022e23,
        "number_exp_neg": -1.6e-19,
        "boolean_true": true,
        "boolean_false": false,
        "null_value": null,
        "empty_object": {},
        "empty_array": [],
        "nested_array": [1, 2, [3, 4], {"nested": "object"}],
        "nested_object": {
            "level1": {
                "level2": {
                    "level3": "deep value"
                }
            }
        },
        "array_of_objects": [
            { "id": 1, "valid": true },
            { "id": 2, "valid": false }
        ],
        "complex_escape": "This is a line\\nwith escapes\\tand unicode: \\u00A9",
        "unicode_emoji": "😀 😁 😂 🤖 🦀"
    }
    "#;

    let tokens = lexer::lexer(String::from(json)).unwrap();
    println!("{:?}", tokens);
    println!("{}", json);
}
