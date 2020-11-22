use std::env;
use std::io::{BufReader, Result as Res};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use endio::LERead;
use lu_packets::{
	auth::server::Message as AuthServerMessage,
	world::server::Message as WorldServerMessage,
	world::client::Message as WorldClientMessage,
};

static mut PRINT_PACKETS: bool = false;

fn visit_dirs(dir: &Path) -> Res<usize> {
	let mut packet_count = 0;
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			packet_count += if path.is_dir() { visit_dirs(&path) } else { parse(&path) }?;
			println!("packet count = {}", packet_count);
		}
	}
	Ok(packet_count)
}

fn parse(path: &Path) -> Res<usize> {
	if path.extension().unwrap() != "zip" { return Ok(0); }

	let src = BufReader::new(File::open(path).unwrap());
	let mut zip = zip::ZipArchive::new(src).unwrap();
	let mut i = 0;
	let mut packet_count = 0;
	while i < zip.len() {
		let mut file = zip.by_index(i).unwrap();
		if file.name().contains("of") {
			i += 1; continue;
		}
		if file.name().contains("[53-01-") {
			let msg: AuthServerMessage = file.read().expect(&format!("Zip: {}, Filename: {}, {} bytes", path.to_str().unwrap(), file.name(), file.size()));
			if unsafe { PRINT_PACKETS } {
				dbg!(msg);
			}
			packet_count += 1
		} else if file.name().contains("[53-04-")
			&& !file.name().contains("[53-04-00-16]")
			&& !file.name().contains("[30-00]")
			&& !file.name().contains("[e6-00]")
			&& !file.name().contains("[6b-03]")
			&& !file.name().contains("[16-04]")
			&& !file.name().contains("[49-04]")
			&& !file.name().contains("[ad-04]")
			&& !file.name().contains("[1c-05]")
			&& !file.name().contains("[48]")
			&& !file.name().contains("[230]")
			&& !file.name().contains("[875]")
			&& !file.name().contains("[1046]")
			&& !file.name().contains("[1097]")
			&& !file.name().contains("[1197]")
			&& !file.name().contains("[1308]")
		{
			let msg: WorldServerMessage = file.read().expect(&format!("Zip: {}, Filename: {}, {} bytes", path.to_str().unwrap(), file.name(), file.size()));
			if unsafe { PRINT_PACKETS } {
				dbg!(msg);
			};
			packet_count += 1;
		} else if file.name().contains("[53-02-") || (file.name().contains("[53-05-")
		&& !file.name().contains("[53-05-00-00]")
		&& !file.name().contains("[53-05-00-15]")
		&& !file.name().contains("[53-05-00-31]")
		&& !file.name().contains("[76-00]")
		&& !file.name().contains("[e6-00]")
		&& !file.name().contains("[ff-00]")
		&& !file.name().contains("[a1-01]")
		&& !file.name().contains("[7f-02]")
		&& !file.name().contains("[a3-02]")
		&& !file.name().contains("[cc-02]")
		&& !file.name().contains("[35-03]")
		&& !file.name().contains("[36-03]")
		&& !file.name().contains("[4d-03]")
		&& !file.name().contains("[6d-03]")
		&& !file.name().contains("[91-03]")
		&& !file.name().contains("[a0-04]")
		&& !file.name().contains("[1a-05]")
		&& !file.name().contains("[e6-05]")
		&& !file.name().contains("[16-06]")
		&& !file.name().contains("[1c-06]")
		&& !file.name().contains("[6f-06]")
		&& !file.name().contains("[70-06]")
		&& !file.name().contains("[118]")
		&& !file.name().contains("[230]")
		&& !file.name().contains("[255]")
		&& !file.name().contains("[417]")
		&& !file.name().contains("[639]")
		&& !file.name().contains("[675]")
		&& !file.name().contains("[716]")
		&& !file.name().contains("[821]")
		&& !file.name().contains("[822]")
		&& !file.name().contains("[845]")
		&& !file.name().contains("[877]")
		&& !file.name().contains("[913]")
		&& !file.name().contains("[1184]")
		&& !file.name().contains("[1306]")
		&& !file.name().contains("[1510]")
		&& !file.name().contains("[1558]")
		&& !file.name().contains("[1564]")
		&& !file.name().contains("[1647]")
		&& !file.name().contains("[1648]"))
		{
			let msg: WorldClientMessage = file.read().expect(&format!("Zip: {}, Filename: {}, {} bytes", path.to_str().unwrap(), file.name(), file.size()));
			if unsafe { PRINT_PACKETS } {
				dbg!(msg);
			}
			packet_count += 1;
		} else { i += 1; continue }
		// assert fully read
		let mut rest = vec![];
		std::io::Read::read_to_end(&mut file, &mut rest).unwrap();
		assert_eq!(rest, vec![], "{}", path.to_str().unwrap());
		i += 1;
	}
	Ok(packet_count)
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let capture = match args.get(1) {
		Some(x) => fs::canonicalize(x).unwrap(),
		None => {
			println!("Usage: capture_parser capture_path --print_packets");
			return;
		}
	};
	unsafe { PRINT_PACKETS = args.get(2).is_some(); }

	let start = Instant::now();
	let packet_count = if capture.ends_with(".zip") {
		parse(&capture)
	} else {
		visit_dirs(&capture)
	}.unwrap();
	println!();
	println!("Number of parsed packets: {}", packet_count);
	println!("Time taken: {:?}", start.elapsed());
}
