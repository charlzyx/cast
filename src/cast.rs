pub mod cast {
    use openapi::*;
    use regex::Regex;
    use std::vec;

    fn safe_name(name: &str) -> String {
        let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
        re.replace_all(name, "_").to_string()
    }

    fn pad(indent: usize, str: &str) -> String {
        let mut s = String::new();
        for _ in 0..indent {
            s.push_str(" ");
        }
        s.push_str(str);
        s
    }

    pub trait Castable {
        fn to_tds(&self) -> String;
        fn js_docs(&self) -> String {
            "".to_owned()
        }
    }

    impl Castable for ReferenceOr<Box<Schema>> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }
        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<Schema> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }
        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<ParameterData> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }

        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<PathItem> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }

        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<Operation> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }
        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<RequestBody> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }

        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ReferenceOr<Response> {
        fn js_docs(&self) -> String {
            match self {
                ReferenceOr::Item(inner) => inner.js_docs(),
                _ => "".to_owned(),
            }
        }

        fn to_tds(&self) -> String {
            match self {
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split("/").last().unwrap().to_owned();
                    safe_name(&ref_name)
                }
                ReferenceOr::Item(inner) => inner.to_tds(),
            }
        }
    }

    impl Castable for ParameterData {
        fn js_docs(&self) -> String {
            let schema = self;

            let mut jsdocs = vec![];
            let mut empty = true;
            jsdocs.push("\n/**".to_owned());

            if let Some(str) = &schema.description {
                empty = false;
                jsdocs.push("@description ".to_owned() + str)
            }
            if let Some(deprecated) = &schema.deprecated {
                empty = false;
                if deprecated.to_owned() {
                    jsdocs.push("@deprecated ".to_owned())
                }
            }

            if empty {
                return "".to_owned();
            }

            jsdocs.push("*/\n".to_owned());

            jsdocs.join("\n")
        }
        fn to_tds(&self) -> String {
            let param = self;
            let required = param.required;
            let question = if required { "" } else { "?" };
            let js_docs = self.js_docs();

            let typings = match &param.schema() {
                Some(schema) => schema.to_tds(),
                None => "string;\n".to_owned(),
            };

            let mut name = param.name.to_owned();
            if name.contains(".") {
                name = vec!["\"", &name, "\""].join("");
            }

            let dts = [js_docs, name, question.to_owned(), ": ".to_owned(), typings].join("");

            dts
        }
    }

    impl Castable for RequestBody {
        fn to_tds(&self) -> String {
            match &self.content.get("application/json") {
                media => {
                    if let Some(media) = media {
                        if let Some(schema) = &media.schema {
                            schema.to_tds()
                        } else {
                            "".to_owned()
                        }
                    } else {
                        "".to_owned()
                    }
                }
            }
        }
    }

    impl Castable for Response {
        fn to_tds(&self) -> String {
            match &self.content.get("application/json") {
                media => {
                    if let Some(media) = media {
                        if let Some(schema) = &media.schema {
                            schema.to_tds()
                        } else {
                            "".to_owned()
                        }
                    } else {
                        "".to_owned()
                    }
                }
            }
        }
    }

    impl Castable for Operation {
        fn to_tds(&self) -> String {
            let mut lines = vec![self.operation_id.to_owned().unwrap()];
            lines.push(": {\n".to_owned());

            let mut in_querys = vec![];
            let mut in_paths = vec![];
            let mut in_headers = vec![];
            let mut in_cookies = vec![];

            self.parameters.iter().for_each(|param| match param {
                ReferenceOr::Reference { reference } => {
                    let line = reference.split("/").last().unwrap().to_owned();
                    match reference.split("/").nth(1).unwrap() {
                        "parameters" => {
                            in_querys.push(line);
                        }
                        "headers" => {
                            in_headers.push(line);
                        }
                        "paths" => {
                            in_paths.push(line);
                        }
                        "cookies" => in_cookies.push(line),
                        _ => {}
                    }
                }
                ReferenceOr::Item(inner) => {
                    let line = inner.parameter_data_ref().to_tds();
                    match inner {
                        Parameter::Query {
                            parameter_data: _,
                            allow_reserved: _,
                            style: _,
                            allow_empty_value: _,
                        } => {
                            in_querys.push(line);
                        }
                        Parameter::Header {
                            parameter_data: _,
                            style: _,
                        } => {
                            in_headers.push(line);
                        }
                        Parameter::Path {
                            parameter_data: _,
                            style: _,
                        } => {
                            in_paths.push(line);
                        }
                        Parameter::Cookie {
                            parameter_data: _,
                            style: _,
                        } => in_cookies.push(line),
                    }
                }
            });
            //  paramters
            let mut parameters = vec![];
            if in_querys.len() > 0 {
                parameters.push("Query: {\n".to_owned());
                parameters.push(in_querys.join("\n"));
                parameters.push("\n},".to_owned());
            }
            if in_paths.len() > 0 {
                parameters.push("Path: {\n".to_owned());
                parameters.push(in_paths.join("\n"));
                parameters.push("\n},".to_owned());
            }

            if in_cookies.len() > 0 {
                parameters.push("Cookie: {\n".to_owned());
                parameters.push(in_paths.join("\n"));
                parameters.push("\n},".to_owned());
            }

            if in_headers.len() > 0 {
                parameters.push("Header: {\n".to_owned());
                parameters.push(in_paths.join("\n"));
                parameters.push("\n}".to_owned());
            }

            if parameters.len() > 0 {
                lines.push("Parameters: {\n".to_owned());
                lines.push(parameters.join("\n"));
                lines.push("\n},".to_owned());
            }

            if let Some(request_body) = &self.request_body {
                let body = request_body.to_tds();
                if !body.eq("") {
                    lines.push(format!("RequestBody: {},", body));
                }
            }
            if let Some(response) = &self.responses.responses.get(&StatusCode::Code(200)) {
                let response = response.to_tds();
                if !response.eq("") {
                    lines.push(format!("Response: {},", response));
                }
            }
            lines.push("},\n".to_owned());

            lines.join(" ")
        }
    }

    impl Castable for PathItem {
        fn to_tds(&self) -> String {
            let mut lines = vec![];
            // let methods = ["get", "post", "patch", "delete"];
            let mut i = 0;

            [&self.get, &self.post, &self.patch, &self.delete]
                .into_iter()
                .for_each(|option| {
                    if let Some(operation) = option {
                        // let method = methods[i];
                        // lines.push([method.to_uppercase(), ":{".to_owned()].join(""));
                        lines.push(operation.to_tds());
                        // lines.push("}".to_owned());
                    }

                    i = i + 1;
                });

            lines.join("\n")
        }
    }

    impl Castable for Schema {
        fn js_docs(&self) -> String {
            let schema = self;
            let mut jsdocs = vec![];
            let mut empty = true;

            jsdocs.push("\n/**".to_owned());

            if let Some(str) = &schema.schema_data.title {
                empty = true;
                jsdocs.push("@title ".to_owned() + str)
            }
            if let Some(str) = &schema.schema_data.description {
                empty = true;
                jsdocs.push("@description ".to_owned() + str)
            }
            if &schema.schema_data.deprecated == &true {
                empty = true;
                jsdocs.push("@deprecated ".to_owned())
            }
            jsdocs.push("*/\n".to_owned());

            if empty {
                return "".to_owned();
            }

            jsdocs.join("\n")
        }
        fn to_tds(&self) -> String {
            let mut lines = vec![];
            let schema = self;
            match &schema.schema_kind {
                SchemaKind::Not { not } => {
                    lines.push("null".to_owned());
                }
                SchemaKind::Any(_) => lines.push("any".to_owned()),
                SchemaKind::Type(t) => match &t {
                    Type::String(_) => lines.push("string".to_owned()),
                    Type::Number(_) => lines.push("number".to_owned()),
                    Type::Integer(_) => lines.push("number".to_owned()),
                    Type::Boolean {} => lines.push("boolean".to_owned()),
                    Type::Object(obj) => {
                        lines.push("{\n".to_owned());
                        for (oname, schema) in &obj.properties {
                            let mut name = oname.to_owned();
                            let required = obj.required.contains(&name);
                            let question = if required { "" } else { "?" };
                            let jd_docs = schema.js_docs();
                            if name.contains(".") {
                                name = vec!["\"", &name, "\""].join("");
                            }
                            lines.push(format!(
                                "{}\n{}{}: {};\n",
                                jd_docs,
                                name,
                                question,
                                schema.to_tds()
                            ));
                        }
                        lines.push("}\n".to_owned());
                    }
                    Type::Array(arr) => {
                        lines.push("Array<".to_owned());
                        if let Some(items) = &arr.items {
                            lines.push(items.to_tds().trim().to_owned());
                        }
                        lines.push(">".to_owned());
                    }
                },
                SchemaKind::AllOf { all_of } => {
                    let mut list = vec![];
                    all_of.iter().for_each(|schema| {
                        list.push(schema.to_tds());
                    });
                    lines.push(list.join(" & "));
                }
                SchemaKind::OneOf { one_of } => {
                    let mut list = vec![];
                    one_of.iter().for_each(|schema| {
                        list.push(schema.to_tds());
                    });
                    lines.push(list.join(" | "));
                }
                SchemaKind::AnyOf { any_of } => {
                    let mut list = vec![];
                    any_of.iter().for_each(|schema| {
                        list.push(schema.to_tds());
                    });
                    lines.push(list.join(" | "));
                }
            }

            lines.join("")
        }
    }

    impl Castable for OpenAPI {
        fn to_tds(&self) -> String {
            // components/schemas
            let mut lines = vec!["declare namespace ApiDefs {".to_owned()];
            self.components.as_ref().map(|components| {
                let schemas = &components.schemas;
                for (name, schema) in schemas {
                    let js_docs = schema.js_docs();
                    let interface_dts = [
                        js_docs,
                        "export interface".to_owned(),
                        safe_name(name),
                        // name.to_owned(),
                        schema.to_tds(),
                    ]
                    .join(" ");

                    lines.push(fix_interface_name_all_of(interface_dts.as_str()))
                }
            });

            // paths
            lines.push("export interface Paths {\n".to_owned());

            self.paths.paths.iter().for_each(|(_req_path, path_item)| {
                lines.push(path_item.to_tds());
            });

            lines.push("}".to_owned());
            // finally
            lines.push("}".to_owned());
            lines.join("\n")
        }
    }

    // "export interface Pet NewPet & Error & { id: number & string"
    // ->
    // "export interface Pet extends NewPet, Error, { id: number & string"
    fn fix_interface_name_all_of(line: &str) -> String {
        let (code, suffix) = line.split_once("{").unwrap();

        let re: Regex = regex::Regex::new(r"(export\s+interface\s+\w+)").unwrap();
        let split = re.split(code).collect::<Vec<&str>>();

        // let body = split.get(1);

        if let Some(body) = split.get(1) {
            if body.trim() == "" {
                return line.to_owned();
            }
            let mut ext = body.split("&").collect::<Vec<&str>>().join(",");
            ext = ext.replace(" ", "") + "{";
            ext = ext.replace(",{", "{");

            let prefix = code.replace(body, " extends ");
            let fxxk = [prefix, ext, suffix.to_owned()].join("");
            fxxk
        } else {
            line.to_owned()
        }
    }
}
