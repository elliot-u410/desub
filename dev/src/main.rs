use core_v14::{Decoder, Metadata, Value};

use primitives::crypto::{Ss58AddressFormat, Ss58Codec};
use serde::{
	ser::{self, SerializeSeq},
	Serializer,
};
use std::convert::TryFrom;
//
// // Utility function to serialize from slice/vec to hex
// // If the SubstrateType is a collection of u8s, will serialize as hex
// pub fn as_hex<S: Serializer>(elements: &[SubstrateType], serializer: S) -> Result<S::Ok, S::Error> {
// 	if elements.iter().any(|ty| !matches!(ty, SubstrateType::U8(_))) {
// 		let mut seq = serializer.serialize_seq(Some(elements.len()))?;
// 		for e in elements.iter() {
// 			seq.serialize_element(&e)?;
// 		}
// 		seq.end()
// 	} else {
// 		let bytes = elements
// 			.iter()
// 			.map(|v| match v {
// 				SubstrateType::U8(byte) => *byte,
// 				_ => unreachable!(),
// 			})
// 			.collect::<Vec<u8>>();
// 		let mut hex_str = String::from("0x");
// 		hex_str.push_str(&hex::encode(bytes.as_slice()));
// 		serializer.serialize_str(&hex_str)
// 	}
// }
//
// /// Serialize a Substrate Type as a ss58 Address
// /// # Panics
// /// Panics if a SubstrateType can not be serialized into an ss58 address type
// pub fn as_substrate_address<S: Serializer>(ty: &SubstrateType, serializer: S) -> Result<S::Ok, S::Error> {
// 	match ty {
// 		SubstrateType::Composite(_) => {
// 			let bytes: Vec<u8> = TryFrom::try_from(ty).map_err(|err: Error| ser::Error::custom(err.to_string()))?;
// 			if bytes.len() != 32 {
// 				return Err(ser::Error::custom("address length is incorrect".to_string()));
// 			}
// 			let mut addr: [u8; 32] = Default::default();
// 			for (i, b) in bytes.into_iter().enumerate() {
// 				addr[i] = b;
// 			}
// 			let addr = primitives::crypto::AccountId32::from(addr)
// 				.to_ss58check_with_version(Ss58AddressFormat::SubstrateAccount);
// 			serializer.serialize_str(&addr)
// 		}
// 		SubstrateType::Address(v) => match v {
// 			runtime_primitives::MultiAddress::Id(ref i) => {
// 				let addr = i.to_ss58check_with_version(Ss58AddressFormat::SubstrateAccount);
// 				serializer.serialize_str(&addr)
// 			}
// 			runtime_primitives::MultiAddress::Index(i) => serializer.serialize_str(&format!("{}", i)),
// 			runtime_primitives::MultiAddress::Raw(bytes) => serializer.serialize_str(&format!("{:?}", bytes)),
// 			runtime_primitives::MultiAddress::Address32(ary) => serializer.serialize_str(&format!("{:?}", ary)),
// 			runtime_primitives::MultiAddress::Address20(ary) => serializer.serialize_str(&format!("{:?}", ary)),
// 		},
// 		_ => Err(ser::Error::custom(format!("Could not format {:?} as Ss58 Address", ty))),
// 	}
// }


static V14_METADATA_POLKADOT_SCALE: &'static [u8] = include_bytes!("data/v14_metadata_polkadot.scale");

fn main() {
	let metadata = Metadata::from_bytes(V14_METADATA_POLKADOT_SCALE).unwrap();
	let decoder = Decoder::with_metadata(metadata);

	// let unbond_call_data = "0x1d004a75651fc8e74c0a4f9684afda5d4cfb054e63a8c5aae101610988b8f58a200b0007020fa0d116fb07a015";

	// add '04' prefix to unsiged call data
	let chill_extrinsic_hex =
		"0x041d004a75651fc8e74c0a4f9684afda5d4cfb054e63a8c5aae101610988b8f58a200b0007020fa0d116fb07a015";

	// perform jedi mind tricks
	let chill_extrinsic_bytes = hex::decode(chill_extrinsic_hex.strip_prefix("0x").unwrap()).unwrap();
	let chill_extrinsic = decoder.decode_unwrapped_extrinsic(&chill_extrinsic_bytes).unwrap();

	println!("{:?}", chill_extrinsic.pallet);
	println!("{:?}", chill_extrinsic.call);
	// --> "Proxy"
	// --> "proxy"
	let address_bz = chill_extrinsic.arguments[2].clone();



	match address_bz {
		Value::Composite(val) => {},
		Value::Variant(val) => {},
		Value::Primitive(val) => {},
		Value::BitSequence(val) => {
			println!("{:?}", val);
			let mut hex_str = String::from("0x");
			hex_str.push_str(&hex::encode(&val));
		},
	}


	println!("{:x?}", chill_extrinsic.arguments[0]);
	println!("{:?}", chill_extrinsic.arguments[1]);
	// println!("{:?}", hex_str);

	// TODO: figure out how to decode extrinsic arguments such that the proxy
	// call data can be further decoded. The below does not work.
	// let proxied_call = chill_extrinsic.arguments[0];
	// let hex_encoded = hex::encode(proxied_call.unwrap());
	// println!("{:?}", Debug::fmt(&proxied_call, "{}"));
	// println!("{:?}", chill_extrinsic.arguments[0].value());
}
