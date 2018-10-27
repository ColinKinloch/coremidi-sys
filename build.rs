extern crate bindgen;

use std::env;
use std::path::PathBuf;

const MIDI_MATCH: &str = "MIDI.*";
const KMIDI_MATCH: &str = "kMIDI.*";

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn osx_include_path() -> Result<String, std::io::Error> {
    use std::process::Command;

    let output = Command::new("xcode-select").arg("-p").output()?.stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcode-select`");
    let prefix = prefix_str.trim_right();

    let platform = if cfg!(target_os = "macos") {
        "MacOSX"
    } else if cfg!(target_os = "ios") {
        "iPhoneOS"
    } else {
        unreachable!();
    };

    let infix = if prefix == "/Library/Developer/CommandLineTools" {
        format!("SDKs/{}.sdk", platform)
    } else {
        format!("Platforms/{}.platform/Developer/SDKs/{}.sdk", platform, platform)
    };

    let directory = format!("{}/{}", prefix, infix);

    Ok(directory)
}

fn main() {
    let sdk_path =  osx_include_path().unwrap();
    let cm_header_path = format!("{}/System/Library/Frameworks/CoreMIDI.framework/Versions/Current/Headers", sdk_path);
    
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let bindings = {
        let builder = bindgen::Builder::default()
            .header(format!("{}/CoreMIDI.h", cm_header_path))
            .derive_default(true)
            .whitelist_type(MIDI_MATCH)
            .whitelist_function(MIDI_MATCH)
            .whitelist_var(MIDI_MATCH)
            .whitelist_type(KMIDI_MATCH)
            .whitelist_function(KMIDI_MATCH)
            .whitelist_var(KMIDI_MATCH)
            // .objc_extern_crate(true)
            // .generate_comments(false)
            .constified_enum(".*")
            .blacklist_type("(__)?CF.*")
            .clang_args(&[
                    "-F", &format!("{}/System/Library/Frameworks", sdk_path),
                    "-I", &format!("{}/usr/include", sdk_path)]);
        
        builder.generate()
            .expect("Unable to generate bindings")
    };
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=framework=CoreMIDI");
}
