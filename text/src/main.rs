use core::ops::Range;

fn main() {
	//                                 111111111122222222223333
	//                       0123456789012345678901234567890123
	let test = String::from("[ h1 | This is a [ em | header ] ]");
	let content: String = parse(&test, 0, Vec::<String>::new());

    println!("new: {}", content);
}

fn parse(s: &String, i: usize, mut elems: Vec<String>) -> String {
	let mut t = s.clone();
	if i == len(&t) {
		return t.to_string();
	} else {
		let char = &slice(&t, i..i+1)[..];
		match char {
			"[" => {
				println!("[: ({}) {}", char, t);
				t = remove(&t, i, 1);
				t = insert(&t, i, "<");

				let next = first_from(&t, i+1).1;
				t = remove(&t, i+1, next);
				let mut j = i;
				let elem = slice(&t, i+1..loop {
					let check = slice(&t, j..j+1);
					if check == "," || check == " " || check == "|" {
						break j;
					}
					j += 1;
				});
				elems.push(elem);
			},
			"|" => {
				println!("|: ({}) {}", char, t);
				t = remove(&t, i, 1);
				t = insert(&t, i, ">");
			}
			"]" => {
				println!("]: ({}) {}", char, t);
				t = remove(&t, i, 1);
				let elem = match elems.pop() {
					Some(e) => e,
					None => String::from(""),
				};
				let end_tag = &format!("</{}>", elem);
				t = insert(&t, i, end_tag);
			}
			_ => {
				println!("_: ({}) {}", char, t);
				let next = first_from(&t, i+1);
				match &next.0[..] {
					"|" => {
						println!("_|: ({}) {}", next.0, t);
						t = remove(&t, i+1, next.1);
					},
					_ => (),
				}
			},
		}
		parse(&t, i+1, elems)
	}
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

/*
inserts str into string, preserving graphemes
*/
pub fn insert(s: &String, idx: usize, ins: &str) -> String {
    assert!(idx <= len(&s), "the index was larger than the target slice");
    let ins_len = len(&ins.to_string());
    let fin_len = len(&s) + ins_len;
    let mut r = String::with_capacity(fin_len);
    for i in 0..fin_len {
        if i < idx {
            r.push_str(&slice(&s, i..i + 1));
        } else if i < idx + ins_len {
            let i_ins = i - idx;
            r.push_str(&slice(&ins.to_string(), i_ins..i_ins + 1));
        } else {
            let a_ins = i - ins_len;
            r.push_str(&slice(&s, a_ins..a_ins + 1));
        }
    }
    r
}
