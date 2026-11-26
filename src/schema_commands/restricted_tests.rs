use super::{contract, tests::prepare};
use crate::http::Http;
use reqwest::Method;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};

// These fixtures assert Google's documented wire contract independently of the
// generated catalog. They never read credentials or connect outside loopback.
#[test]
fn restricted_commands_send_exact_requests_and_handle_permission_errors() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../tests/data/restricted-requests.json")).unwrap();
    assert_eq!(cases.len(), 14);
    for fixture in cases {
        for status in [200, 403] {
            transport_case(&fixture, status);
        }
        if fixture["command"] == "calendars.clear" {
            // A write receiving a transient error must not be retried.
            transport_case(&fixture, 503);
        }
    }
}

fn transport_case(fixture: &Value, status: u16) {
    let service = fixture["service"].as_str().unwrap();
    let name = fixture["command"].as_str().unwrap();
    let call = prepare(service, &[name], Some(&fixture["input"]))
        .unwrap_or_else(|error| panic!("{service} {name}: {error:#}"));
    let document = &contract(service).document;
    let method = document.method(name).unwrap();
    let mut address = document.url(method, &call.params).unwrap();
    crate::http::google_url(&address).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    let expected = fixture.clone();
    let server = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "request did not reach loopback");
                    std::thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("{error}"),
            }
        };
        stream.set_nonblocking(false).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert_eq!(Some(parts[0]), expected["method"].as_str());
        let request = url::Url::parse(&format!("http://127.0.0.1{}", parts[1])).unwrap();
        assert_eq!(
            percent_encoding::percent_decode_str(request.path())
                .decode_utf8()
                .unwrap(),
            expected["path"].as_str().unwrap()
        );
        let query = request
            .query_pairs()
            .into_owned()
            .collect::<BTreeMap<_, _>>();
        assert_eq!(serde_json::to_value(query).unwrap(), expected["query"]);
        let mut headers = BTreeMap::new();
        loop {
            line.clear();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            assert!(line.len() <= 8192);
            if line == "\r\n" {
                break;
            }
            let (key, value) = line.split_once(':').unwrap();
            headers.insert(key.to_ascii_lowercase(), value.trim().to_owned());
            assert!(headers.len() < 32);
        }
        assert!(!headers.contains_key("authorization"));
        let length = headers
            .get("content-length")
            .map_or(0, |value| value.parse::<usize>().unwrap());
        assert!(length < 16384);
        let mut body = vec![0; length];
        reader.read_exact(&mut body).unwrap();
        if expected["body"].is_null() {
            assert!(body.is_empty());
            if expected["method"] == "POST" {
                assert_eq!(headers.get("content-length").map(String::as_str), Some("0"));
            }
        } else {
            assert_eq!(headers["content-type"], "application/json");
            assert_eq!(
                serde_json::from_slice::<Value>(&body).unwrap(),
                expected["body"]
            );
        }
        let response = if status == 200 {
            r#"{"fixture":"ok"}"#
        } else {
            r#"{"error":{"message":"private-fixture-message","code":403}}"#
        };
        write!(stream, "HTTP/1.1 {status} Fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",response.len()).unwrap();
    });
    // Change the destination only after validating the real Google URL. The
    // public client still forbids sending bearer tokens to this HTTP address.
    address.set_scheme("http").unwrap();
    address.set_host(Some("127.0.0.1")).unwrap();
    address.set_port(Some(port)).unwrap();
    let result = Http::new(2).unwrap().json(
        &Method::from_bytes(method.http_method.as_bytes()).unwrap(),
        &address,
        call.body.as_ref(),
        None,
        1024,
    );
    server.join().unwrap();
    if status == 200 {
        assert_eq!(result.unwrap(), json!({"fixture":"ok"}));
    } else {
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains(&format!("HTTP {status}")),
            "{service} {name}: {error}"
        );
        assert!(!error.contains("private-fixture-message"));
    }
}

#[test]
fn youtube_media_requirements_match_files_arguments_and_schema_validation() {
    for (name, input) in [
        ("channel-banners.insert", json!({"upload":"fixture.png"})),
        (
            "watermarks.set",
            json!({"channelId":"fixture","timing":{"type":"offsetFromStart","offsetMs":"0"},"upload":"fixture.png"}),
        ),
        (
            "playlist-images.insert",
            json!({"snippet":{"playlistId":"fixture","type":"hero"},"upload":"fixture.png"}),
        ),
        (
            "playlist-images.update",
            json!({"id":"fixture","snippet":{"playlistId":"fixture","type":"hero"},"upload":"fixture.png"}),
        ),
    ] {
        let schema = &contract("youtube").inputs[name];
        prepare("youtube", &[name], Some(&input)).unwrap();
        crate::file_inputs::validate_rules(schema, &input).unwrap();
        let method = &contract("youtube").document.root.methods[name];
        let file = tempfile::NamedTempFile::new().unwrap();
        let limit = schema["properties"]["upload"]["x-file-max-bytes"]
            .as_u64()
            .unwrap();
        file.as_file().set_len(limit + 1).unwrap();
        assert!(
            super::validate_upload(method, file.path().to_str().unwrap(), "image/png").is_err()
        );
        file.as_file().set_len(1).unwrap();
        super::validate_upload(method, file.path().to_str().unwrap(), "image/png").unwrap();
        assert!(
            super::validate_upload(method, file.path().to_str().unwrap(), "text/plain").is_err()
        );
        let mut missing = input.clone();
        missing.as_object_mut().unwrap().remove("upload");
        assert!(prepare("youtube", &[name], Some(&missing)).is_err());
        assert!(crate::file_inputs::validate_rules(schema, &missing).is_err());
        prepare(
            "youtube",
            &[name, "--upload", "fixture.png"],
            Some(&missing),
        )
        .unwrap();
    }
    for timing in [
        json!({}),
        json!({"type":"offsetFromStart"}),
        json!({"offsetMs":"0"}),
    ] {
        let input = json!({"channelId":"fixture","upload":"fixture.png","timing":timing});
        assert!(prepare("youtube", &["watermarks.set"], Some(&input)).is_err());
        assert!(
            crate::file_inputs::validate_rules(
                &contract("youtube").inputs["watermarks.set"],
                &input
            )
            .is_err()
        );
    }
}
