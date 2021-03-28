use core::ops::Range;

fn main() {
	let test = String::from(" This  is		a test.");

	let first = first(&test);
    println!("first: {}@{}", first.0, first.1);
    let first_from = first_from(&test, 5);
    println!("first_from: {}@{}", first_from.0, first_from.1);
    let last_from = last_from(&test, 7);
    println!("last_from: {}@{}", last_from.0, last_from.1);
}

/*
returns the first character in a string, as well as the index of that string
*/
pub fn first(s: &String) -> (String, usize) {
    let line = s.clone();
    let mut num = 0;
    for i in 0..len(&line) {
        let char = slice(&line, i..i + 1);
        if char == " " || char == "\t" {
            num += 1;
        } else {
            return (char, num);
        }
    }
    (String::from(""), num)
}

/*
returns the first character in a string from an index, as well as the index of that string
*/
pub fn first_from(s: &String, i: usize) -> (String, usize) {
    let line = s.clone();
    first(&slice(&line, i..len(&line)))
}

/**/
pub fn last_from(s: &String, i: usize) -> (String, usize) {
    for i in (0..i).rev() {
		let char = &slice(&s, i..i+1) as &str;
		match char {
			" " => (),
			"\t" => (),
			_ => {
				return (String::from(char), i);
			},
		}
    }
    (String::from(""), i)
}

/*
returns the length of a String, taking graphemes into account
*/
pub fn len(s: &String) -> usize {
    s.chars().count()
}

/*
returns a slice of a string from a range, utf-8 compliant
*/
pub fn slice(s: &String, r: Range<usize>) -> String {
    let mut sub_string = Vec::<String>::new();
    for (i, c) in s.chars().enumerate() {
        if r.contains(&i) {
            sub_string.push(c.to_string());
        }
    }
    sub_string.join("")
}


/*
removes whitespace around given string from start and end indices
*/
pub fn trim(l: &String, start: usize, end: usize) -> (String, usize, usize) {
    let mut line = l.clone();
    let mut first: usize = 0;
    let mut last: usize = 0;
    let mut hit_text = false;
    for i in (0..len(&line) - end).rev() {
        let next = slice(&line, i..i + 1);
        if !hit_text && (next == " " || next == "\t") {
            line = remove(&line, i, 1);
        } else {
        	first = i;
            hit_text = true;
        }
    }
    hit_text = false;
    let mut i = start;
    while i < len(&line) - end {
        let next = slice(&line, i..i + 1);
        if !hit_text && (next == " " || next == "\t") {
            line = remove(&line, i, 1);
        } else {
            hit_text = true;
            last = i;
            i += 1;
        }
    }
    (line, first, last)
}

/*
removes from String from index with length, preserving graphemes
*/
pub fn remove(s: &String, idx: usize, l: usize) -> String {
    assert!(idx <= len(&s), "the index was larger than the target slice");

    let first = slice(&s, 0..idx);
    let second = slice(&s, idx + l..len(&s));

    [first, second].concat()
}
