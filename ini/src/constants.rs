pub const SECTION_START: char = '[';
pub const SECTION_END: char = ']';
pub const VALUE_SEPARATOR: char = '=';
pub const COMMENT: char = '#';
pub const COMMENT_ALT: char = ';';
pub const QUOTE: char = '"';

// TODO: Allow channging these?

pub const ARRAY_START: char = '[';
pub const ARRAY_END: char = ']';

/// These characters are disallowed in section and key names.
const RESERVED_CHARACTERS: [char;8] = [QUOTE, COMMENT, COMMENT_ALT, ARRAY_START, ARRAY_END, VALUE_SEPARATOR, SECTION_START, SECTION_END];
