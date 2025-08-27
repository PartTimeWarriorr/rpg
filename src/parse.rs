use serde::*;
use serde_json::*;
use std::fs::File;
use std::io::BufReader;


#[derive(Debug, Serialize, Deserialize)]
pub enum TestEnum {
    A,
    B
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestStruct {
    name: String,
    value: TestEnum,
}

pub fn parse() -> Result<()> {

    let file = File::open("src/abilities.json").expect("Character config file not found.");
    let reader = BufReader::new(file);

    let val : Vec<TestStruct> = serde_json::from_reader(reader).expect("Bad JSON formatting");
    
    dbg!(&val);

    Ok(()) 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        parse();
    }
}