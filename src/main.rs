use std::fs::File;
use std::vec;

use crate::cast::cast::Castable;
use openapi::OpenAPI as OpenAPIV3;
use openapi::VersionedOpenAPI as OpenAPIV2;

mod cast;

fn main() {
    fn run(list: Vec<(&str, bool)>) {
        for (path, v3) in list {
            let filepath = format!(".debug/{}", path);
            let file = File::open(filepath).unwrap();
            if v3 {
                let openapi: OpenAPIV3 = serde_json::from_reader(file).unwrap();
                println!("{}", openapi.to_tds());
            } else {
                let openapi: OpenAPIV2 = serde_json::from_reader(file).unwrap();
                let openapi: OpenAPIV3 = openapi.upgrade();
                println!("{}", openapi.to_tds());
            }
        }
    }

    run(vec![
        // ("openapi.json", true),
        ("petstore.json", false),
        // ("swagger.json", false),
    ]);

    // fix_interface_name_all_of("export interface Pet NewPet & Error & { id: number & string");
}
