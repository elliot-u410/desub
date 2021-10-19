use clap::{App, Arg};
use core_v14::{Decoder, Metadata};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), String> {
	let matches = App::new("Substrate Extrinsic Parser")
		.version("0.1.0")
		.about("Parses substrate extrinsics using a v14+ metadata")
		.arg(
			Arg::with_name("metadata")
				.short("m")
				.long("metadata")
				.takes_value(true)
				.help("Path to file containing v14+ metadata as scale encoded binary")
				.required(true),
		)
		.arg(
			Arg::with_name("extrinsic")
				.short("e")
				.long("extrinsic")
				.takes_value(true)
				.help("Extrinsic in hex format")
				.required(true),
		)
		.get_matches();

	let metadata_path: String = matches.value_of("metadata").expect("metadata argument is required").to_string();
	let extrinsic_raw: String = matches.value_of("extrinsic").expect("extrinsic argument is required").to_string();

	let mut metadata_fh = File::open(metadata_path).map_err(|e| e.to_string())?;
	let mut metadata: Vec<u8> = Vec::new();
	metadata_fh.read_to_end(&mut metadata).map_err(|e| e.to_string())?;

	let metadata = Metadata::from_bytes(&metadata[..]).map_err(|e| e.to_string())?;
	let decoder = Decoder::with_metadata(metadata);

	// add '04' prefix to unsiged call data
	let extrinsic_with_prefix = format!("04{}", extrinsic_raw.strip_prefix("0x").unwrap_or(&extrinsic_raw));

	// perform jedi mind tricks
	let extrinsic_bytes = hex::decode(extrinsic_with_prefix).map_err(|e| e.to_string())?;
	let extrinsic = decoder.decode_unwrapped_extrinsic(&extrinsic_bytes).map_err(|e| e.to_string())?;
	println!("{:?}", extrinsic);
	Ok(())
}
