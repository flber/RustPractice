#[derive(Debug)]
pub struct Width(pub u16);
#[derive(Debug)]
pub struct Height(pub u16);

fn main() {
	let size = terminal_size();
	if let Some((Width(w), Height(h))) = size {
	    println!("Your terminal is {} cols wide and {} lines tall", w, h);
	} else {
	    println!("Unable to get terminal size");
	}
}

/// Compare with the output of `stty size`
fn terminal_size() -> Option<(Width, Height)> {
    use std::process::Command;
    use std::process::Stdio;

    let output = Command::new("stty")
	    .arg("size")
	    .arg("-F")
	    .arg("/dev/stderr")
	    .stderr(Stdio::inherit())
	    .output()
	    .unwrap();
	
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !output.status.success() {
    	return None;
    }
    
    // stdout is "rows cols"
    let mut data = stdout.split_whitespace();
    let cols = u16::from_str_radix(data.next().unwrap(), 10).unwrap();
    let rows = u16::from_str_radix(data.next().unwrap(), 10).unwrap();
    Some((Width(rows), Height(cols)))
}
