fn main() {
    let mut generated = serde_json::from_str::<names::Generator>(r#"{
        "casing": "CamelCase"
    }"#).unwrap();

    println!("My new name is: {}", generated.next().unwrap());
}