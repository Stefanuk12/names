use rand::rngs::ThreadRng;

fn main() {
    let mut generated = serde_json::from_str::<names::Generator<ThreadRng>>(r#"{
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