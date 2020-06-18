use crate::config::*;
use crate::keyvalue::*;

#[test]
fn to_string()
{
	let cfg = Config::new();
	let kv = KeyValue::new_full(Option::None, String::from("key0"), String::from("TestValue"), String::new(), &cfg);
	let expected = "key0 = TestValue";
	let result = kv.to_string();
	assert!(result == expected, format!("\nexpected: {}\nresult: {}\n", expected, result));
}

#[test]
fn comment_to_string()
{
	let cfg = Config::new();
	let kv = KeyValue::new_full(Option::None, String::new(), String::new(), "test".to_string(), &cfg);
	let expected = "# test";
	let result = kv.to_string();
	assert!(result == expected, format!("\nexpected: {}\nresult: {}\n", expected, result));
}

#[test]
fn full_to_string()
{
	let cfg = Config::new();
	let kv = KeyValue::new_full(Option::None, String::from("key1"), String::from("7est4alue"), "tset".to_string(), &cfg);
	let expected = "key1 = 7est4alue # tset";
	let result = kv.to_string();
	assert!(result == expected, format!("\nexpected: {}\nresult: {}\n", expected, result));
}

#[test]
fn vec_convert_i64()
{
	let x = vec![1,2,3];
	let y = vec_to_string(&x[..]);
	assert!(y == ["1","2","3"]);
}

#[test]
fn escaped_array()
{
	let cfg = Config::new();
	let testarray = vec![String::from(r#""zoit"#),String::from("bim#"),String::from(" bazah=]")];
	let mut kv = KeyValue::new_simple(String::from("test"), &cfg);
	kv.with_string_array(testarray);
	kv.update();
	let result = kv.to_string();
	let expected = r#"test = [ "\"zoit", "bim#", " bazah=]" ]"#;
	assert!(result == expected, format!("\nexpected: {}\nresult: {}\n", expected, result));
}
