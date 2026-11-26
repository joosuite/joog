use crate::discovery::MethodSpec;
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value};
use std::collections::HashSet;

pub struct Plan {
    pub paginated: bool,
    maximum: Option<u32>,
    pages: Option<u32>,
    page_size: Option<(String, u32, u32)>,
}

impl Plan {
    pub fn new(
        spec: &MethodSpec,
        read: bool,
        maximum: Option<u32>,
        pages: Option<u32>,
    ) -> Result<Self> {
        let paginated = read
            && (spec.parameters.contains_key("pageToken")
                || spec.body_parameters.contains_key("pageToken"));
        ensure!(
            pages.is_none() || paginated,
            "--max-pages requires a paginated read method"
        );
        let page_size = ["pageSize", "maxResults"].into_iter().find_map(|name| {
            spec.parameters
                .get(name)
                .or_else(|| spec.body_parameters.get(name))
                .map(|parameter| {
                    (
                        name.to_owned(),
                        parameter
                            .minimum
                            .as_deref()
                            .and_then(|value| value.parse::<u32>().ok())
                            .unwrap_or(1),
                        parameter
                            .maximum
                            .as_deref()
                            .and_then(|value| value.parse::<u32>().ok())
                            .unwrap_or(100),
                    )
                })
        });
        let list = paginated
            || page_size.is_some()
            || spec.id.ends_with(".list")
            || spec.id.ends_with(".search");
        ensure!(
            maximum.is_none() || (read && list),
            "--max requires a read method returning a list"
        );
        Ok(Self {
            paginated,
            maximum,
            pages,
            page_size,
        })
    }
}

fn collection(value: &mut Value) -> Result<Option<&mut Vec<Value>>> {
    if value.is_array() {
        return Ok(value.as_array_mut());
    }
    let Some(object) = value.as_object_mut() else {
        return Ok(None);
    };
    let key = if object.get("items").is_some_and(Value::is_array) {
        Some("items".to_owned())
    } else {
        let mut arrays = object.iter().filter(|(_, value)| value.is_array());
        let first = arrays.next().map(|(key, _)| key.clone());
        ensure!(
            arrays.next().is_none(),
            "cannot apply --max to multiple result lists; select the result list with --fields"
        );
        first
    };
    Ok(key
        .and_then(|key| object.get_mut(&key))
        .and_then(Value::as_array_mut))
}

pub fn run(
    params: &mut Map<String, Value>,
    plan: Plan,
    mut fetch: impl FnMut(&Map<String, Value>) -> Result<Value>,
    mut emit: impl FnMut(&Value, bool) -> Result<()>,
) -> Result<()> {
    if plan.paginated
        && let Some(Value::String(fields)) = params.get_mut("fields")
        && !fields
            .split(',')
            .any(|field| matches!(field.trim(), "*" | "nextPageToken"))
    {
        fields.push_str(",nextPageToken");
    }
    let mut remaining = plan.maximum;
    let mut pages = 0_u64;
    let mut seen = HashSet::new();
    if let Some(Value::String(token)) = params.get("pageToken") {
        seen.insert(token.clone());
    }
    loop {
        if let (Some(remaining), Some((name, api_min, api_max))) = (remaining, &plan.page_size) {
            let requested = params
                .get(name)
                .and_then(|value| {
                    value
                        .as_u64()
                        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
                })
                .unwrap_or(u64::from(*api_max));
            params.insert(
                name.clone(),
                Value::from(
                    u64::from(remaining)
                        .min(u64::from(*api_max))
                        .min(requested)
                        .max(u64::from(*api_min)),
                ),
            );
        }
        let mut value = fetch(params)?;
        let next = value
            .get("nextPageToken")
            .and_then(Value::as_str)
            .filter(|token| !token.is_empty())
            .map(str::to_owned);
        if let Some(limit) = remaining {
            if let Some(rows) = collection(&mut value)? {
                rows.truncate(limit as usize);
                remaining = Some(limit - u32::try_from(rows.len()).context("too many results")?);
            } else if !plan.paginated && !value.is_null() {
                // An empty list response may omit its collection entirely.
                let empty = value.as_object().is_some_and(|object| {
                    object
                        .keys()
                        .all(|key| matches!(key.as_str(), "kind" | "etag" | "resultSizeEstimate"))
                });
                ensure!(
                    empty,
                    "--max requires list results; include the result list in --fields"
                );
            }
        }
        emit(&value, plan.paginated)?;
        pages += 1;
        if !plan.paginated
            || next.is_none()
            || remaining == Some(0)
            || plan
                .pages
                .is_some_and(|maximum| pages >= u64::from(maximum))
        {
            return Ok(());
        }
        let token = next.context("missing page token")?;
        ensure!(
            token.len() <= 8192,
            "Google returned an oversized page token"
        );
        if !seen.insert(token.clone()) {
            bail!("Google returned a repeated page token; output contains partial results");
        }
        params.insert("pageToken".into(), Value::String(token));
    }
}

pub fn request(
    spec: &MethodSpec,
    params: &mut Map<String, Value>,
    body: &mut Option<Value>,
    mut plan: Plan,
    mut fetch: impl FnMut(&Map<String, Value>, Option<&Value>) -> Result<Value>,
    emit: impl FnMut(&Value, bool) -> Result<()>,
) -> Result<()> {
    if spec
        .unpaginated_when
        .iter()
        .any(|key| params.contains_key(key))
    {
        plan.paginated = false;
        plan.page_size = None;
    }
    if spec.body_parameters.contains_key("pageToken") {
        if let Some(Value::String(fields)) = params.get_mut("fields")
            && !fields
                .split(',')
                .any(|field| matches!(field.trim(), "*" | "nextPageToken"))
        {
            fields.push_str(",nextPageToken");
        }
        let body = body
            .get_or_insert_with(|| serde_json::json!({}))
            .as_object_mut()
            .context("pagination requires an object body")?;
        run(
            body,
            plan,
            |page| fetch(params, Some(&Value::Object(page.clone()))),
            emit,
        )
    } else {
        run(params, plan, |page| fetch(page, body.as_ref()), emit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn specification() -> MethodSpec {
        serde_json::from_value(json!({"id":"test.items.list","httpMethod":"GET","path":"items","parameters":{"pageToken":{"type":"string","location":"query"},"pageSize":{"type":"integer","location":"query","maximum":"2"}}})).unwrap()
    }

    #[test]
    fn id_lookups_apply_client_limits_without_sending_unsupported_pagination() {
        let mut spec = specification();
        spec.unpaginated_when = vec!["id".into()];
        let mut parameters = json!({"id":["first","second"],"fields":"items(id)"})
            .as_object()
            .unwrap()
            .clone();
        let plan = Plan::new(&spec, true, Some(1), None).unwrap();
        let mut calls = 0;
        request(
            &spec,
            &mut parameters,
            &mut None,
            plan,
            |parameters, _| {
                calls += 1;
                assert!(!parameters.contains_key("pageSize"));
                assert!(!parameters.contains_key("pageToken"));
                assert_eq!(parameters["fields"], "items(id)");
                Ok(json!({"items":[{"id":"first"},{"id":"second"}],"nextPageToken":"unused"}))
            },
            |value, paginated| {
                assert!(!paginated);
                assert_eq!(value["items"], json!([{"id":"first"}]));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(calls, 1);
    }

    #[test]
    fn post_searches_put_pagination_in_the_body_and_preserve_query_fields() {
        let spec:MethodSpec=serde_json::from_value(json!({"id":"photoslibrary.mediaItems.search","httpMethod":"POST","path":"v1/mediaItems:search","x-read-only":true,"x-body-parameters":{"pageToken":{"type":"string"},"pageSize":{"type":"integer","maximum":"2"}}})).unwrap();
        let mut query = json!({"fields":"mediaItems(id)"})
            .as_object()
            .unwrap()
            .clone();
        let mut body = Some(json!({"albumId":"album"}));
        let plan = Plan::new(&spec, true, Some(3), None).unwrap();
        let mut calls = 0;
        let mut count = 0;
        request(
            &spec,
            &mut query,
            &mut body,
            plan,
            |query, body| {
                calls += 1;
                assert!(!query.contains_key("pageToken"));
                assert_eq!(query["fields"], "mediaItems(id),nextPageToken");
                let body = body.unwrap();
                assert_eq!(body["albumId"], "album");
                assert_eq!(body["pageSize"], if calls == 1 { 2 } else { 1 });
                if calls == 2 {
                    assert_eq!(body["pageToken"], "next");
                }
                Ok(json!({"mediaItems":[{"id":"one"},{"id":"two"}],"nextPageToken":"next"}))
            },
            |value, _| {
                count += value["mediaItems"].as_array().unwrap().len();
                Ok(())
            },
        )
        .unwrap();
        assert_eq!((calls, count), (2, 3));
    }

    #[test]
    fn total_limits_do_not_violate_an_apis_minimum_page_size() {
        let mut spec = specification();
        spec.parameters.get_mut("pageSize").unwrap().minimum = Some("200".into());
        spec.parameters.get_mut("pageSize").unwrap().maximum = Some("2000".into());
        run(
            &mut Map::new(),
            Plan::new(&spec, true, Some(1), None).unwrap(),
            |params| {
                assert_eq!(params["pageSize"], 200);
                Ok(json!({"items":[{"id":1},{"id":2}]}))
            },
            |value, _| {
                assert_eq!(value["items"].as_array().unwrap().len(), 1);
                Ok(())
            },
        )
        .unwrap();
    }

    #[test]
    fn no_limit_fetches_all_pages_including_more_than_one_hundred() {
        let plan = Plan::new(&specification(), true, None, None).unwrap();
        let mut calls = 0;
        let mut results = 0;
        run(
            &mut Map::new(),
            plan,
            |params| {
                calls += 1;
                assert_eq!(
                    params
                        .get("pageToken")
                        .and_then(Value::as_str)
                        .unwrap_or("0"),
                    (calls - 1).to_string()
                );
                Ok(if calls == 115 {
                    json!({"items":[{"id":calls}]})
                } else {
                    json!({"items":[{"id":calls}],"nextPageToken":calls.to_string()})
                })
            },
            |value, paginated| {
                assert!(paginated);
                results += value["items"].as_array().unwrap().len();
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(calls, 115);
        assert_eq!(results, 115);
    }

    #[test]
    fn maximum_limits_total_rows_and_requests_only_the_remaining_page_size() {
        let plan = Plan::new(&specification(), true, Some(3), None).unwrap();
        let mut calls = 0;
        let mut output = Vec::new();
        run(&mut Map::new(), plan, |params| {
            calls += 1;
            assert_eq!(params["pageSize"], if calls == 1 { 2 } else { 1 });
            Ok(json!({"items":[{"id":calls * 2 - 1},{"id":calls * 2}],"nextPageToken":calls.to_string()}))
        }, |value, _| { output.extend(value["items"].as_array().unwrap().iter().cloned()); Ok(()) }).unwrap();
        assert_eq!(calls, 2);
        assert_eq!(
            output,
            vec![json!({"id":1}), json!({"id":2}), json!({"id":3})]
        );
    }

    #[test]
    fn explicit_page_limit_stops_successfully_and_preserves_pagination_fields() {
        let plan = Plan::new(&specification(), true, None, Some(2)).unwrap();
        let mut params = json!({"fields":"items(id)"}).as_object().unwrap().clone();
        let mut calls = 0;
        run(
            &mut params,
            plan,
            |params| {
                assert_eq!(params["fields"], "items(id),nextPageToken");
                calls += 1;
                Ok(json!({"items":[{"id":calls}],"nextPageToken":calls.to_string()}))
            },
            |_, _| Ok(()),
        )
        .unwrap();
        assert_eq!(calls, 2);
    }

    #[test]
    fn maximum_also_limits_single_page_lists_and_handles_empty_pages() {
        let mut spec = specification();
        spec.parameters.clear();
        let plan = Plan::new(&spec, true, Some(1), None).unwrap();
        run(
            &mut Map::new(),
            plan,
            |_| Ok(json!({"labels":[{"id":"a"},{"id":"b"}]})),
            |value, paginated| {
                assert!(!paginated);
                assert_eq!(value["labels"], json!([{"id":"a"}]));
                Ok(())
            },
        )
        .unwrap();
        let plan = Plan::new(&specification(), true, Some(1), None).unwrap();
        let mut calls = 0;
        run(
            &mut Map::new(),
            plan,
            |_| {
                calls += 1;
                Ok(if calls == 1 {
                    json!({"nextPageToken":"next"})
                } else {
                    json!({"items":[{"id":"a"}]})
                })
            },
            |_, _| Ok(()),
        )
        .unwrap();
        assert_eq!(calls, 2);
    }

    #[test]
    fn repeated_or_oversized_tokens_fail_instead_of_looping() {
        let mut calls = 0;
        let error = run(
            &mut Map::new(),
            Plan::new(&specification(), true, None, None).unwrap(),
            |_| {
                calls += 1;
                Ok(json!({"items":[],"nextPageToken":"repeat"}))
            },
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert_eq!(calls, 2);
        assert!(error.to_string().contains("repeated page token"));
        assert!(
            run(
                &mut Map::new(),
                Plan::new(&specification(), true, None, None).unwrap(),
                |_| Ok(json!({"items":[],"nextPageToken":"x".repeat(8193)})),
                |_, _| Ok(())
            )
            .is_err()
        );
    }

    #[test]
    fn writes_never_follow_page_tokens_and_limits_require_lists() {
        let mut spec = specification();
        spec.id = "test.items.create".into();
        assert!(Plan::new(&spec, false, Some(5), None).is_err());
        assert!(Plan::new(&spec, false, None, Some(2)).is_err());
        let plan = Plan::new(&spec, false, None, None).unwrap();
        let mut calls = 0;
        run(
            &mut Map::new(),
            plan,
            |_| {
                calls += 1;
                Ok(json!({"nextPageToken":"unexpected"}))
            },
            |_, paginated| {
                assert!(!paginated);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(calls, 1);
    }
}
