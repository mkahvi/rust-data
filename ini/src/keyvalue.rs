use std::sync::Arc;
use crate::constants;

pub struct KeyValue<'ini, 'parent>
{
	parent: Option<Arc<&'parent super::Section>>,
	key: String,
	value: ValueType,
	escaped_value: String,
	comment: String,

	dirty: bool,

	config: &'ini super::Config
}

pub enum ValueType {
	None,
	String(String),
	StringArray(Vec<String>),
	Integer(i64),
	IntegerArray(Vec<i64>),
	Float(f64),
	FloatArray(Vec<f64>)
}

impl<'ini, 'parent> KeyValue<'ini, 'parent>
{
	/*
	pub fn new() -> Self
	{
		let &'ini cfg = super::Config::new();
		Self {
			parent: Option::None,
			key: String::new(),
			value: ValueType::None,
			escaped_value:String::new(),
			comment: String::new(),
			dirty: false,
			config: &cfg
		}
	}
	*/

	pub fn new_simple(key: String, config: &'ini super::Config) -> Self
	{
		Self { parent: Option::None, key, value: ValueType::None, escaped_value: String::new(), comment: String::new(), dirty: false, config }
	}

	pub fn new_full(parent: Option<Arc<&'parent super::Section>>, key: String, value: String, comment: String, config: &'ini super::Config) -> Self
	{
		let mut t = Self { parent, key, value: ValueType::String(value), escaped_value: String::new(), comment, dirty: true, config };
		t.update_escaped_value();
		t
	}

	// - - - - - Set basic values - - - - -

	pub fn set_string(&mut self, value: String)
	{
		self.value = ValueType::String(value);
		self.dirty = true;
	}

	pub fn set_integer(&mut self, value: i64)
	{
		self.value = ValueType::Integer(value);
		self.dirty = true;
	}

	pub fn set_float(&mut self, value: f64)
	{
		self.value = ValueType::Float(value);
		self.dirty = false;
	}

	// - - - - - Whatever these are - - - - -

	pub fn with_i64(&mut self, value: i64) -> &Self
	{
		self.set_integer(value);
		self
	}

	pub fn with_f64(&mut self, value: f64) -> &Self
	{
		self.set_float(value);
		self
	}

	pub fn with_string(&mut self, value: String) -> &Self
	{
		self.set_string(value);
		self
	}

	// - - - - - Set array values - - - - -

	pub fn set_string_array(&mut self, value: Vec<String>)
	{
		self.value = ValueType::StringArray(value);
		self.dirty = true;
	}

	pub fn set_integer_array(&mut self, value: Vec<i64>)
	{
		self.value = ValueType::IntegerArray(value);
		self.dirty = true;
	}

	pub fn set_float_array(&mut self, value: Vec<f64>)
	{
		self.value = ValueType::FloatArray(value);
		self.dirty = true;
	}

	// - - - - - Whatever these are, they don't work correctly. - - - - -

	pub fn with_string_array(&mut self, value: Vec<String>) -> &Self
	{
		self.set_string_array(value);
		self
	}

	pub fn with_i64_array(&mut self, value: Vec<i64>) -> &Self
	{
		self.set_integer_array(value);
		self
	}

	pub fn with_f64_array(&mut self, value: Vec<f64>) -> &Self
	{
		self.set_float_array(value);
		self
	}

	// - - - - - Get basic values - - - - -

	pub fn get_string(&self) -> Result<&String, &'static str> {
		match &self.value {
			ValueType::String(string) => Ok(&string),
			_ => Err("type mismatch; not a string"),
		}
	}

	pub fn get_integer(&self) -> Result<i64, &'static str> {
		match self.value {
			ValueType::Integer(integer) => Ok(integer),
			_ => Err("type mismatch; not an integer")
		}
	}

	pub fn get_float(&self) -> Result<f64, &'static str> {
		match self.value {
			ValueType::Float(float) => Ok(float),
			_ => Err("type mismatch; not a float")
		}
	}

	// - - - - - Get array values - - - - -

	pub fn get_string_array(&self) -> Result<&Vec<String>, &'static str>
	{
		match &self.value {
			ValueType::StringArray(array) => Ok(&array),
			_ => Err("type mismatch; not a string array")
		}
	}

	pub fn get_integer_array(&self) -> Result<&Vec<i64>, &'static str>
	{
		match &self.value {
			ValueType::IntegerArray(array) => Ok(&array),
			_ => Err("type mismatch; not an integer array")
		}
	}

	pub fn get_float_array(&self) -> Result<&Vec<f64>, &'static str>
	{
		match &self.value {
			ValueType::FloatArray(array) => Ok(&array),
			_ => Err("type mismatch; not a float array")
		}
	}

	// - - - - - Others - - - - -

	pub fn set_config(&mut self, config: &'ini super::Config)
	{
		self.config = config;
	}

	pub fn set_comment(&mut self, comment: String) { self.comment = comment; }
	pub fn get_comment(&self) -> &str { &self.comment }

	pub fn set_parent(&mut self, parent: Option<Arc<&'parent super::Section>>) { self.parent = parent; }
	pub fn get_parent(&self) -> &Option<Arc<&'parent super::Section>> { &self.parent }

	pub fn comment_only(&self) -> bool { self.key.is_empty() }

	fn escape_string(&self, string: &String) -> String
	{
		let needs_escaping = string.find(constants::QUOTE).is_some();
		let lookup: &[_] = &constants::RESERVED_CHARACTERS;
		let excess_whitespace = string.starts_with(' ') || string.ends_with(' ');
		let needs_quotes = string.find(lookup).is_some() || excess_whitespace;

		let mut ns = String::from(string);

		// TODO: Escape comment characters, too.
		if needs_escaping { ns = ns.replace("\"", "\\\""); }
		if needs_quotes { ns = format!("\"{}\"", ns); }

		ns
	}

	fn escape_string_array(&self, slice: &[String]) -> Vec<String>
	{
		slice.iter().map(|x| self.escape_string(x)).collect()
	}

	/// Updates escaped_value.
	fn update_escaped_value(&mut self)
	{
		match &self.value {
		    ValueType::String(value) => { self.escaped_value = self.escape_string(value); },
		    ValueType::StringArray(array) => {
				let vec = self.escape_string_array(&array[..]);
				self.escaped_value = format!("[ {} ]", vec.join(", "));
			},
		    ValueType::Integer(value) => { self.escaped_value = format!("{}", value); },
		    ValueType::IntegerArray(array) => {
				let vec = vec_to_string(&array[..]);
				self.escaped_value = format!("[ {} ]", vec.join(", "));
			},
		    ValueType::Float(value) => { self.escaped_value = format!("{}", value); },
			ValueType::FloatArray(array) => {
				let vec = vec_to_string(&array[..]);
				self.escaped_value = format!("[ {} ]", vec.join(", "));
			},
			_ => { self.escaped_value = String::new(); }
		};

		self.dirty = false;
	}

	pub fn update(&mut self)
	{
		if self.dirty { self.update_escaped_value(); }
	}
}

impl mkah_traits::Empty for KeyValue<'_, '_>
{
	fn is_empty(&self) -> bool
	{
		self.escaped_value.is_empty() && self.comment.is_empty()
	}

	fn empty(&mut self)
	{
		self.escaped_value = String::new();
		self.comment = String::new();
    }
}

impl ToString for KeyValue<'_, '_>
{
	fn to_string(&self) -> String
	{
		if self.dirty { println!("KeyValue::to_string() called without update()"); }

		if self.comment_only() { return format!("# {}", self.comment); } // ugh
		if self.comment.is_empty() { return format!("{} = {}", self.key, self.escaped_value); }
		format!("{} = {} # {}", self.key, self.escaped_value, self.comment)
	}
}

// - - - - - Free floating - - - - -

/// Converts Vec<T> to Vec<String>
pub(crate) fn vec_to_string<T: ToString>(vector: &[T]) -> Vec<String>
{
	vector.iter().map(|v| v.to_string()).collect::<Vec<String>>()
}
