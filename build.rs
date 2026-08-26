// SPDX-FileCopyrightText: 2026 Foundation Devices, Inc. <hello@foundation.xyz>
// SPDX-FileCopyrightText: 2026 BitZk contributors
// SPDX-License-Identifier: MIT

use slint_keyos_platform_build::{compile_options, CompileOptions};

fn main() {
    // SDK 1.0.0 generates app themes locally during preview/simulator preflight.
    // Keep direct Cargo checks aligned with that output, not a global cache.
    let themes_rust_dir = std::env::var_os("FOUNDATION_THEMES_RUST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("target/foundation/themes/rust")
        });
    assert!(
        themes_rust_dir.join("app_theme.rs").is_file(),
        "Generated app theme is missing; run foundation preview or foundation sim first"
    );
    println!("cargo:rustc-env=FOUNDATION_THEMES_RUST_DIR={}", themes_rust_dir.display());
    println!("cargo:rerun-if-env-changed=FOUNDATION_THEMES_RUST_DIR");
    println!("cargo:rerun-if-changed={}", themes_rust_dir.display());

    compile_options(CompileOptions {
        module_path: "ui/app.slint",
        include_slint: true,
        include_router: true,
        include_translations: false,
        include_time_localization: false,
    });
}
