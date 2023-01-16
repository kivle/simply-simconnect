use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=c:/MSFS SDK/SimConnect SDK/lib/static");
    println!("cargo:rustc-link-lib=static=SimConnect");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_var("MAX_METAR_LENGTH")
        .allowlist_var("MAX_THERMAL_SIZE")
        .allowlist_var("MAX_THERMAL_RATE")
        .allowlist_var("UNKNOWN_SENDID")
        .allowlist_var("UNKNOWN_INDEX")
        .allowlist_var("UNKNOWN_GROUP")
        .allowlist_var("SIMCONNECT_.*")
        .allowlist_var("S_OK")
        .allowlist_var("E_FAIL")
        .allowlist_type("SIMCONNECT_.*")
        .allowlist_function("SimConnect_.*")
        .blocklist_function("SimConnect_Text")
        .blocklist_function("SimConnect_InsertString")
        .blocklist_function("SimConnect_RetrieveString")
        .blocklist_function("SimConnect_Weather.*")
        .blocklist_item("SIMCONNECT_EXCEPTION_SIMCONNECT_EXCEPTION_WEATHER_.*") // TODO: why does this not work?!
        .blocklist_item("SIMCONNECT_CLOUD_STATE.*")
        .blocklist_item("SIMCONNECT_TEXT_RESULT.*")
        .blocklist_item("SIMCONNECT_TEXT_TYPE.*")
        .blocklist_item("SIMCONNECT_MISSION.*")
        .blocklist_item("SIMCONNECT_WEATHER.*")
        .impl_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
