pub struct Config
{
	strip_empty_lines: bool,
	pad_sections: bool,
	preserve_whitespace: bool,
	allow_nameless_sections: bool,
	unique_sections: bool,
	unique_keys: bool,
	always_quote_strings: bool,
	require_string_quotes: bool,

	comment_char: char,
	comment_chars: Vec<char>, // consider external crates 'smallvec' and 'arrayvec', or overallocated static array

	line_end: String,

	strict: bool,
}

/// All the getters and setters... why not just make everything pub? Feels redundant beyond belief.
impl Config {
	pub fn get_strip_empty_lines(&self) -> bool { self.strip_empty_lines }
	pub fn set_strip_empty_lines(&mut self, strip: bool) { self.strip_empty_lines = strip; }
	pub fn get_pad_sections(&self) -> bool { self.pad_sections }
	pub fn set_pad_sections(&mut self, pad: bool) { self.pad_sections = pad; }
	pub fn get_preserve_whitespace(&self) -> bool { self.preserve_whitespace }
	pub fn set_preserve_whitespace(&mut self, preserve: bool) { self.preserve_whitespace = preserve; }
	pub fn get_allow_nameless_sections(&self) -> bool { self.allow_nameless_sections }
	pub fn set_allow_nameless_sections(&mut self, allow_nameless: bool) { self.allow_nameless_sections = allow_nameless; }
	pub fn get_unique_sections(&self) -> bool { self.unique_sections }
	pub fn set_unique_sections(&mut self, unique_sections: bool) { self.unique_sections = unique_sections; }
	pub fn get_unique_keys(&self) -> bool { self.unique_keys }
	pub fn set_unique_keys(&mut self, unique_keys: bool) { self.unique_keys = unique_keys; }
	pub fn get_always_quote_strings(&self) -> bool { self.always_quote_strings }
	pub fn set_always_quote_strings(&mut self, always_quote: bool) { self.always_quote_strings = always_quote; }
	pub fn get_require_string_quotes(&self) -> bool { self.require_string_quotes }
	pub fn set_require_string_quotes(&mut self, require_quotes: bool) { self.require_string_quotes = require_quotes; }
	pub fn get_comment_char(&self) -> char { self.comment_char }
	pub fn set_comment_char(&mut self, comment_char: char) { self.comment_char = comment_char; }
	pub fn get_comment_chars(&self) -> &Vec<char> { &self.comment_chars } // &bad idea?
	pub fn set_comment_chars(&mut self, comment_chars: Vec<char>) { self.comment_chars = comment_chars; }
	pub fn get_line_end(&self) -> &String { &self.line_end } // &bad idea?
	pub fn set_line_end(&mut self, line_end: String) { self.line_end = line_end; }
	pub fn get_strict(&self) -> bool { self.strict }
	pub fn set_strict(&mut self, strict: bool) { self.strict = strict }
}
