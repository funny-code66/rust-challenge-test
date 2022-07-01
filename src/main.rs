use std::io::{stdin,stdout,Write};

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}
  
fn main() {
    println!("Please enter number of meetings: ");
    read!(x as u32);
	let mut aa = Vec::new();
	let mut bb = Vec::new();
	for i in 0..x {
		read_vec!(v as u32); // Reads space separated integers and stops when newline is encountered.
		println!("{:?}", v);
		aa.push(v[0]);
		bb.push(v[1]);
	}
	let mut min = 0;
	for i in 0..24 {
		let mut mins = 0;
		for j in 0..x {
			if aa[j as usize] <= i && bb[j as usize] >= i {
				mins += 1;
			}
		}
		if min < mins {
			min = mins;
		}
	}
	println!("{}", min);
}
