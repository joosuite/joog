use anyhow::{Context, Result, ensure};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use serde_json::{Map, Value, json};
#[cfg(test)]
use std::collections::BTreeSet;
use std::{collections::BTreeMap, sync::OnceLock};

use crate::{
    discovery::{Document, MethodSpec, Parameter, Resource},
    http, native, pagination,
    services::Invocation,
};

pub struct Contract {
    pub document: Document,
    pub compose_paths: BTreeMap<String, Vec<String>>,
    pub compose_schema: Value,
    pub inputs: BTreeMap<String, Value>,
}
macro_rules! sources {
    ($($name:literal;)*) => {
        static SOURCES:[(&str,&str,&str,OnceLock<Contract>);crate::services::SERVICES.len()]=[$(($name,include_str!(concat!("../api/",$name,".json")),include_str!(concat!("../schemas/",$name,".schema.json")),OnceLock::new())),*];
    }
}
sources! {"mail";"drive";"calendar";"sheets";"docs";"slides";"forms";"tasks";"contacts";"youtube";"photos";}

pub fn contract(service: &str) -> &'static Contract {
    let (_, source, inputs, cache) = SOURCES
        .iter()
        .find(|(name, _, _, _)| *name == service)
        .expect("known service");
    cache.get_or_init(|| {
        let public: Value = serde_json::from_str(inputs).expect("checked CLI input schema");
        let inputs: BTreeMap<String, Value> =
            serde_json::from_value(public["$defs"].clone()).expect("checked CLI commands");
        let mut root: Value = serde_json::from_str(source).expect("checked bundled schema");
        let compose_paths = root["$defs"]
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(name, definition)| {
                definition.get("x-cli-compose-path").map(|path| {
                    (
                        name.clone(),
                        serde_json::from_value(path.clone()).expect("checked compose path"),
                    )
                })
            })
            .collect();
        let mut compose_schema = public["x-input-types"]["MailComposition"].clone();
        if let Some(properties) = compose_schema.get_mut("properties") {
            *properties = Value::Object(
                inputs["messages.send"]["properties"]
                    .as_object()
                    .expect("checked mail inputs")
                    .values()
                    .filter(|shape| shape["x-cli-binding"]["source"] == "compose")
                    .map(|shape| {
                        (
                            shape["x-cli-binding"]["property"]
                                .as_str()
                                .unwrap()
                                .to_owned(),
                            shape.clone(),
                        )
                    })
                    .collect(),
            );
        }
        let catalog: BTreeMap<String, MethodSpec> =
            serde_json::from_value(root["x-google-methods"].take())
                .expect("checked method catalog");
        let methods = root["$defs"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, definition)| {
                let id = definition["x-google-method-id"]
                    .as_str()
                    .expect("checked method ID");
                let mut method = catalog.get(id).expect("checked catalog reference").clone();
                method.cli_inputs = serde_json::from_value(inputs[name]["properties"].clone())
                    .expect("checked CLI inputs");
                method.description = inputs[name]["description"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned();
                for (key, shape) in &method.cli_inputs {
                    if shape["x-cli-binding"]["source"] == "query" {
                        let property = shape["x-cli-binding"]["property"].as_str().unwrap();
                        if let Some(parameter) = method.parameters.get_mut(property) {
                            parameter.description =
                                shape["description"].as_str().unwrap_or_default().to_owned();
                            parameter.required =
                                inputs[name]["allOf"].as_array().into_iter().flatten().any(
                                    |rule| {
                                        rule["x-cli-kind"] == "required"
                                            && rule["x-cli-parameters"].as_array().is_some_and(
                                                |keys| keys.iter().any(|item| item == key),
                                            )
                                    },
                                );
                        }
                    }
                }
                if let Some(operation) = definition["x-batch-operation"].as_str() {
                    method.batch_operation = Some(operation.into());
                    method.operation_request = Some(inputs[name]["properties"]["body"].clone());
                    method.description = definition["properties"]["body"]["description"]
                        .as_str()
                        .expect("checked operation description")
                        .into();
                    method.batch_fields = definition["properties"]
                        .as_object()
                        .unwrap()
                        .iter()
                        .filter(|(key, _)| {
                            !matches!(key.as_str(), "body" | "fields")
                                && !method.parameters.contains_key(*key)
                        })
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect();
                }
                (name.clone(), method)
            })
            .collect();
        let document = Document {
            name: root["x-google-api"].as_str().unwrap().into(),
            version: root["x-google-api-version"].as_str().unwrap().into(),
            title: root["title"].as_str().unwrap_or_default().into(),
            root_url: root["x-google-api-root"].as_str().unwrap().into(),
            service_path: root["x-google-service-path"]
                .as_str()
                .unwrap_or_default()
                .into(),
            base_url: String::new(),
            parameters: serde_json::from_value(root["x-google-parameters"].take()).unwrap(),
            schemas: serde_json::from_value(
                public
                    .get("x-input-types")
                    .cloned()
                    .unwrap_or_else(|| json!({})),
            )
            .unwrap(),
            root: Resource {
                methods,
                resources: BTreeMap::new(),
            },
        };
        let service = crate::services::SERVICES
            .iter()
            .find(|item| item.cli == service)
            .unwrap();
        assert_eq!(document.name, service.api);
        assert_eq!(document.version, service.version);
        Contract {
            document,
            compose_paths,
            compose_schema,
            inputs,
        }
    })
}

pub fn kebab(name: &str) -> String {
    let mut result = String::new();
    let chars = name.chars().collect::<Vec<_>>();
    for (index, &ch) in chars.iter().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0
                && (chars[index - 1].is_ascii_lowercase()
                    || chars[index - 1].is_ascii_digit()
                    || (chars[index - 1].is_ascii_uppercase()
                        && chars.get(index + 1).is_some_and(char::is_ascii_lowercase)))
            {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(if matches!(ch, '_' | '.') { '-' } else { ch });
        }
    }
    result
}

fn type_schema<'a>(document: &'a Document, schema: &'a Value) -> &'a Value {
    let mut schema = schema;
    for _ in 0..64 {
        if let Some(choices) = schema["anyOf"].as_array() {
            schema = choices
                .iter()
                .find(|choice| choice["type"] != "null")
                .unwrap_or(schema);
        } else if let Some(reference) = schema["$ref"].as_str() {
            let name = reference
                .strip_prefix("#/x-body-types/")
                .or_else(|| reference.strip_prefix("#/x-input-types/"))
                .unwrap_or(reference);
            schema = document
                .schemas
                .get(name)
                .expect("checked schema reference");
        } else {
            return schema;
        }
    }
    schema
}
#[derive(Clone)]
enum Source {
    Query(Parameter),
    Body(Value),
    Envelope(Value),
    Nested(Value, Vec<String>),
}
#[derive(Clone)]
struct Binding {
    id: String,
    long: String,
    property: String,
    source: Source,
    repeated: bool,
}

fn required_for(schema: &Value, method: &str) -> bool {
    schema["x-required-in-methods"]
        .as_array()
        .is_some_and(|methods| methods.iter().any(|id| id == method))
        || schema["anyOf"]
            .as_array()
            .is_some_and(|choices| choices.iter().any(|schema| required_for(schema, method)))
}
#[cfg(test)]
fn hidden_parameter(name: &str) -> bool {
    matches!(
        name,
        "fields"
            | "access_token"
            | "oauth_token"
            | "$.xgafv"
            | "alt"
            | "key"
            | "quotaUser"
            | "callback"
            | "prettyPrint"
            | "uploadType"
            | "upload_protocol"
            | "pageSize"
            | "maxResults"
    )
}
fn input_request(method: &MethodSpec) -> Option<&Value> {
    method
        .operation_request
        .as_ref()
        .or(method.request.as_ref())
}

fn required_input_field(schema: &Value, key: &str, property: &Value, method: &str) -> bool {
    required_for(property, method)
        || ["required", "x-required-fields"].iter().any(|name| {
            schema[name]
                .as_array()
                .is_some_and(|fields| fields.iter().any(|field| field == key))
        })
}

fn bindings(_document: &Document, method: &MethodSpec) -> Vec<Binding> {
    let schema = &method.cli_inputs;
    schema
        .iter()
        .filter_map(|(key, shape)| {
            let mapping = &shape["x-cli-binding"];
            let kind = mapping["source"].as_str()?;
            let property = mapping["property"].as_str().unwrap_or(key).to_owned();
            let source = match kind {
                "query" => Source::Query(method.parameters.get(&property)?.clone()),
                "body" => Source::Body(shape.clone()),
                "envelope" => Source::Envelope(shape.clone()),
                "nested" => Source::Nested(
                    shape.clone(),
                    serde_json::from_value(mapping["path"].clone()).expect("checked input path"),
                ),
                _ => return None,
            };
            let prefix = match kind {
                "query" => "q",
                "body" => "b",
                "envelope" => "e",
                _ => "n",
            };
            Some(Binding {
                id: format!("{prefix}:{property}"),
                long: shape["x-cli-option"]
                    .as_str()
                    .expect("checked option")
                    .into(),
                property,
                source,
                repeated: shape["x-cli-repeated"] == true,
            })
        })
        .collect()
}

#[cfg(test)]
fn rest_bindings(document: &Document, method: &MethodSpec) -> Vec<Binding> {
    let reserved = [
        "file",
        "params",
        "body",
        "fields",
        "max",
        "max-pages",
        "ttl",
        "upload",
        "content-type",
        "output",
        "home",
        "timeout",
        "json",
        "help",
    ];
    let mut used = reserved
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let mut result = Vec::new();
    for (name, parameter) in &method.parameters {
        if hidden_parameter(name) || name == "userId" && document.name == "gmail" {
            continue;
        }
        let mut long = kebab(name);
        if used.contains(&long) {
            long = format!("query-{long}");
        }
        assert!(used.insert(long.clone()), "duplicate query option");
        result.push(Binding {
            id: format!("q:{name}"),
            long,
            property: name.clone(),
            source: Source::Query(parameter.clone()),
            repeated: parameter.repeated,
        });
    }
    if let Some(request) = input_request(method) {
        let body = type_schema(document, request);
        for (name, schema) in body["properties"].as_object().into_iter().flatten() {
            let shape = type_schema(document, schema);
            if shape["readOnly"] == true {
                continue;
            }
            let mut long = kebab(name);
            if used.contains(&long) {
                long = format!("body-{long}");
            }
            assert!(used.insert(long.clone()), "duplicate body option");
            let items = type_schema(document, &shape["items"]);
            let repeated = shape["type"] == "array"
                && matches!(
                    items["type"].as_str(),
                    Some("string" | "integer" | "number" | "boolean")
                );
            result.push(Binding {
                id: format!("b:{name}"),
                long,
                property: name.clone(),
                source: Source::Body(schema.clone()),
                repeated,
            });
        }
    }
    for (name, schema) in &method.batch_fields {
        let mut long = kebab(name);
        if used.contains(&long) {
            long = format!("batch-{long}");
        }
        assert!(used.insert(long.clone()), "duplicate batch option");
        result.push(Binding {
            id: format!("e:{name}"),
            long,
            property: name.clone(),
            source: Source::Envelope(schema.clone()),
            repeated: false,
        });
    }
    result
}
fn option(name: &str, label: &str, help: &str) -> Arg {
    Arg::new(name.to_owned())
        .long(name.to_owned())
        .value_name(label.to_owned())
        .help(help.to_owned())
}
fn default_parameter<'a>(
    _document: &Document,
    method: &'a MethodSpec,
    name: &str,
) -> Option<&'a str> {
    method
        .cli_inputs
        .values()
        .find(|shape| {
            shape["x-cli-binding"]["source"] == "query"
                && shape["x-cli-binding"]["property"] == name
        })
        .and_then(|shape| shape["default"].as_str())
}
fn shape_kind<'a>(document: &'a Document, binding: &'a Binding) -> (&'a str, bool) {
    match &binding.source {
        Source::Query(parameter) => (parameter.kind.as_str(), false),
        Source::Body(schema) | Source::Envelope(schema) | Source::Nested(schema, _) => {
            let shape = type_schema(document, schema);
            let item = if binding.repeated {
                type_schema(document, &shape["items"])
            } else {
                shape
            };
            (item["type"].as_str().unwrap_or("object"), true)
        }
    }
}
fn label(document: &Document, binding: &Binding) -> String {
    let (kind, body) = shape_kind(document, binding);
    if (kind == "object" || kind == "array") && body {
        "JSON".into()
    } else if kind == "boolean" {
        "BOOL".into()
    } else {
        binding.long.to_ascii_uppercase().replace('-', "_")
    }
}
pub fn direct_download(method: &MethodSpec) -> bool {
    method.supports_media_download
        && (method.response.is_none() || method.id.ends_with(".media.download"))
}

pub fn upload_mime(method: &MethodSpec) -> &str {
    let accept = method
        .media_upload
        .as_ref()
        .and_then(|upload| upload["accept"].as_array());
    if accept.is_some_and(|values| values.len() == 1 && values[0] == "message/*") {
        "message/rfc822"
    } else {
        "application/octet-stream"
    }
}
pub fn validate_upload(method: &MethodSpec, path: &str, mime: &str) -> Result<()> {
    reqwest::header::HeaderValue::from_str(mime).context("invalid upload MIME type")?;
    let upload = method
        .media_upload
        .as_ref()
        .context("this method does not support uploads")?;
    let input = method.cli_inputs.get("upload");
    let limit = input
        .and_then(|shape| shape["x-file-max-bytes"].as_u64())
        .or_else(|| {
            upload["maxSize"]
                .as_str()
                .and_then(|value| value.parse::<u64>().ok())
        })
        .unwrap_or(u64::MAX)
        .min(1024 * 1024 * 1024);
    let metadata = std::fs::metadata(path).context("cannot read upload file")?;
    ensure!(
        metadata.is_file() && metadata.len() <= limit,
        "upload must be a regular file no larger than {limit} bytes"
    );
    if let Some(accepted) = input
        .and_then(|shape| shape["x-file-mime-types"].as_array())
        .or_else(|| upload["accept"].as_array())
    {
        ensure!(
            accepted
                .iter()
                .filter_map(Value::as_str)
                .any(|kind| kind == "*/*"
                    || kind == mime
                    || kind
                        .strip_suffix('*')
                        .is_some_and(|prefix| mime.starts_with(prefix))),
            "this method does not accept the upload MIME type"
        );
    }
    Ok(())
}
pub fn validate_file_parameters(
    method: &MethodSpec,
    parameters: &Map<String, Value>,
) -> Result<()> {
    for name in parameters.keys() {
        ensure!(
            matches!(name.as_str(), "body" | "fields")
                || method.parameters.contains_key(name)
                || method.batch_fields.contains_key(name),
            "unknown request-file parameter {name:?}"
        );
    }
    Ok(())
}

pub fn command(service: &str, name: &str) -> Command {
    let contract = contract(service);
    let document = &contract.document;
    let method = &document.root.methods[name];
    let about = method
        .description
        .split('\n')
        .next()
        .unwrap_or(name)
        .to_owned();
    let about = about.split(". ").next().unwrap_or(&about).to_owned();
    let read = method.http_method == "GET" || method.http_method == "HEAD" || method.read_only;
    let mut command = Command::new(name.to_owned())
        .about(about)
        .long_about(method.description.clone())
        .override_usage(format!(
            "joog [OPTIONS] {service} <ACCOUNT> {name} [COMMAND_OPTIONS]"
        ))
        .arg(option(
            "file",
            "PATH",
            "Read command parameters from a JSON file",
        ))
        .arg(option("fields", "MASK", "Select response fields"));
    if read && !method.fresh_response && !direct_download(method) {
        command = command.arg(
            option(
                "ttl",
                "MINUTES",
                "Reuse encrypted response data; 0 disables caching",
            )
            .default_value("0")
            .value_parser(clap::value_parser!(u32)),
        );
    }
    if pagination::Plan::new(method, read, Some(1), None).is_ok() {
        command = command.arg(
            option(
                "max",
                "N",
                "Return at most N results; by default, return all results",
            )
            .value_parser(clap::value_parser!(u32).range(1..)),
        );
    }
    if pagination::Plan::new(method, read, None, Some(1)).is_ok() {
        command = command.arg(
            option(
                "max-pages",
                "N",
                "Stop after N pages; by default, fetch all pages",
            )
            .value_parser(clap::value_parser!(u32).range(1..=10000)),
        );
    }
    if method.supports_media_download || method.id == "gmail.users.messages.attachments.get" {
        let mut output = option(
            "output",
            "PATH",
            "Save the downloaded content to a new file",
        )
        .short('o');
        if contract.inputs[name]["allOf"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|rule| {
                rule["x-cli-kind"] == "required"
                    && rule["x-cli-parameters"]
                        .as_array()
                        .is_some_and(|keys| keys.iter().any(|key| key == "output"))
            })
        {
            output = output.required_unless_present("file");
        }
        command = command.arg(output);
    }
    if method.media_upload.is_some() {
        command = command
            .arg(option(
                "upload",
                "PATH",
                contract.inputs[name]["properties"]["upload"]["description"]
                    .as_str()
                    .unwrap_or("Upload a local file"),
            ))
            .arg(
                option("content-type", "MIME", "Upload MIME type")
                    .default_value(upload_mime(method).to_owned()),
            );
    }
    let mut body_input = vec!["body".to_owned(), "file".to_owned()];
    if contract.compose_paths.contains_key(name) {
        for arg in crate::mail::options(&contract.compose_schema) {
            body_input.push(arg.get_id().to_string());
            command = command.arg(arg);
        }
    }
    if method.request.is_some() {
        command = command.arg(option("body", "JSON", "Inline JSON request body"));
    } else {
        body_input.retain(|name| name != "body");
    }
    for binding in bindings(document, method) {
        let (kind, body) = shape_kind(document, &binding);
        let mut help = match &binding.source {
            Source::Query(p) => p.description.as_str(),
            Source::Body(schema) | Source::Envelope(schema) | Source::Nested(schema, _) => {
                type_schema(document, schema)["description"]
                    .as_str()
                    .unwrap_or("Request body field")
            }
        }
        .to_owned();
        if let Source::Query(parameter) = &binding.source {
            if !parameter.choices.is_empty() {
                help.push_str(&format!(" Choices: {}.", parameter.choices.join(", ")));
            }
        } else if let Source::Body(schema) | Source::Envelope(schema) | Source::Nested(schema, _) =
            &binding.source
        {
            let shape = type_schema(document, schema);
            if let Some(choices) = shape["enum"].as_array() {
                help.push_str(&format!(
                    " Choices: {}.",
                    choices
                        .iter()
                        .map(|value| value.as_str().unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        let mut arg = Arg::new(binding.id.clone())
            .long(binding.long.clone())
            .value_name(label(document, &binding))
            .help(help.to_owned());
        if binding.repeated {
            arg = arg.action(ArgAction::Append).num_args(1..);
        } else if kind == "boolean" {
            arg = arg
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true");
        }
        if let Source::Query(p) = &binding.source {
            if let Some(value) = default_parameter(document, method, &binding.property) {
                arg = arg.default_value(value.to_owned());
            } else if p.required {
                arg = arg.required_unless_present("file");
            }
            let p = p.clone();
            let property = binding.property.clone();
            arg = arg.value_parser(move |value: &str| {
                let value = Value::String(value.into());
                crate::discovery::validate_parameter(&property, &p, &value)
                    .map(|_| value.as_str().unwrap().to_owned())
                    .map_err(|error| error.to_string())
            });
        } else if body {
            body_input.push(binding.id.clone());
        }
        command = command.arg(arg);
    }
    if let Some(request) = input_request(method) {
        let schema = type_schema(document, request);
        let empty = schema["properties"].as_object().is_some_and(Map::is_empty)
            || schema["type"] == "object"
                && schema.get("properties").is_none()
                && schema.get("additionalProperties").is_none();
        if !empty {
            if method.media_upload.is_some() {
                body_input.push("upload".into());
            }
            command = command.group(
                ArgGroup::new("request-content")
                    .args(body_input)
                    .required(
                        contract.inputs[name]["allOf"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .any(|rule| rule["x-cli-kind"] == "content"),
                    )
                    .multiple(true),
            );
        }
    }
    crate::help::register_conflicts(command.after_help(format!("Reference: docs/services/{service}.md\nParameter schema: schemas/{service}.schema.json#/$defs/{name}")), &contract.inputs[name])
}
pub fn commands(service: &str) -> Vec<Command> {
    contract(service)
        .document
        .root
        .methods
        .keys()
        .map(|name| command(service, name))
        .collect()
}

fn parse_scalar(kind: &str, text: &str) -> Result<Value> {
    match kind {
        "integer" | "number" | "boolean" => {
            serde_json::from_str(text).context("invalid numeric or boolean value")
        }
        "object" | "array" => {
            ensure!(
                text.len() as u64 <= http::DISCOVERY_LIMIT,
                "JSON input exceeds 16 MiB"
            );
            serde_json::from_str(text).context("invalid JSON field value")
        }
        _ => Ok(json!(text)),
    }
}
fn value_for(document: &Document, binding: &Binding, values: Vec<&str>) -> Result<Value> {
    let (kind, _) = shape_kind(document, binding);
    if binding.repeated {
        Ok(Value::Array(
            values
                .into_iter()
                .map(|value| parse_scalar(kind, value))
                .collect::<Result<_>>()?,
        ))
    } else {
        parse_scalar(kind, values[0])
    }
}

pub fn validate_body(
    document: &Document,
    method: &str,
    schema: &Value,
    value: &Value,
    path: &str,
) -> Result<()> {
    fn check(
        document: &Document,
        method: &str,
        schema: &Value,
        value: &Value,
        path: &str,
        depth: usize,
    ) -> Result<()> {
        ensure!(depth <= 128, "request body exceeds nesting limit");
        if let Some(reference) = schema["$ref"].as_str() {
            let name = reference
                .strip_prefix("#/x-body-types/")
                .or_else(|| reference.strip_prefix("#/x-input-types/"))
                .unwrap_or(reference);
            return check(
                document,
                method,
                document.schemas.get(name).context("unknown body type")?,
                value,
                path,
                depth + 1,
            );
        }
        if let Some(choices) = schema["anyOf"].as_array() {
            ensure!(
                choices.iter().any(|schema| check(
                    document,
                    method,
                    schema,
                    value,
                    path,
                    depth + 1
                )
                .is_ok()),
                "invalid value for {path}"
            );
            return Ok(());
        }
        match schema["type"].as_str().unwrap_or_default() {
            "null" => ensure!(value.is_null(), "{path} must be null"),
            "string" => ensure!(value.is_string(), "{path} must be a string"),
            "integer" => ensure!(
                value.as_i64().is_some() || value.as_u64().is_some(),
                "{path} must be an integer"
            ),
            "number" => ensure!(value.is_number(), "{path} must be a number"),
            "boolean" => ensure!(value.is_boolean(), "{path} must be a boolean"),
            "array" => {
                let items = value
                    .as_array()
                    .with_context(|| format!("{path} must be an array"))?;
                for (index, item) in items.iter().enumerate() {
                    check(
                        document,
                        method,
                        &schema["items"],
                        item,
                        &format!("{path}[{index}]"),
                        depth + 1,
                    )?;
                }
            }
            "object" => {
                let object = value
                    .as_object()
                    .with_context(|| format!("{path} must be an object"))?;
                if let Some(fields) = schema["x-required-one-of"]
                    .get(method)
                    .and_then(Value::as_array)
                {
                    ensure!(
                        fields.iter().any(|key| object
                            .get(key.as_str().unwrap())
                            .is_some_and(|value| !value.is_null())),
                        "{path}: provide one of {}",
                        fields
                            .iter()
                            .map(|key| key.as_str().unwrap())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                for group in schema["x-exclusive-fields"]
                    .as_array()
                    .into_iter()
                    .flatten()
                {
                    let active = group
                        .as_array()
                        .expect("checked exclusive fields")
                        .iter()
                        .filter_map(|key| {
                            let key = key.as_str().unwrap();
                            object
                                .get(key)
                                .is_some_and(|value| !value.is_null())
                                .then_some(key)
                        })
                        .collect::<Vec<_>>();
                    ensure!(
                        active.len() <= 1,
                        "{path}: conflicting fields {}",
                        active.join(", ")
                    );
                }
                if !method.is_empty()
                    && let Some(required) = schema
                        .get("required")
                        .or_else(|| schema.get("x-required-fields"))
                        .and_then(Value::as_array)
                {
                    for key in required {
                        let key = key.as_str().context("invalid required property")?;
                        ensure!(object.contains_key(key), "{path}.{key} is required");
                    }
                }
                for (key, property) in schema["properties"].as_object().into_iter().flatten() {
                    if !method.is_empty() && required_input_field(schema, key, property, method) {
                        ensure!(
                            object.get(key).is_some_and(|value| !value.is_null()),
                            "{path}.{key} is required"
                        );
                    }
                }
                for (key, value) in object {
                    let property = schema["properties"]
                        .get(key)
                        .or_else(|| {
                            schema
                                .get("additionalProperties")
                                .filter(|value| value.is_object())
                        })
                        .with_context(|| format!("unknown body field {path}[{key:?}]"))?;
                    check(
                        document,
                        method,
                        property,
                        value,
                        &format!("{path}[{key:?}]"),
                        depth + 1,
                    )?;
                }
            }
            _ => {}
        }
        if let Some(choices) = schema["enum"].as_array() {
            ensure!(choices.contains(value), "invalid choice for {path}");
        }
        Ok(())
    }
    check(document, method, schema, value, path, 0)
}

pub fn resolve(
    service: &str,
    account: &str,
    name: &str,
    args: &ArgMatches,
    mut params: Map<String, Value>,
) -> Result<Invocation> {
    let contract = contract(service);
    let document = &contract.document;
    let method = &document.root.methods[name];
    let compose = if contract.compose_paths.contains_key(name) {
        params.remove("compose")
    } else {
        None
    };
    validate_file_parameters(method, &params)?;
    let mut envelope = Map::new();
    for key in method.batch_fields.keys() {
        if let Some(value) = params.remove(key) {
            envelope.insert(key.clone(), value);
        }
    }
    if native::text(args, "content-type").is_some()
        && args.value_source("content-type") == Some(clap::parser::ValueSource::CommandLine)
    {
        ensure!(
            native::text(args, "upload").is_some(),
            "--content-type requires --upload"
        );
    }
    let custom_fields = params.contains_key("fields")
        || native::text(args, "fields").is_some()
        || ["personFields", "readMask", "part"].iter().any(|name| {
            params.contains_key(*name)
                || native::text(args, &format!("q:{name}")).is_some()
                    && args
                        .value_source(&format!("q:{name}"))
                        .is_some_and(|source| source == clap::parser::ValueSource::CommandLine)
        })
        || params.get("format").is_some_and(|format| format != "full")
        || native::text(args, "q:format").is_some_and(|format| format != "full");
    let file_part = params.contains_key("part");
    let mut body = params.remove("body");
    if let Some(raw) = native::text(args, "body") {
        ensure!(
            body.is_none(),
            "choose either --body or the body's entry in --file"
        );
        ensure!(
            raw.len() as u64 <= http::DISCOVERY_LIMIT,
            "request body exceeds 16 MiB"
        );
        body = Some(crate::read_json(raw)?);
    }
    for binding in bindings(document, method) {
        let Some(values) = args.try_get_many::<String>(&binding.id).ok().flatten() else {
            continue;
        };
        let value = value_for(document, &binding, values.map(String::as_str).collect())?;
        match binding.source {
            Source::Query(_) => {
                if args.value_source(&binding.id) == Some(clap::parser::ValueSource::CommandLine)
                    || !params.contains_key(&binding.property)
                {
                    params.insert(binding.property, value);
                }
            }
            Source::Body(_) => {
                body.get_or_insert_with(|| json!({}))
                    .as_object_mut()
                    .context("named body fields require an object body")?
                    .insert(binding.property, value);
            }
            Source::Envelope(_) => {
                envelope.insert(binding.property, value);
            }
            Source::Nested(_, path) => {
                crate::file_inputs::set_path(body.get_or_insert_with(|| json!({})), &path, value)?;
            }
        }
    }
    if document.name == "gmail" {
        params.entry("userId").or_insert(json!("me"));
    }
    for name in method.parameters.keys() {
        if let Some(value) = default_parameter(document, method, name) {
            params.entry(name).or_insert(json!(value));
        }
    }
    if let Some(fields) = native::text(args, "fields") {
        params.insert("fields".into(), json!(fields));
    }
    if document.name == "youtube"
        && method.request.is_some()
        && (method
            .parameters
            .get("part")
            .is_some_and(|part| part.required)
            || default_parameter(document, method, "part").is_some())
        && args.value_source("q:part") != Some(clap::parser::ValueSource::CommandLine)
        && !file_part
        && let Some(object) = body.as_ref().and_then(Value::as_object)
    {
        let parts = object
            .keys()
            .filter(|key| !matches!(key.as_str(), "id" | "kind" | "etag"))
            .cloned()
            .collect::<Vec<_>>();
        if !parts.is_empty() {
            params.insert("part".into(), json!(parts.join(",")));
        }
    }
    crate::file_inputs::validate_constraints(
        service,
        name,
        args,
        crate::file_inputs::RequestValues {
            params: &params,
            body: body.as_ref(),
            envelope: &envelope,
            compose: compose.as_ref(),
        },
    )?;
    if let Some(path) = contract.compose_paths.get(name) {
        crate::mail::compose(
            account,
            name,
            args,
            compose,
            &contract.compose_schema,
            path,
            &mut body,
        )?;
    }
    if let Some(request) = input_request(method) {
        if body.is_none() && native::text(args, "upload").is_none() {
            body = Some(json!({}));
        }
        if let Some(body) = &body {
            validate_body(document, &method.id, request, body, "body")?;
            ensure!(
                serde_json::to_vec(body)?.len() as u64 <= http::DISCOVERY_LIMIT,
                "request body exceeds 16 MiB"
            );
        }
    } else {
        ensure!(body.is_none(), "this command does not accept a body");
    }
    if let Some(operation) = &method.batch_operation {
        for (key, value) in &envelope {
            validate_body(document, &method.id, &method.batch_fields[key], value, key)?;
        }
        envelope.insert(
            "requests".into(),
            json!([{operation:body.unwrap_or_else(||json!({}))}]),
        );
        let encoded = Value::Object(envelope);
        validate_body(
            document,
            &method.id,
            method
                .request
                .as_ref()
                .context("missing batch request schema")?,
            &encoded,
            "body",
        )?;
        ensure!(
            serde_json::to_vec(&encoded)?.len() as u64 <= http::DISCOVERY_LIMIT,
            "request body exceeds 16 MiB"
        );
        body = Some(encoded);
    }
    if document.name == "calendar"
        && name == "events.list"
        && params
            .get("singleEvents")
            .is_some_and(|value| value == true || value == "true")
    {
        ensure!(
            params.contains_key("timeMax")
                || args.try_get_one::<u32>("max").ok().flatten().is_some()
                || args
                    .try_get_one::<u32>("max-pages")
                    .ok()
                    .flatten()
                    .is_some(),
            "expanded recurring events require --time-max, --max, or --max-pages"
        );
    }
    document.url(method, &params)?;
    let custom_fields = custom_fields
        || params
            .get("includeGridData")
            .is_some_and(|value| value == true || value == "true");
    Ok(Invocation {
        method: name.into(),
        params,
        body,
        custom_fields,
        relative_times: Vec::new(),
        options: None,
    })
}

#[cfg(test)]
fn reference(service: &str) -> String {
    use std::fmt::Write;
    let contract = contract(service);
    let mut cli = crate::cli::verbose_command();
    cli.build();
    let namespace = cli.find_subcommand(service).unwrap();
    let mut text = format!(
        "# {} methods\n\nSee the [command guide](../grammar.md) and [request-file format](../file-schemas.md).\n\n",
        crate::services::SERVICES
            .iter()
            .find(|item| item.cli == service)
            .unwrap()
            .title
    );
    text.push_str("| Method | Description |\n| --- | --- |\n");
    for name in contract.document.root.methods.keys() {
        let about = namespace
            .find_subcommand(name)
            .unwrap()
            .get_about()
            .unwrap()
            .to_string()
            .replace('|', "\\|")
            .replace('\n', " ");
        writeln!(&mut text, "| `{name}` | {about} |").unwrap();
    }
    text.push('\n');
    for name in contract.document.root.methods.keys() {
        let mut command = namespace.find_subcommand(name).unwrap().clone();
        let help = crate::help::order_sections(&command.render_long_help().to_string());
        let help = help.split("\nReference: ").next().unwrap().trim_end();
        writeln!(&mut text, "## {name}\n\n```text\n{help}\n```\n").unwrap();
    }
    text.push_str("Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).\n");
    text
}

#[cfg(test)]
#[path = "schema_commands/restricted_tests.rs"]
mod restricted_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cli, services::SERVICES};

    #[test]
    fn cli_schemas_cover_every_rest_input_and_registered_option() {
        for service in SERVICES {
            let contract = contract(service.cli);
            for (name, method) in &contract.document.root.methods {
                let actual = bindings(&contract.document, method);
                for expected in rest_bindings(&contract.document, method) {
                    assert!(
                        actual
                            .iter()
                            .any(|binding| binding.id == expected.id
                                && binding.long == expected.long),
                        "{} {name}: missing {}",
                        service.cli,
                        expected.long
                    );
                }
                let command = command(service.cli, name);
                for shape in contract.inputs[name]["properties"]
                    .as_object()
                    .unwrap()
                    .values()
                {
                    if let Some(long) = shape["x-cli-option"].as_str() {
                        assert!(
                            command
                                .get_arguments()
                                .any(|arg| arg.get_long() == Some(long)),
                            "{} {name}: schema has unsupported --{long}",
                            service.cli
                        );
                    }
                }
                for arg in command
                    .get_arguments()
                    .filter(|arg| arg.get_long().is_some_and(|long| long != "file"))
                {
                    let long = arg.get_long().unwrap();
                    assert!(
                        contract.inputs[name]["properties"]
                            .as_object()
                            .unwrap()
                            .values()
                            .any(|shape| shape["x-cli-option"] == long),
                        "{} {name}: --{long} is absent from its CLI schema",
                        service.cli
                    );
                }
            }
            for command in native::commands(service.cli) {
                let name = command.get_name();
                for arg in command
                    .get_arguments()
                    .filter(|arg| arg.get_long() != Some("file"))
                {
                    assert!(
                        contract.inputs[name]["properties"]
                            .as_object()
                            .unwrap()
                            .values()
                            .any(
                                |shape| shape["x-cli-binding"]["property"] == arg.get_id().as_str()
                            ),
                        "{} {name}: native input {} has no schema",
                        service.cli,
                        arg.get_id()
                    );
                }
            }
        }
    }

    fn scalar(parameter: &Parameter) -> Value {
        if let Some(value) = parameter.choices.first() {
            return json!(value);
        }
        match parameter.kind.as_str() {
            "boolean" => json!(true),
            "integer" => json!(
                parameter
                    .minimum
                    .as_deref()
                    .and_then(|value| value.parse::<i64>().ok())
                    .unwrap_or(1)
                    .max(1)
            ),
            "number" => json!(1),
            _ => json!("example-id"),
        }
    }
    fn sample(document: &Document, schema: &Value, method: &str, depth: usize) -> Value {
        if depth > 32 {
            return Value::Null;
        }
        let shape = type_schema(document, schema);
        if let Some(value) = shape["enum"].as_array().and_then(|values| values.first()) {
            return value.clone();
        }
        match shape["type"].as_str().unwrap_or("object") {
            "string" => json!("Example"),
            "integer" | "number" => json!(1),
            "boolean" => json!(true),
            "array" => json!([]),
            _ => {
                let mut fields = Map::new();
                for (key, value) in shape["properties"].as_object().into_iter().flatten() {
                    if required_for(value, method)
                        || shape["x-required-one-of"][method]
                            .as_array()
                            .and_then(|fields| fields.first())
                            .is_some_and(|field| field == key)
                        || shape
                            .get("required")
                            .or_else(|| shape.get("x-required-fields"))
                            .and_then(Value::as_array)
                            .is_some_and(|required| required.iter().any(|name| name == key))
                    {
                        fields.insert(key.clone(), sample(document, value, method, depth + 1));
                    }
                }
                Value::Object(fields)
            }
        }
    }
    pub(super) fn prepare(
        service: &str,
        arguments: &[&str],
        file: Option<&Value>,
    ) -> Result<Invocation> {
        let file_path = tempfile::NamedTempFile::new()?;
        if let Some(file) = file {
            std::fs::write(file_path.path(), serde_json::to_vec(file)?)?;
        }
        let mut args = vec!["joog", service, "reader@example.com"];
        args.extend(arguments);
        if file.is_some() {
            args.extend(["--file", file_path.path().to_str().unwrap()]);
        }
        let parsed = cli::parse_from(args)?;
        let context = parsed.subcommand().unwrap().1;
        let service = SERVICES.iter().find(|item| item.cli == service).unwrap();
        crate::service_call(service, context)
    }

    fn mime(call: &Invocation) -> String {
        use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
        let body = call.body.as_ref().unwrap();
        let raw = body.get("message").unwrap_or(body)["raw"].as_str().unwrap();
        String::from_utf8(URL_SAFE_NO_PAD.decode(raw).unwrap()).unwrap()
    }

    #[test]
    fn youtube_playlist_item_files_preserve_the_video_and_playlist_ids() {
        let body = json!({"snippet":{"playlistId":"playlist-fixture","resourceId":{"kind":"youtube#video","videoId":"video-fixture"}}});
        let flat = prepare(
            "youtube",
            &["playlist-items.insert"],
            Some(&json!({"part":"snippet","snippet":body["snippet"]})),
        )
        .unwrap();
        let nested = prepare(
            "youtube",
            &["playlist-items.insert"],
            Some(&json!({"part":"snippet","body":body})),
        )
        .unwrap();
        assert_eq!(flat.body, Some(body));
        assert_eq!(flat.body, nested.body);
        assert_eq!(flat.params, nested.params);
    }

    #[test]
    fn optional_youtube_parts_are_not_invented_from_resource_fields() {
        let body = json!({"id":"image-fixture","snippet":{"playlistId":"playlist-fixture","type":"hero","width":256,"height":256}});
        let call = prepare(
            "youtube",
            &["playlist-images.update", "--upload", "fixture.png"],
            Some(&body),
        )
        .unwrap();
        assert!(!call.params.contains_key("part"));
        assert_eq!(call.body, Some(body.clone()));
        let explicit = prepare(
            "youtube",
            &[
                "playlist-images.update",
                "--part",
                "snippet",
                "--upload",
                "fixture.png",
            ],
            Some(&body),
        )
        .unwrap();
        assert_eq!(explicit.params["part"], json!(["snippet"]));
    }

    #[test]
    fn youtube_channel_and_image_updates_preserve_complete_resource_bodies() {
        for (command, part, body) in [
            (
                "channels.update",
                "brandingSettings",
                json!({"id":"channel-fixture","brandingSettings":{"channel":{"title":"Fixture","defaultLanguage":"en","description":"Keep this description","keywords":"one two"},"image":{"bannerExternalUrl":"https://yt3.googleusercontent.com/fixture"}}}),
            ),
            (
                "channels.update",
                "localizations",
                json!({"id":"channel-fixture","localizations":{"fr":{"title":"Titre","description":"Texte"}}}),
            ),
            (
                "playlist-images.update",
                "snippet",
                json!({"id":"image-fixture","snippet":{"playlistId":"playlist-fixture","type":"hero","width":256,"height":256}}),
            ),
        ] {
            let mut flat = body.clone();
            flat["part"] = json!(part);
            if command == "playlist-images.update" {
                flat["upload"] = json!("fixture.png");
            }
            let call = prepare("youtube", &[command], Some(&flat)).unwrap();
            assert_eq!(call.body, Some(body.clone()), "{command} lost body fields");
            let raw = prepare(
                "youtube",
                &[command],
                Some(&if command == "playlist-images.update" {
                    json!({"part":part,"body":body,"upload":"fixture.png"})
                } else {
                    json!({"part":part,"body":body})
                }),
            )
            .unwrap();
            assert_eq!(raw.body, call.body, "{command} file forms differ");
        }
    }

    #[test]
    fn youtube_id_filters_reject_provider_pagination_but_allow_client_limits() {
        for command in ["comments.list", "videos.list"] {
            for field in ["maxResults", "pageToken", "maxPages"] {
                let mut input = json!({"id":["fixture"],"part":"snippet"});
                input[field] = if field == "pageToken" {
                    json!("token")
                } else {
                    json!(1)
                };
                assert!(
                    prepare("youtube", &[command], Some(&input)).is_err(),
                    "accepted {command} {field}"
                );
                assert!(
                    crate::file_inputs::validate_rules(
                        &contract("youtube").inputs[command],
                        &input
                    )
                    .is_err()
                );
            }
            assert!(
                prepare(
                    "youtube",
                    &[command],
                    Some(&json!({"id":["fixture"],"part":"snippet","max":1}))
                )
                .is_ok()
            );
        }
    }

    #[test]
    fn flat_cli_parameter_files_match_options_for_every_service() {
        let cases: Value =
            serde_json::from_str(include_str!("../tests/data/cli-inputs.json")).unwrap();
        assert_eq!(cases.as_array().unwrap().len(), SERVICES.len());
        for case in cases.as_array().unwrap() {
            let service = case["service"].as_str().unwrap();
            let name = case["command"].as_str().unwrap();
            let mut arguments = vec![name];
            arguments.extend(
                case["cli"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap()),
            );
            let from_cli = prepare(service, &arguments, None)
                .unwrap_or_else(|error| panic!("{service} {name} CLI: {error:#}"));
            let from_file = prepare(service, &[name], Some(&case["file"]))
                .unwrap_or_else(|error| panic!("{service} {name} file: {error:#}"));
            assert_eq!(
                from_file.params, from_cli.params,
                "{service} {name} parameters"
            );
            if service == "mail" {
                for call in [&from_cli, &from_file] {
                    let raw = mime(call);
                    for header in [
                        "To: <alice@example.com>",
                        "Cc: <bob@example.com>",
                        "Bcc: <archive@example.com>",
                        "Subject: Report",
                        "Hello",
                    ] {
                        assert!(raw.contains(header), "missing {header}");
                    }
                }
            } else {
                assert_eq!(
                    from_file.body.as_ref(),
                    Some(&case["body"]),
                    "{service} {name} Google body"
                );
                assert_eq!(from_file.body, from_cli.body, "{service} {name} body");
            }
        }
    }

    #[test]
    fn file_controls_and_native_positionals_share_cli_validation_and_overrides() {
        let call = prepare(
            "drive",
            &["files.list", "--max", "2"],
            Some(&json!({"max":5,"maxPages":3,"ttl":10,"pageSize":7})),
        )
        .unwrap();
        let options = call.options.unwrap();
        assert_eq!(options.get_one::<u32>("max"), Some(&2));
        assert_eq!(options.get_one::<u32>("max-pages"), Some(&3));
        assert_eq!(options.get_one::<u32>("ttl"), Some(&10));
        assert_eq!(call.params["pageSize"], 7);
        for invalid in [
            json!({"max":0}),
            json!({"maxPages":10001}),
            json!({"ttl":-1}),
            json!({"max":false}),
        ] {
            assert!(
                prepare("drive", &["files.list"], Some(&invalid)).is_err(),
                "accepted {invalid}"
            );
        }
        let native = prepare("mail", &["read"], Some(&json!({"id":"message"}))).unwrap();
        assert_eq!(native.params["id"], "message");
        assert_eq!(native.body.unwrap(), json!({"removeLabelIds":["UNREAD"]}));
        assert!(prepare("mail", &["read"], Some(&json!({}))).is_err());
        let search = prepare(
            "drive",
            &["search", "explicit"],
            Some(&json!({"query":"file query","parent":"folder","max":4})),
        )
        .unwrap();
        let q = search.params["q"].as_str().unwrap();
        assert!(q.contains("explicit") && !q.contains("file query") && q.contains("folder"));
        assert_eq!(search.options.unwrap().get_one::<u32>("max"), Some(&4));
        let upload = prepare(
            "drive",
            &["files.create"],
            Some(&json!({"name":"Report","upload":"local.pdf","contentType":"application/pdf"})),
        )
        .unwrap();
        assert_eq!(
            native::text(upload.options.as_ref().unwrap(), "upload"),
            Some("local.pdf")
        );
        assert_eq!(
            native::text(upload.options.as_ref().unwrap(), "content-type"),
            Some("application/pdf")
        );
        let download = prepare(
            "drive",
            &["files.export"],
            Some(&json!({"fileId":"doc","mimeType":"application/pdf","output":"report.pdf"})),
        )
        .unwrap();
        assert_eq!(
            native::text(download.options.as_ref().unwrap(), "output"),
            Some("report.pdf")
        );
    }

    #[test]
    fn nested_file_values_are_partial_and_explicit_options_override_them() {
        let repaired = prepare(
            "drive",
            &["files.create", "--name", "Valid"],
            Some(&json!({"name":42})),
        )
        .unwrap();
        assert_eq!(repaired.body.unwrap()["name"], "Valid");
        let repaired = prepare(
            "drive",
            &["files.list", "--max", "2"],
            Some(&json!({"max":false})),
        )
        .unwrap();
        assert_eq!(repaired.options.unwrap().get_one::<u32>("max"), Some(&2));
        let call = prepare(
            "sheets",
            &["create", "--title", "New"],
            Some(&json!({"title":"Old","propertiesLocale":"en_US"})),
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap(),
            json!({"properties":{"title":"New","locale":"en_US"}})
        );
        let call = prepare(
            "calendar",
            &["events.insert", "--end-time", "2026-12-01T11:00:00Z"],
            Some(&json!({"summary":"Plan","startTime":"2026-12-01T10:00:00Z"})),
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap()["end"],
            json!({"dateTime":"2026-12-01T11:00:00Z"})
        );
        let call = prepare("docs", &["update-text-style", "--text-style-bold=false"], Some(&json!({"documentId":"doc","bodyFields":"bold","textStyleBold":true,"rangeStartIndex":1,"rangeEndIndex":3}))).unwrap();
        assert_eq!(
            call.body.unwrap()["requests"][0]["updateTextStyle"]["textStyle"]["bold"],
            false
        );
        assert!(
            prepare(
                "photos",
                &["albums.create"],
                Some(&json!({"unknown":"invalid"}))
            )
            .is_err()
        );
    }

    #[test]
    fn public_command_rules_cover_requirements_conflicts_and_record_fields() {
        let cases: Value =
            serde_json::from_str(include_str!("../tests/data/cli-inputs.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let service = case["service"].as_str().unwrap();
            let name = case["command"].as_str().unwrap();
            crate::file_inputs::validate_rules(&contract(service).inputs[name], &case["file"])
                .unwrap_or_else(|error| panic!("{service} {name}: {error}"));
        }
        let clear = json!({"fileId":"file", "name":null});
        crate::file_inputs::validate_rules(&contract("drive").inputs["files.update"], &clear)
            .unwrap();
        assert!(prepare("drive", &["files.update"], Some(&clear)).is_ok());
        for (service, name, input) in [
            ("drive", "files.get", json!({})),
            ("forms", "create", json!({})),
            ("mail", "messages.send", json!({"to":[], "subject":"Hello"})),
            (
                "mail",
                "messages.send",
                json!({"to":["a@example.com"], "raw":"encoded"}),
            ),
            (
                "mail",
                "messages.send",
                json!({"to":["a@example.com"], "text":"inline", "textFile":"body.txt"}),
            ),
            (
                "calendar",
                "events.insert",
                json!({"startDate":"2026-12-01", "startTime":"2026-12-01T10:00:00Z", "endDate":"2026-12-02"}),
            ),
            (
                "calendar",
                "events.insert",
                json!({"body":{"start":{}, "end":{"date":"2026-12-02"}}}),
            ),
            (
                "photos",
                "media-items.search",
                json!({"albumId":"album", "mediaTypes":["PHOTO"]}),
            ),
            (
                "drive",
                "files.modify-labels",
                json!({"fileId":"file", "labelModifications":[{}]}),
            ),
            (
                "mail",
                "messages.send",
                json!({"raw":"encoded", "contentType":"message/rfc822"}),
            ),
        ] {
            let schema = &contract(service).inputs[name];
            let error = crate::file_inputs::validate_rules(schema, &input).unwrap_err();
            assert!(
                schema["allOf"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|rule| rule["description"].as_str() == Some(error.to_string().as_str()))
            );
        }
        fn checked_keywords(rule: &Value) {
            for (key, value) in rule.as_object().unwrap() {
                match key.as_str() {
                    "properties" => value
                        .as_object()
                        .unwrap()
                        .values()
                        .for_each(checked_keywords),
                    "allOf" | "anyOf" => {
                        value.as_array().unwrap().iter().for_each(checked_keywords)
                    }
                    "if" | "then" | "else" | "not" | "items" => checked_keywords(value),
                    "required" | "type" | "minItems" | "description" => {}
                    key if key.starts_with("x-cli-") => {}
                    key => panic!("command assertion uses unsupported keyword {key}"),
                }
            }
        }
        for service in SERVICES {
            for schema in contract(service.cli).inputs.values() {
                for rule in schema["allOf"].as_array().into_iter().flatten() {
                    checked_keywords(rule);
                    assert!(
                        rule["description"]
                            .as_str()
                            .is_some_and(|text| !text.is_empty())
                    );
                    for group in rule["x-cli-groups"].as_array().into_iter().flatten() {
                        for key in group.as_array().unwrap() {
                            assert!(schema["properties"].get(key.as_str().unwrap()).is_some());
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn merged_constraints_reject_files_and_raw_bodies_before_execution() {
        for (service, arguments, file) in [
            (
                "calendar",
                vec!["events.insert", "--start-time", "2026-12-01T10:00:00Z"],
                json!({"startDate":"2026-12-01","endDate":"2026-12-02"}),
            ),
            (
                "calendar",
                vec!["events.insert"],
                json!({"body":{"start":{},"end":{"date":"2026-12-02"}}}),
            ),
            (
                "photos",
                vec!["media-items.search"],
                json!({"albumId":"album","mediaTypes":["PHOTO"]}),
            ),
            (
                "photos",
                vec!["media-items.search"],
                json!({"body":{"albumId":"album","filters":{}}}),
            ),
            (
                "mail",
                vec!["messages.send", "--text", "Body"],
                json!({"to":["recipient@example.com"],"raw":"encoded"}),
            ),
            ("forms", vec!["create"], json!({})),
        ] {
            assert!(
                prepare(service, &arguments, Some(&file)).is_err(),
                "accepted {service} {arguments:?} {file}"
            );
        }
        // Explicit options replace their file counterpart before conflict checks.
        assert!(
            prepare(
                "mail",
                &["messages.send", "--text", "Body"],
                Some(&json!({"to":["recipient@example.com"],"textFile":"must-not-be-read.txt"}))
            )
            .is_ok()
        );
        // A partial event patch can update the time zone without replacing dates.
        assert!(
            prepare(
                "calendar",
                &["events.patch", "--event-id", "event"],
                Some(&json!({"startTimeZone":"Europe/Paris"}))
            )
            .is_ok()
        );
    }

    #[test]
    fn input_validation_applies_to_every_service() {
        let cases: Value =
            serde_json::from_str(include_str!("../tests/data/cli-inputs.json")).unwrap();
        for (service, invalid) in [
            ("mail", json!({"subject":42})),
            ("drive", json!({"name":42})),
            ("calendar", json!({"summary":42})),
            ("sheets", json!({"title":42})),
            ("docs", json!({"text":42})),
            ("slides", json!({"layout":"INVALID"})),
            ("forms", json!({"title":42})),
            ("tasks", json!({"title":42})),
            ("contacts", json!({"names":42})),
            ("youtube", json!({"privacy":"INVALID"})),
            ("photos", json!({"title":42})),
        ] {
            let case = cases
                .as_array()
                .unwrap()
                .iter()
                .find(|case| case["service"] == service)
                .unwrap();
            let mut file = case["file"].as_object().unwrap().clone();
            file.extend(invalid.as_object().unwrap().clone());
            assert!(
                prepare(
                    service,
                    &[case["command"].as_str().unwrap()],
                    Some(&Value::Object(file))
                )
                .is_err(),
                "accepted invalid {service} inputs"
            );
        }
    }

    #[test]
    fn mail_composition_builds_recipients_and_mime_for_messages_and_drafts() {
        let call = prepare(
            "mail",
            &[
                "messages.send",
                "--to",
                "alice@example.com",
                "--to",
                "bob@example.com",
                "--cc",
                "carol@example.com",
                "--bcc",
                "archive@example.com",
                "--reply-to",
                "replies@example.com",
                "--subject",
                "Report",
                "--text",
                "Plain report",
                "--html",
                "<p>HTML report</p>",
            ],
            None,
        )
        .unwrap();
        assert_eq!(
            call.params,
            json!({"userId":"me"}).as_object().unwrap().clone()
        );
        let message = mime(&call);
        for header in [
            "From: <reader@example.com>",
            "To: <alice@example.com>, <bob@example.com>",
            "Cc: <carol@example.com>",
            "Bcc: <archive@example.com>",
            "Reply-To: <replies@example.com>",
            "Subject: Report",
            "multipart/alternative",
            "text/plain",
            "text/html",
            "Plain report",
            "<p>HTML report</p>",
        ] {
            assert!(message.contains(header), "missing {header}: {message}");
        }
        for (method, flags) in [
            ("drafts.create", vec![]),
            ("drafts.update", vec!["--id", "draft-id"]),
        ] {
            let mut args = vec![method, "--subject", "Unfinished draft"];
            args.extend(flags);
            let draft = prepare("mail", &args, None).unwrap();
            assert!(draft.body.as_ref().unwrap()["message"]["raw"].is_string());
            assert!(mime(&draft).contains("Subject: Unfinished draft"));
        }
        // A Bcc-only send is valid, and an explicit sender replaces ACCOUNT.
        let blind = prepare(
            "mail",
            &[
                "messages.send",
                "--bcc",
                "archive@example.com",
                "--from",
                "alias@example.com",
            ],
            None,
        )
        .unwrap();
        assert!(mime(&blind).contains("From: <alias@example.com>"));
        for method in ["messages.import", "messages.insert"] {
            let call = prepare("mail", &[method], Some(&json!({"from":"original@example.com","to":["recipient@example.com"],"subject":"Archived","text":"Original content"}))).unwrap();
            assert!(mime(&call).contains("Subject: Archived"));
        }
        let draft = prepare("mail", &["drafts.send"], Some(&json!({"id":"draft-id","to":["recipient@example.com"],"subject":"Send draft","text":"Body"}))).unwrap();
        assert_eq!(draft.body.as_ref().unwrap()["id"], "draft-id");
        assert!(mime(&draft).contains("Subject: Send draft"));
        let existing = prepare("mail", &["drafts.send", "--id", "draft-id"], None).unwrap();
        assert_eq!(existing.body.unwrap(), json!({"id":"draft-id"}));
    }

    #[test]
    fn mail_composition_merges_files_and_keeps_google_metadata() {
        let call = prepare("mail", &["messages.send", "--to", "new@example.com", "--subject", "New subject", "--text", "New body"], Some(&json!({
            "compose":{"to":["old@example.com"],"cc":["keep@example.com"],"subject":"Old subject","textFile":"must-not-be-read.txt"},
            "body":{"labelIds":["label-id"]}
        }))).unwrap();
        let message = mime(&call);
        assert!(message.contains("To: <new@example.com>"));
        assert!(!message.contains("old@example.com"));
        assert!(message.contains("Cc: <keep@example.com>"));
        assert!(message.contains("Subject: New subject"));
        assert!(message.contains("New body"));
        assert_eq!(call.body.unwrap()["labelIds"], json!(["label-id"]));
        let file: Value =
            serde_json::from_str(include_str!("../examples/requests/mail-compose.json")).unwrap();
        assert!(
            mime(&prepare("mail", &["messages.send"], Some(&file)).unwrap())
                .contains("Quarterly report")
        );
        // Existing raw files keep their exact body.
        let raw = json!({"raw":"b3JpZ2luYWw"});
        assert_eq!(
            prepare("mail", &["messages.send"], Some(&json!({"body":raw})))
                .unwrap()
                .body
                .unwrap(),
            raw
        );
    }

    #[test]
    fn mail_composition_reads_utf8_body_files_and_binary_attachments() {
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("body.txt");
        let attachment = dir.path().join("report.bin");
        std::fs::write(&text, "A UTF-8 body: café").unwrap();
        std::fs::write(&attachment, [0, 1, 2, 255]).unwrap();
        let call = prepare(
            "mail",
            &[
                "messages.send",
                "--to",
                "alice@example.com",
                "--subject",
                "Résumé",
                "--text-file",
                text.to_str().unwrap(),
                "--attach",
                attachment.to_str().unwrap(),
            ],
            None,
        )
        .unwrap();
        let message = mime(&call);
        assert!(message.contains("multipart/mixed"));
        assert!(message.contains("application/octet-stream"));
        assert!(message.contains("report.bin"));
        assert!(message.contains("AAEC/w=="));
        assert!(message.to_ascii_lowercase().contains("=?utf-8?"));
        let invalid = dir.path().join("invalid.txt");
        std::fs::write(&invalid, [255]).unwrap();
        assert!(
            prepare(
                "mail",
                &[
                    "messages.send",
                    "--to",
                    "alice@example.com",
                    "--text-file",
                    invalid.to_str().unwrap()
                ],
                None
            )
            .is_err()
        );
    }

    #[test]
    fn mail_composition_enforces_input_and_encoded_request_limits() {
        let file = tempfile::NamedTempFile::new().unwrap();
        file.as_file().set_len(http::DISCOVERY_LIMIT + 1).unwrap();
        let error = prepare(
            "mail",
            &[
                "messages.send",
                "--to",
                "alice@example.com",
                "--attach",
                file.path().to_str().unwrap(),
            ],
            None,
        )
        .err()
        .unwrap();
        assert!(format!("{error:#}").contains("16 MiB input limit"));
        // The MIME content fits the input limit but its base64url JSON does not.
        std::fs::write(file.path(), "x".repeat(13 * 1024 * 1024)).unwrap();
        let error = prepare(
            "mail",
            &[
                "messages.send",
                "--to",
                "alice@example.com",
                "--text-file",
                file.path().to_str().unwrap(),
            ],
            None,
        )
        .err()
        .unwrap();
        assert!(error.to_string().contains("16 MiB JSON request limit"));
    }

    #[test]
    fn invalid_or_ambiguous_mail_composition_is_rejected_locally() {
        for (args, file) in [
            (
                vec!["messages.send", "--subject", "No recipient"],
                json!({}),
            ),
            (
                vec!["messages.send", "--to", "Name <alice@example.com>"],
                json!({}),
            ),
            (
                vec!["messages.send", "--to", "alice@example.com,bob@example.com"],
                json!({}),
            ),
            (
                vec![
                    "messages.send",
                    "--to",
                    "alice@example.com\r\nBcc:evil@example.com",
                ],
                json!({}),
            ),
            (
                vec![
                    "messages.send",
                    "--to",
                    "alice@example.com",
                    "--subject",
                    "Hello\nInjected: value",
                ],
                json!({}),
            ),
            (
                vec![
                    "messages.send",
                    "--to",
                    "alice@example.com",
                    "--upload",
                    "must-not-be-read.eml",
                ],
                json!({}),
            ),
            (
                vec!["messages.send", "--to", "alice@example.com"],
                json!({"body":{"raw":"encoded"}}),
            ),
            (
                vec!["messages.send", "--to", "alice@example.com"],
                json!({"body":{"payload":{}}}),
            ),
            (
                vec![
                    "messages.send",
                    "--to",
                    "alice@example.com",
                    "--text",
                    "inline",
                    "--text-file",
                    "other.txt",
                ],
                json!({}),
            ),
            (
                vec!["messages.send"],
                json!({"compose":{"to":["alice@example.com"],"text":"inline","textFile":"other.txt"}}),
            ),
            (
                vec!["messages.send"],
                json!({"compose":{"to":"alice@example.com"}}),
            ),
            (
                vec!["messages.send"],
                json!({"compose":{"to":["alice@example.com"],"unknown":true}}),
            ),
            (vec!["messages.send"], json!({"compose":null})),
            (
                vec!["drafts.send"],
                json!({"compose":{"to":["alice@example.com"]}}),
            ),
        ] {
            assert!(
                prepare("mail", &args, Some(&file)).is_err(),
                "accepted {args:?} {file}"
            );
        }
    }

    #[test]
    fn adversarial_input_types_are_rejected_for_all_public_parameters() {
        let mut commands = 0;
        let mut parameters = 0;
        let mut arbitrary_json = 0;
        for service in SERVICES {
            let contract = contract(service.cli);
            for (name, schema) in &contract.inputs {
                for (key, shape) in schema["properties"].as_object().unwrap() {
                    let resolved = type_schema(&contract.document, shape);
                    // Discovery deliberately leaves a few marker values untyped.
                    if resolved.get("type").is_none() && resolved.get("enum").is_none() {
                        arbitrary_json += 1;
                        continue;
                    }
                    let invalid = if resolved["type"] == "object" {
                        json!([])
                    } else {
                        json!({"injected":true})
                    };
                    assert!(
                        validate_body(&contract.document, "", shape, &invalid, key).is_err(),
                        "{} {name} accepted the wrong JSON type for {key}",
                        service.cli
                    );
                    parameters += 1;
                }
                commands += 1;
            }
        }
        assert_eq!(commands, 340);
        assert_eq!(parameters + arbitrary_json, 5065);
        assert!(parameters > 4900);
    }

    #[test]
    fn every_schema_method_has_a_registered_command_and_builds_the_declared_request() {
        let mut implemented = 0;
        let file = tempfile::NamedTempFile::new().unwrap();
        for service in SERVICES {
            let doc = &contract(service.cli).document;
            for (name, spec) in &doc.root.methods {
                command(service.cli, name).debug_assert();
                let help = cli::parse_from(["joog", "help", service.cli, name]).unwrap_err();
                assert_eq!(
                    help.kind(),
                    clap::error::ErrorKind::DisplayHelp,
                    "{} {name}",
                    service.cli
                );
                let mut params = Map::new();
                for (key, parameter) in &spec.parameters {
                    if parameter.required {
                        params.insert(key.clone(), scalar(parameter));
                    }
                }
                if let Some(request) = input_request(spec) {
                    params.insert("body".into(), sample(doc, request, &spec.id, 0));
                }
                if contract(service.cli).inputs[name]["allOf"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|rule| {
                        rule["x-cli-kind"] == "required"
                            && rule["x-cli-parameters"] == json!(["upload"])
                    })
                {
                    params.insert("upload".into(), json!("fixture.png"));
                }
                std::fs::write(file.path(), serde_json::to_vec(&params).unwrap()).unwrap();
                let mut argv = vec![
                    "joog",
                    service.cli,
                    "reader@example.com",
                    name,
                    "--file",
                    file.path().to_str().unwrap(),
                ];
                if direct_download(spec) {
                    argv.extend(["--output", "download.bin"]);
                }
                let matches = cli::parse_from(argv)
                    .unwrap_or_else(|error| panic!("{} {name}: {error}", service.cli));
                let result = crate::service_call(service, matches.subcommand().unwrap().1);
                let call =
                    result.unwrap_or_else(|error| panic!("{} {name}: {error:#}", service.cli));
                assert_eq!(call.method, *name);
                assert_eq!(doc.method(name).unwrap().http_method, spec.http_method);
                let url = doc.url(spec, &call.params).unwrap();
                crate::http::google_url(&url).unwrap();
                for (key, value) in &params {
                    if !matches!(key.as_str(), "body" | "upload") {
                        assert_eq!(
                            call.params.get(key),
                            Some(value),
                            "{} {name}: file input {key} was lost",
                            service.cli
                        );
                    }
                }
                if let Some(body) = call.body.as_ref() {
                    validate_body(doc, &spec.id, spec.request.as_ref().unwrap(), body, "body")
                        .unwrap();
                }
                let scopes = crate::services::scopes(service.cli).unwrap();
                assert!(
                    spec.scopes.is_empty()
                        || spec.scopes.iter().any(|scope| scopes.contains(scope)),
                    "{} {name}: missing login permission",
                    service.cli
                );
                if spec.media_upload.is_some() {
                    doc.upload_url(spec, &call.params).unwrap();
                }
                let fields = command(service.cli, name)
                    .get_arguments()
                    .filter_map(|argument| argument.get_long())
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
                assert!(!fields.iter().any(|field| field == "params"));
                if fields.iter().any(|field| field == "max") {
                    pagination::Plan::new(spec, true, Some(1), None).unwrap();
                }
                if fields.iter().any(|field| field == "max-pages") {
                    pagination::Plan::new(spec, true, None, Some(1)).unwrap();
                }
                params.insert("__proto__".into(), json!({"injected":true}));
                std::fs::write(file.path(), serde_json::to_vec(&params).unwrap()).unwrap();
                assert!(
                    crate::service_call(service, matches.subcommand().unwrap().1).is_err(),
                    "{} {name} accepted an unknown file field",
                    service.cli
                );
                implemented += 1;
            }
        }
        assert_eq!(implemented, 330);
    }

    #[test]
    fn named_parameters_and_body_fields_work_with_files_and_preserve_explicit_values() {
        let call = prepare(
            "drive",
            &[
                "permissions.create",
                "--file-id",
                "document",
                "--type",
                "user",
                "--role",
                "reader",
                "--email-address",
                "alice@example.com",
            ],
            None,
        )
        .unwrap();
        assert_eq!(call.params["fileId"], "document");
        assert_eq!(
            call.body.unwrap(),
            json!({"type":"user","role":"reader","emailAddress":"alice@example.com"})
        );
        let call=prepare("drive",&["files.update","--file-id","document","--name","New name","--supports-all-drives=false"],Some(&json!({"fileId":"old","supportsAllDrives":true,"body":{"description":"Keep this","name":"Old name"}}))).unwrap();
        assert_eq!(call.params["fileId"], "document");
        assert_eq!(call.params["supportsAllDrives"], false);
        assert_eq!(
            call.body.unwrap(),
            json!({"description":"Keep this","name":"New name"})
        );
        let call = prepare(
            "calendar",
            &["events.get", "--event-id", "event"],
            Some(&json!({"calendarId":"my-calendar"})),
        )
        .unwrap();
        assert_eq!(call.params["calendarId"], "my-calendar");
        let call = prepare(
            "youtube",
            &[
                "playlists.insert",
                "--snippet",
                "{\"title\":\"Example\"}",
                "--status",
                "{\"privacyStatus\":\"private\"}",
            ],
            None,
        )
        .unwrap();
        assert_eq!(call.params["part"], "snippet,status");
        let call = prepare(
            "mail",
            &[
                "messages.modify",
                "--id",
                "message",
                "--add-label-ids",
                "INBOX",
                "STARRED",
                "--remove-label-ids",
                "UNREAD",
            ],
            None,
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap(),
            json!({"addLabelIds":["INBOX","STARRED"],"removeLabelIds":["UNREAD"]})
        );
        assert_eq!(call.params["userId"], "me");
        let call = prepare(
            "sheets",
            &[
                "values.append",
                "--spreadsheet-id",
                "sheet",
                "--range",
                "A1",
                "--value-input-option",
                "RAW",
                "--values",
                "[[\"Name\",1]]",
            ],
            None,
        )
        .unwrap();
        assert_eq!(call.body.unwrap()["values"], json!([["Name", 1]]));
    }

    #[test]
    fn invalid_requests_are_rejected_before_authentication() {
        for (service, args, file) in [
            ("drive", vec!["files.get"], json!({})),
            ("drive", vec!["files.get"], json!({"fileId":".."})),
            (
                "drive",
                vec!["files.get"],
                json!({"fileId":"file","access_token":"must-not-be-used"}),
            ),
            (
                "drive",
                vec!["files.get"],
                json!({"fileId":"file","body":{}}),
            ),
            (
                "drive",
                vec!["files.create"],
                json!({"body":{"inventedField":"no"}}),
            ),
            (
                "drive",
                vec!["files.create", "--body", "{}"],
                json!({"body":{"name":"Conflicting body"}}),
            ),
            (
                "youtube",
                vec!["playlists.insert"],
                json!({"body":{"snippet":{"title":"Example"},"status":{"privacyStatus":"INVALID"}}}),
            ),
            (
                "youtube",
                vec!["videos.update"],
                json!({"part":"snippet","body":{"snippet":{"title":"Missing video ID"}}}),
            ),
            (
                "photos",
                vec!["media-items.search"],
                json!({"body":{"pageSize":"wrong type"}}),
            ),
        ] {
            assert!(
                prepare(service, &args, Some(&file)).is_err(),
                "{service} {args:?}"
            );
        }
    }

    #[test]
    fn canonical_names_and_global_option_positions_select_the_registered_command() {
        for args in [
            vec![
                "joog",
                "drive",
                "user@example.com",
                "files.get",
                "--file-id",
                "file",
            ],
            vec![
                "joog",
                "--home",
                "drive",
                "drive",
                "user@example.com",
                "--json",
                "files.get",
                "--file-id",
                "file",
            ],
        ] {
            let parsed = cli::parse_from(args).unwrap();
            let context = parsed.subcommand().unwrap().1;
            assert_eq!(context.subcommand_name(), Some("files.get"));
        }
        assert_eq!(
            cli::parse_from(["joog", "--home", "drive", "help", "drive", "files.get"])
                .unwrap_err()
                .kind(),
            clap::error::ErrorKind::DisplayHelp
        );
        assert!(
            cli::parse_from([
                "joog",
                "drive",
                "user@example.com",
                "files.get",
                "--file-id",
                "file",
                "--supports-all-drives",
                "--json"
            ])
            .is_err()
        );
        assert!(cli::parse_from(["joog", "drive", "user@example.com", "unknown.method"]).is_err());
    }
    #[test]
    fn create_titles_override_nested_content_without_dropping_other_fields() {
        for (service, key, other) in [
            ("sheets", "properties", "locale"),
            ("forms", "info", "description"),
        ] {
            let mut content = json!({key:{"title":"Old title",other:"Keep this"}});
            if service == "sheets" {
                content[key][other] = json!("en_US");
            }
            let file = json!({"body":content});
            let call = prepare(service, &["create", "--title", "New title"], Some(&file)).unwrap();
            let body = call.body.unwrap();
            assert_eq!(body[key]["title"], "New title");
            assert_eq!(body[key][other], file["body"][key][other]);
            assert_eq!(
                prepare(service, &["create", "--title", "Only title"], None)
                    .unwrap()
                    .body
                    .unwrap()[key]["title"],
                "Only title"
            );
        }
    }
    #[test]
    fn docs_and_slides_editing_commands_build_one_typed_batch_request() {
        let call = prepare(
            "docs",
            &[
                "insert-text",
                "--document-id",
                "document",
                "--text",
                "Hello",
                "--location",
                r#"{"index":1}"#,
                "--write-control",
                r#"{"requiredRevisionId":"revision"}"#,
            ],
            None,
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap(),
            json!({"requests":[{"insertText":{"text":"Hello","location":{"index":1}}}],"writeControl":{"requiredRevisionId":"revision"}})
        );
        let file = json!({"presentationId":"presentation","body":{"objectId":"slide","slideLayoutReference":{"predefinedLayout":"TITLE_AND_BODY"}},"writeControl":{"requiredRevisionId":"old"}});
        let call = prepare(
            "slides",
            &[
                "create-slide",
                "--write-control",
                r#"{"requiredRevisionId":"new"}"#,
            ],
            Some(&file),
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap(),
            json!({"requests":[{"createSlide":file["body"]}],"writeControl":{"requiredRevisionId":"new"}})
        );
        let call = prepare(
            "slides",
            &[
                "insert-text",
                "--presentation-id",
                "presentation",
                "--object-id",
                "shape",
                "--text",
                "Hello",
            ],
            None,
        )
        .unwrap();
        assert_eq!(
            call.body.unwrap(),
            json!({"requests":[{"insertText":{"objectId":"shape","text":"Hello"}}]})
        );
        assert!(
            prepare(
                "docs",
                &["insert-text"],
                Some(&json!({"documentId":"document","body":{"text":42}}))
            )
            .is_err()
        );
        assert!(
            prepare(
                "slides",
                &["create-slide"],
                Some(&json!({"presentationId":"presentation","writeControl":"invalid"}))
            )
            .is_err()
        );
        for service in ["docs", "slides"] {
            let doc = &contract(service).document;
            let operations = type_schema(doc, &json!({"$ref":"Request"}))["properties"]
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>();
            let implemented = doc
                .root
                .methods
                .values()
                .filter_map(|method| method.batch_operation.clone())
                .collect::<BTreeSet<_>>();
            assert_eq!(implemented, operations, "{service} editing coverage");
        }
    }
    #[test]
    fn service_references_match_every_registered_method_and_option() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for service in SERVICES {
            let expected = reference(service.cli);
            let actual =
                std::fs::read_to_string(root.join(format!("docs/services/{}.md", service.cli)))
                    .unwrap();
            assert!(
                actual == expected,
                "{} method reference differs from command help",
                service.cli
            );
        }
    }
    #[test]
    fn every_service_write_example_builds_its_documented_request() {
        let examples: Value =
            serde_json::from_str(include_str!("../tests/data/write-examples.json")).unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for service in SERVICES {
            let name = examples[service.cli].as_str().unwrap();
            let file: Value = serde_json::from_slice(
                &std::fs::read(root.join(format!("examples/requests/{}-write.json", service.cli)))
                    .unwrap(),
            )
            .unwrap();
            let call = prepare(service.cli, &[name], Some(&file))
                .unwrap_or_else(|error| panic!("{} {name}: {error:#}", service.cli));
            assert_eq!(call.method, name);
            assert!(call.body.is_some());
        }
    }
}
