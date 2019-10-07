extern crate cc;
extern crate pkg_config;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    let target = env::var("TARGET").unwrap();

    if target.contains("windows") && cfg!(feature = "edge") {
        build
            .include("webview.h")
            .file("webview_edge.cpp")
            .flag_if_supported("/std:c++17");
    } else {
        build
            .include("webview.h")
            .flag_if_supported("-std=c11")
            .flag_if_supported("-w");
    }

    if env::var("DEBUG").is_err() {
        build.define("NDEBUG", None);
    } else {
        build.define("DEBUG", None);
    }

    if target.contains("windows") {
        if !cfg!(feature = "edge") {
            build.file("webview_mshtml.c");

            for &lib in &["ole32", "comctl32", "oleaut32", "uuid", "gdi32"] {
                println!("cargo:rustc-link-lib={}", lib);
            }
        }
    } else if target.contains("linux") || target.contains("bsd") {
        let webkit = pkg_config::Config::new()
            .atleast_version("2.8")
            .probe("webkit2gtk-4.0")
            .unwrap();

        build.file("webview_gtk.c");

        for path in webkit.include_paths {
            build.include(path);
        }
    } else if target.contains("apple") {
        build
            .file("webview_cocoa.c")
            .define("OBJC_OLD_DISPATCH_PROTOTYPES", "1")
            .flag("-x")
            .flag("objective-c");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    } else {
        panic!("unsupported target");
    }

    build.compile("webview");
}
