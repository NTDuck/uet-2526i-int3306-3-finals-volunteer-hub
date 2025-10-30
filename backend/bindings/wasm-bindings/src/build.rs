// An alternative approach where a `.rs` file is generated from a `.env` file
// https://dev.to/javiasilis/how-to-pass-environment-variables-to-a-rust-wasm-application-like-yew-dioxus-and-leptos-as-a-typescript-developer-ond	
pub fn main() -> ::aliases::result::Fallible {
	self::env::load(::core::concat!(::core::env!("CARGO_WORKSPACE_DIR"), ".env"))?;

	::aliases::result::Fallible::Ok(())
}

pub mod env {
	pub fn load<Path: ::core::convert::AsRef<::std::path::Path>>(path: Path) -> ::aliases::result::Fallible {
		::std::println!("cargo:rerun-if-changed={}", path.as_ref().display());

		::std::fs::read_to_string(path)?.lines()
			.map(|line| line.trim())
			.filter(|line| !line.is_empty() && !line.starts_with('#'))
			.filter_map(|line| line.split_once('='))
			.for_each(|(key, value)| ::std::println!("cargo:rustc-env={}={}", key.trim(), value.trim()));

		::aliases::result::Fallible::Ok(())
	}
}
