use apiai::lang::Language;
use std::collections::HashMap;
use std::option::Option;
use apiai::client::{ApiQuery, ApiRequest, ApiEvent};

extern crate serde_json;

/**
* Simple test to make sure that languages serialize predictably into JSON strings
*/
#[test]
fn test_serialize_language_json() {

   let english = Language::English;

   assert!(String::from("\"en\"") == serde_json::to_string(&english).unwrap());
}

/**
* Dummy object that contains only a Language object to test serialization
*/
#[derive(Deserialize)]
struct LangObject{
    pub lang: Language
}

/**
* Test that deserializing a language from its locale string (e.g. "en") works ok.
*
*/
#[test]
fn test_deserialize_language() {

   let en_str : &'static str = "{\"lang\":\"en\"}";

   let langmap : LangObject = serde_json::from_str(en_str).unwrap();

   assert!(langmap.lang == Language::English);
}

/**
* Test that creating an Api Query object works as expected
*
*/
#[test]
fn test_serialize_apiquery_query_json() {
    let q = ApiQuery::Query(String::from("hello moto"));
    let query_string = "{\"query\":\"hello moto\"}";

    assert!(String::from(query_string) == serde_json::to_string(&q).unwrap());
}

/**
* Test that creating an Api Event object with no 'data' args works as expected
*
*/
#[test]
fn test_serialize_apiquery_event_no_args() {
    let e = ApiQuery::Event(ApiEvent{name: String::from("Welcome"), data: Option::None});
    let event_string = "{\"event\":{\"name\":\"Welcome\",\"data\":null}}";

    assert!(String::from(event_string) == serde_json::to_string(&e).unwrap());
}

/**
* Test that creating an Api Event object with 'data' args works as expected
*
*/
#[test]
fn test_serialize_apiquery_event_with_args_json() {
    let mut data = HashMap::new();
    data.insert(String::from("client"), String::from("Slack"));

    let e = ApiQuery::Event(ApiEvent{
        name: String::from("Welcome"),
        data: Option::Some(data)
    });

    let event_string = "{\"event\":{\"name\":\"Welcome\",\"data\":{\"client\":\"Slack\"}}}";


    assert!(String::from(event_string) == serde_json::to_string(&e).unwrap());
}


/**
* Test that  deserializing an event with args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_event_with_args() {

    let event_string = "{\"event\":{\"name\":\"Welcome\",\"data\":{\"client\":\"Slack\"}}}";

    let query : ApiQuery = serde_json::from_str(event_string).unwrap();

    match query {
        ApiQuery::Event(evt) => {

            assert!(evt.name == String::from("Welcome"));

            assert!(evt.data.unwrap().get("client").unwrap() == "Slack");

        }
        _ => panic!("Expected to find event, not query or anything else")
    }
}

/**
* Test that  deserializing an event with no args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_event_without_args() {

    let event_string = "{\"event\":{\"name\":\"Welcome\"}}";

    let query : ApiQuery = serde_json::from_str(event_string).unwrap();

    match query {
        ApiQuery::Event(evt) => {

            assert!(evt.name == String::from("Welcome"));

            assert!(evt.data.is_none());

        }
        _ => panic!("Expected to find event, not query or anything else")
    }
}

/**
* Test that  deserializing an event with no args works as expected
*
*/
#[test]
fn test_deserialize_apiquery_query() {

    let query_string = "{\"query\":\"hello moto\"}";

    let query : ApiQuery = serde_json::from_str(query_string).unwrap();

    match query {
        ApiQuery::Query(q) => {

            assert!(q == String::from("hello moto"));

        }
        _ => panic!("Expected to find event, not query or anything else")
    }
}


/**
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_serialize_apirequest_query(){

    let query_string = r#"{"query":"hello moto","sessionId":"12345","lang":"en","contexts":[]}"#;

    let q = ApiQuery::Query(String::from("hello moto"));

    let req = ApiRequest{
        query:q,
        session_id: String::from("12345"),
        lang: Language::English,
        contexts: Vec::new()
    };

    assert_eq!(query_string, serde_json::to_string(&req).unwrap());
}


/**
* Test that deserializing an event with no args works as expected
*
*/
#[test]
fn test_serialize_apirequest_event_no_data(){

    let query_string = r#"{"event":{"name":"Welcome","data":null},"sessionId":"12345","lang":"en","contexts":[]}"#;

    let e = ApiQuery::Event(ApiEvent{
        name: String::from("Welcome"),
        data: Option::None
    });

    let req = ApiRequest{
        query:e,
        session_id: String::from("12345"),
        lang: Language::English,
        contexts: Vec::new()
    };

    assert_eq!(query_string, serde_json::to_string(&req).unwrap());
}
