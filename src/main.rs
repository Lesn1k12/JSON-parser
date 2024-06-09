use std::any::Any;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde_json::Value;


fn main() -> Result<()> {
    // let file = File::open("./src/data.json")?;
    // let reader = BufReader::new(file);
    let file = fs::read_to_string("./src/data.json")?;
    let k: HashMap<String, Value> = serde_json::from_str(&file)?;

    let mut dict:HashMap<String, Box<dyn Any>> = HashMap::new();
    for (key, value) in k {
        dict.insert(key, Box::new(value));
    }

    for (key, value) in dict {
        println!("{}: {:?}", key, formating(&value));
    }

    Ok(())
}

fn formating(value: &Box<dyn Any>) -> String{
    if let Some(v) = value.downcast_ref::<Value>() {
        return v.to_string();
    }
    return "Unknown".to_string();
}
