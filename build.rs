use std::env;
use std::path::PathBuf;
use cc::Build;
use bindgen;

fn main() {
    #[cfg(feature = "recompile")]
    compile();

    #[cfg(feature = "regenerate-bindings")]
    generate_bindings();
}

fn compile(){
    // Configure cross compilation
    let target = env::var("TARGET").unwrap();
    if !target.starts_with("thumbv6m-none-eabi") {
        panic!("This crate can only be built for ARM Cortex-M0+ targets.");
    }

    // Configure Compiler
    let mut opt_level = "s";
    let mut debug_flag = None;
    match &*env::var("PROFILE").unwrap() {
        "release" => opt_level = "fast",
        "debug" => { opt_level = "g"; debug_flag = Some("-g") },
        "relwithdebinfo" => { opt_level = "fast"; debug_flag = Some("-g") },
        "minsizerel" => opt_level = "s",
        _ => { opt_level = "g"; debug_flag = Some("-g") },
    };

    let mut build = Build::new();
    build.compiler("arm-none-eabi-gcc")
        .opt_level_str(opt_level)
        .flag("-mcpu=cortex-m0plus")
        .flag("-mthumb-interwork")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-common")
        .flag("-fmessage-length=0")
        //.flag(debug_flags)
        .files(vec![
            "PY32F0_Drivers/CMSIS/Device/PY32F0xx/Source/system_py32f0xx.c",
            "csrc/src/extra_functions.c",
        ])
        .include("csrc/inc")
        .include("PY32F0_Drivers/CMSIS/Include")
        .include("PY32F0_Drivers/CMSIS/Device/PY32F0xx/Include")
        .include("PY32F0_Drivers/PY32F0xx_HAL_Driver/Inc")
        .define("DEBUG", None)
        .define("PY32F030x6", None);

    if let Some(debug_flag_str) = debug_flag {
        build.flag(debug_flag_str);
    }

    for entry in std::fs::read_dir("PY32F0_Drivers/PY32F0xx_HAL_Driver/Src").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "c") {
            build.file(path);
        }
    }

    build.compile("py32csdk_hal");
}

fn generate_bindings(){

    // binding
    let bindings = bindgen::Builder::default()
        .header("csrc/inc/wrapper.h")
        .clang_arg("--target=arm-none-eabi")
        .clang_arg("-Icsrc/inc")
        .clang_arg("-IPY32F0_Drivers/CMSIS/Include")
        .clang_arg("-IPY32F0_Drivers/CMSIS/Device/PY32F0xx/Include")
        .clang_arg("-IPY32F0_Drivers/PY32F0xx_HAL_Driver/Inc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}