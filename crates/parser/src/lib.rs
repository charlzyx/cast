use anyhow::Result;
use serde_json::{self, Value};
use std::fs::File;

pub fn parser() -> Result<()> {
    let reader = File::open("./swagger.json")?;
    let json_schema: Value = serde_json::from_reader(reader)?;

    json_schema.as_object().unwrap().iter().for_each(|(k, v)| {
        println!("{}: {:?}", k, v);
    });
    Ok(())
}

fn main() -> Result<()> {
    parser()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!("YES" == "YES")
    }
}
