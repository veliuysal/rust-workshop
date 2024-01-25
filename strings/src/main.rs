fn main() {
    //################ STRING LITERALS ################
    //&str -> std::str
    //let camp:&str="OYK";
    //let camp:&'static str = "OYK";

    //################ STRING OBJECT ################
    //String::new() -> String::from()
    //let empty_string = String::new();
    // let content_string = String::from("OYK");

    //1	new()	pub const fn new() → String	Creates a new empty String.
    // 2	to_string()	fn to_string(&self) → String	Converts the given value to a String.
    // 3	replace()	pub fn replace<'a, P>(&'a self, from: P, to: &str) → String	Replaces all matches of a pattern with another string.
    // 4	as_str()	pub fn as_str(&self) → &str	Extracts a string slice containing the entire string.
    // 5	push()	pub fn push(&mut self, ch: char)	Appends the given char to the end of this String.
    // 6	push_str()	pub fn push_str(&mut self, string: &str)	Appends a given string slice onto the end of this String.
    // 7	len()	pub fn len(&self) → usize	Returns the length of this String, in bytes.
    // 8	trim()	pub fn trim(&self) → &str	Returns a string slice with leading and trailing whitespace removed.
    // 9	split_whitespace()	pub fn split_whitespace(&self) → SplitWhitespace	Splits a string slice by whitespace and returns an iterator.
    // 10	split()	pub fn split<'a, P>(&'a self, pat: P) → Split<'a, P> , where P is pattern can be &str, char, or a closure that determines the split.	Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.
    // 11	chars()	pub fn chars(&self) → Chars	Returns an iterator over the chars of a string slice.

    //################ C STRING LITERALS ################
    //C language Strings
    //cstr::cstr -> std::ffi::CStr
    //let test = cstr!(b"hello\xff");

    //################ USING FORMAT MACRO ################
    //format!("test");                             // => "test"
    // format!("hello {}", "world!");               // => "hello world!"
    // format!("x = {}, y = {val}", 10, val = 30);  // => "x = 10, y = 30"
    // let (x, y) = (1, 2);
    // format!("{x} + {y} = 3");                    // => "1 + 2 = 3"
}
