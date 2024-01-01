// NOTE: Every `.unwrap()` seen here is SAFE, just make sure that `adjectives` and `nouns` are not empty.

fn main() {
    let mut generated = serde_json::from_str::<names::Generator>(r#"{
        "casing": "CamelCase",
        "naming": {
            "ZeroPaddedNumbered": [2, "_"]
        },
        "length": {
            "Truncate": 20
        }
    }"#).unwrap();

    println!("My new name is: {}", generated.next().unwrap());
}