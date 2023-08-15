extern crate cc;

fn main() {
    let out_dir_str = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir_str);
    let llhttp_c_dir = std::env::current_dir().unwrap().join("..").join("llhttp_c");
    let llhttp_native_c_dir = std::env::current_dir()
        .unwrap()
        .join("..")
        .join("llhttp")
        .join("src")
        .join("native");

    let llhttp_bindings =
        bindgen::Builder::default().header(llhttp_c_dir.join("llhttp.h").to_str().unwrap());

    #[cfg(target_os = "macos")]
    let llhttp_bindings = llhttp_bindings
        .blocklist_type("^__darwin_.*")
        .blocklist_type("^_opaque_.*");

    llhttp_bindings
        .use_core()
        .ctypes_prefix("::libc")
        .allowlist_var("^llhttp_.*")
        .allowlist_type("^llhttp_.*")
        .allowlist_function("^llhttp_.*")
        .size_t_is_usize(true)
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .newtype_enum("llhttp_errno")
        .newtype_enum("llhttp_flags")
        .newtype_enum("llhttp_lenient_flags")
        .newtype_enum("llhttp_type")
        .newtype_enum("llhttp_method")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("llhttp.rs"))
        .unwrap();

    cc::Build::new()
        .file(llhttp_c_dir.join("llhttp.c"))
        .file(llhttp_native_c_dir.join("api.c"))
        .file(llhttp_native_c_dir.join("http.c"))
        .include(llhttp_c_dir)
        .include(llhttp_native_c_dir)
        .warnings(false)
        .compile("llhttp");
}
