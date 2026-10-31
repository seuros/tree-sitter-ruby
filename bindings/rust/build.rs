fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = if std::path::Path::new(&manifest_dir).join("src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("src")
    } else if std::path::Path::new(&manifest_dir).join("../../src").exists() {
        std::path::PathBuf::from(&manifest_dir).join("../../src")
    } else {
        panic!("Could not find src directory for tree-sitter-ruby")
    };

    let mut c_config = cc::Build::new();

    // FreeBSD-specific flags for proper symbol export
    #[cfg(target_os = "freebsd")]
    {
        c_config.flag("-fPIC");
    }

    c_config
        .std("c11")
        .include(&src_dir)
        .flag_if_supported("-Wno-unused-value");

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let Ok(wasm_headers) = std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS") else {
            panic!("Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS must be set by the language crate");
        };
        let Ok(wasm_src) =
            std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_SRC").map(std::path::PathBuf::from)
        else {
            panic!("Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_SRC must be set by the language crate");
        };

        c_config.include(&wasm_headers);
        c_config.files([
            wasm_src.join("stdio.c"),
            wasm_src.join("stdlib.c"),
            wasm_src.join("string.c"),
            wasm_src.join("wctype.c"),
        ]);
    }

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    c_config.compile("tree-sitter-ruby");
}
