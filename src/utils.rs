use serde_json::Value;

pub fn print_json_pretty(input: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(val) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&val).unwrap_or_else(|_| input.to_string())
            );
        }
        Err(_) => println!("{}", input),
    }
}

pub fn print_json_value(value: &Value) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}
