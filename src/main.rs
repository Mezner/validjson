extern crate serde_json;
extern crate valico;
use structopt::StructOpt;
use std::path::PathBuf;
use std::process::exit;

use serde_json::Value;
use valico::json_schema::{Scope};
use valico::json_schema::schema::ScopedSchema;
use std::fs::File;

#[derive(StructOpt)]
#[structopt(name = "json_schema_validator")]
struct Opts {
    /// JSON Schema file
    #[structopt(name = "SCHEMA")]
    schema: PathBuf,

    /// JSON content file[s]
    #[structopt(name = "CONTENT")]
    content: Vec<PathBuf>,

    /// Ignore unknown keywords in schema validation
    #[structopt(short)]
    ignore_unknowns: bool,
}

fn main() {
    let opts = Opts::from_args();
    let mut scope = Scope::new();
    let schema = compile_schema_or_exit(&mut scope, opts.schema, opts.ignore_unknowns);
    validate_content(&schema, &opts.content);
}

fn compile_schema_or_exit<'a>(scope: &'a mut Scope, path: PathBuf, ignore_unknowns: bool) -> ScopedSchema<'a> {
    let result = File::open(path.clone());
    let reader = match result {
        Err(_er) => {
            eprintln!("{:?}: Schema file cannot be loaded", path);
            exit(1);
        },
        Ok(reader) => reader,
    };

    let result = serde_json::from_reader(reader);
    let json: Value = match result {
        Err(er) => {
            eprintln!("{:?}: Error parsing schema JSON: {:?}", path, er);
            exit(2);
        },
        Ok(json) => json,
    };

    let result = scope.compile_and_return(json.clone(), !ignore_unknowns);
    match result {
        Err(er) => {
            eprintln!("{:?}: Error compiling schema: {:?}", path, er);
            exit(3);
        },
        Ok(schema) => schema
    }
}

fn validate_content(schema: &ScopedSchema, content: &Vec<PathBuf>) {
    for c in content {
        let result = File::open(c);
        let reader = match result {
            Err(_er) => {
                eprintln!("{:?}: Could not open file", c);
                continue;
            }
            Ok(reader) => reader,
        };
        let json = serde_json::from_reader(reader);
        let json = match json {
            Err(er) => {
                eprintln!("{:?}: Error reading JSON: {:?}", c, er);
                continue;
            }
            Ok(json) => json,
        };
        let json: Value = json;
        let validation_state = schema.validate(&json);
        if validation_state.is_valid() {
            println!("{:?}: valid against schema", c);
        } else {
            println!("{:?}: Validation errors:", c);
            for er in validation_state.errors {
                println!("------");
                println!("Code: {}", er.get_code());
                println!("Path: {}", er.get_path());
                println!("Title: {}", er.get_title());
                if let Some(detail) = er.get_detail() {
                    println!("Detail: {}", detail);
                }
            }
        }
    }
}
