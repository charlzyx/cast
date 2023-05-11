use anyhow::Result;
use log;
use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};
use std::{any::type_name, array, collections::HashMap, fs::File, ops::Add};

#[derive(Debug, Serialize, Deserialize)]
struct DefSchema {
    #[serde(rename = "type")]
    schema_type: Option<String>,
    #[serde(rename = "$ref")]
    ref_name: Option<String>,
    schema: Option<Box<DefSchema>>,
    description: Option<String>,
    properties: Option<HashMap<String, DefSchema>>,
    items: Option<Box<DefSchema>>,
    required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParameterSchema<DefSchema> {
    /// body | query | path | formData | header
    #[serde(rename = "in")]
    param_in: String,
    required: Option<bool>,
}

#[derive(Debug)]
struct Definition {
    name: String,
    props: HashMap<String, DefSchema>,
    def: Value,
}

impl Definition {
    pub fn new(name: &str, props: HashMap<String, DefSchema>, def: Value) -> Self {
        Definition {
            name: name.to_owned(),
            props,
            def,
        }
    }
    pub fn stringify(&self) -> String {
        let mut ts_str = vec!["".to_owned()];
        let name = esapce_name(self.name.as_str());

        let prefix = ["export interface ", name.as_str(), " {"].join(" ");
        ts_str.push(prefix);
        let required = self.def.get("required");
        for (name, prop) in self.props.iter() {
            let ts = def_schema_to_string(name, prop, &get_required_map(required));
            ts_str.push(ts)
        }
        ts_str.push("}".to_owned());
        ts_str.join("\n")
    }
}

fn get_required_map(value: Option<&Value>) -> HashMap<String, bool> {
    match value {
        Some(listlike) => {
            let arraylike = listlike.as_array();
            match arraylike {
                Some(list) => list
                    .iter()
                    .map(|x| (x.as_str().unwrap_or("").to_owned(), true))
                    .collect::<HashMap<String, bool>>(),
                None => HashMap::new(),
            }
        }
        None => HashMap::new(),
    }
}
fn esapce_name(name: &str) -> String {
    name.replace("«", "__GENERICS_LEFT__")
        .replace("»", "__GENERICS_RIGHT__")
        .to_owned()
}

fn def_schema_to_string(
    name: &str,
    schema: &DefSchema,
    required_map: &HashMap<String, bool>,
) -> String {
    // «CouponRespDTO»
    let mut prefix;
    let suffix;
    if name.eq("") {
        prefix = name.to_owned();
        suffix = "".to_owned();
    } else {
        prefix = name.to_owned();
        match required_map.get(name) {
            Some(_) => prefix = prefix.add(":"),
            None => prefix = prefix.add("?:"),
        }

        suffix = ";\n".to_owned();
    };

    let comment = "/** ___ */\n"
        .replace(
            "___",
            schema
                .description
                .to_owned()
                .unwrap_or("".to_owned())
                .as_str(),
        )
        .replace("/**  */", "");
    let mut types = vec![comment, prefix];

    // refs
    if schema.ref_name.is_some() {
        let def_ref_str = schema.ref_name.as_ref().unwrap();
        // println!("ref: {}", ref_str);
        let ref_str = def_ref_str.replace("#/definitions/", "Apidefs.");

        types.push(ref_str.to_owned());
    } else if schema.schema_type.is_some() {
        // println!("type: {}", schema.schema_type.as_ref().unwrap());
        let item_type = schema.schema_type.as_ref().unwrap();
        match item_type.as_str() {
            "string" => types.push("string".to_owned()),
            "integer" => types.push("number".to_owned()),
            "number" => types.push("number".to_owned()),
            "boolean" => types.push("boolean".to_owned()),
            "null" => types.push("null".to_owned()),
            "object" => {
                match &schema.properties {
                    Some(mapping) => {
                        for (_, child) in mapping {
                            let child_str = def_schema_to_string(name, child, &HashMap::new());
                            types.push(child_str)
                        }
                    }
                    None => types.push("Record<string, any>".to_owned()),
                }

                //  object get properties
            }
            "array" => {
                // array get items
                types.push("Array<".to_owned());
                // parset item
                match &schema.items {
                    Some(item_def) => {
                        let item_str = def_schema_to_string("", &item_def, &HashMap::new());
                        types.push(item_str.trim().to_owned())
                    }
                    None => types.push("any".to_owned()),
                }
                types.push(">".to_owned());
            }
            _ => types.push("any".to_owned()),
        }
    }

    types.push(suffix);

    types.join("")
}

fn parse_defs(mapping: &Map<String, Value>) -> HashMap<&String, Definition> {
    let mut defs = HashMap::new();

    for name in mapping.keys() {
        // println!("name: {}", name);
        let mut props = HashMap::new();

        let def = mapping.get(name).unwrap();

        if def.get("properties").is_some() {
            let properties = def.get("properties").unwrap().as_object().unwrap();

            for (name, v) in properties {
                let schema: DefSchema = serde_json::from_value(v.clone()).unwrap();
                props.insert(name.to_owned(), schema);
            }
        }

        defs.insert(name, Definition::new(name, props, def.to_owned()));
    }

    defs
}

#[derive(Debug)]
struct ReqRespInfo {
    body: Option<String>,
    query: Option<String>,
    path: Option<String>,
    form_data: Option<String>,
    header: Option<String>,
    resp: Option<String>,
}

impl ReqRespInfo {
    pub fn new() -> Self {
        ReqRespInfo {
            body: (),
            query: (),
            path: (),
            form_data: (),
            header: (),
            resp: (),
        }
    }
}

fn parse_paths(mapping: &Map<String, Value>) -> HashMap<&String, Vec<ParameterSchema>> {
    let mut path_objects = HashMap::new();
    let operations = ["post", "get", "put", "delete", "patch"];

    for req_path in mapping.keys() {
        let path_item_object = mapping.get(req_path).unwrap();

        for method in operations {
            match path_item_object.get(method) {
                Some(operation_object) => {
                    let req_res_info = ReqRespInfo::new();

                    if let Some(paramslist) = operation_object.get("parameters") {
                        let list: Vec<ParameterSchema> =
                            serde_json::from_value(paramslist.to_owned()).unwrap();
                        for param in list {
                            match param.param_in.as_str() {
                                "body" => {
                                    // req_res_info.body = param
                                }
                                "query" => {}
                                "path" => {}
                                "formData" => {}
                                "header" => {}
                            }
                        }
                    }

                    if let Some(responsesok) = operation_object.get("responses.200") {
                        let ok: DefSchema = serde_json::from_value(responsesok.to_owned()).unwrap();
                    }
                }
                None => (),
            }
        }

        // if def.get("properties").is_some() {
        //     let properties = def.get("properties").unwrap().as_object().unwrap();

        //     for (name, v) in properties {
        //         let schema: DefSchema = serde_json::from_value(v.clone()).unwrap();
        //         props.insert(name.to_owned(), schema);
        //     }
        // }

        // defs.insert(req_path, Definition::new(req_path, props, def.to_owned()));
    }

    defs
}

pub fn parser() -> Result<()> {
    let reader = File::open("swagger.json")?;
    let json_schema: Value = serde_json::from_reader(reader)?;

    json_schema.as_object().unwrap().iter().for_each(|(k, v)| {
        let defs;
        if "definitions".eq(k) {
            let defs_values = v.as_object().unwrap();
            defs = parse_defs(defs_values);
            // to namespace dts
            let mut ts_str = vec!["declare namespace Apidefs {".to_owned()];
            for (_, def) in defs.iter() {
                ts_str.push(def.stringify());
                // println!(" // interface of: {} \n{}", name, def.stringify());
            }
            ts_str.push("}".to_owned());
            let dts = ts_str.join("\n");

            println!("{}", dts);
        }
        if "paths".eq(k) {}
    });
    Ok(())
}

fn main() {
    env_logger::init();
    log::debug!("Initialized logger");
    let ans = parser();

    let ok = match ans {
        Ok(_) => true,
        Err(err) => {
            log::error!("{:?} Error parsing swagger.json", err);
            false
        }
    };

    println!("Hello, world!, {}", ok);
}
