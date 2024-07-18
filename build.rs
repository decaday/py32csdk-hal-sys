use std::env;
use std::path::PathBuf;
use cc::Build;
use bindgen;

fn main() {
    #[cfg(feature = "recompile")]
    compile();

    #[cfg(not(feature = "regenerate-bindings"))]
    generate_bindings();
}

fn recompile(){
    // Configure cross compilation
    let target = env::var("TARGET").unwrap();
    if !target.starts_with("thumbv6m-none-eabi") {
        panic!("This crate can only be built for ARM Cortex-M0+ targets.");
    }

    // Configure Compiler
    // let clang = "arm-clang";
    let cflags = "-mcpu=cortex-m0plus -mthumb -mthumb-interwork -ffunction-sections -fdata-sections -fno-common -fmessage-length=0";
    let debug_flags = match &*env::var("PROFILE").unwrap() {
        "release" => "-Ofast",
        "debug" => "-Og -g",
        "relwithdebinfo" => "-Ofast -g",
        "minsizerel" => "-Os",
        _ => "-Og -g",
    };

    let mut build = Build::new();
    build//.compiler(clang)
        .flag(cflags)
        .flag(debug_flags)
        .files(vec![
            "Drivers/CMSIS/Device/PY32F0xx/Source/system_py32f0xx.c",
            "Src/extra_functions.c",
        ])
        .include("Inc")
        .include("Drivers/CMSIS/Include")
        .include("Drivers/CMSIS/Device/PY32F0xx/Include")
        .include("Drivers/PY32F0xx_HAL_Driver/Inc")
        .define("DEBUG", None)
        .define("PY32F030x6", None);

    for entry in std::fs::read_dir("Drivers/PY32F0xx_HAL_Driver/Src").unwrap() {
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
        .header("wrapper.h")
        .clang_arg("-IInc")
        .clang_arg("-IDrivers/CMSIS/Include")
        .clang_arg("-IDrivers/CMSIS/Device/PY32F0xx/Include")
        .clang_arg("-IDrivers/PY32F0xx_HAL_Driver/Inc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}