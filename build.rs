use std::env;
use std::path::PathBuf;
use std::fs;

#[cfg(feature = "recompile")]
use cc::Build;

#[cfg(feature = "regenerate-bindings")]
use bindgen;

fn main() {
    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "py32f030")]
    let mcu_series = "py32f030";

    #[cfg(all(feature = "py32f030", feature = "py32xxx6"))]
    let mcu_name = "PY32F030x6";
    #[cfg(all(feature = "py32f030", feature = "py32xxx8"))]
    let mcu_name = "PY32F030x8";



    #[cfg(feature = "recompile")]
    compile(&mut out_path, mcu_name);

    #[cfg(feature = "regenerate-bindings")]
    generate_bindings(&mut out_path, mcu_name, mcu_series);

    #[cfg(not(feature = "recompile"))]
    copy_lib_file(&mut out_path, mcu_series);

    #[cfg(not(feature = "regenerate-bindings"))]
    copy_bindings_file(&mut out_path, mcu_series);

    copy_interrupt_file(&mut out_path, mcu_series);
}

#[cfg(not(feature = "recompile"))]
fn copy_lib_file(out_path: &mut PathBuf, mcu_series: &str){
    let sourse_path = format!("src/prebuild/{mcu_series}/libpy32csdk_hal.a");
    let source = PathBuf::from(sourse_path);
    let destination = out_path.join("libpy32csdk_hal.a");

    fs::copy(source, destination).unwrap();

    println!("cargo:rustc-link-lib=static=py32csdk_hal");
    println!("cargo:rustc-link-search={}", out_path.display());
}

#[cfg(not(feature = "regenerate-bindings"))]
fn copy_bindings_file(out_path: &mut PathBuf, mcu_series: &str){
    let sourse_path = format!("src/prebuild/{mcu_series}/bindings.rs");
    let source = PathBuf::from(sourse_path);
    let destination = out_path.join("bindings.rs");

    fs::copy(source, destination).unwrap();
}

fn copy_interrupt_file(out_path: &mut PathBuf, mcu_series: &str){
    // let sourse_path = format!("src/prebuild/{mcu_series}/interrupt.rs");
    // let source = PathBuf::from(sourse_path);
    // let destination = out_path.join("interrupt.rs");

    // fs::copy(source, destination).unwrap();

    let sourse_path = format!("src/prebuild/{mcu_series}/device.x");
    let source = PathBuf::from(sourse_path);
    let destination = out_path.join("device.x");

    fs::copy(source, destination).unwrap();

    // println!("cargo:rustc-link-search={}", out.display());
}

#[cfg(feature = "recompile")]
fn compile(out_path: &mut PathBuf, mcu_name: &str){
    // Configure cross compilation
    let target = env::var("TARGET").unwrap();
    if !target.starts_with("thumbv6m-none-eabi") {
        panic!("This crate can only be built for ARM Cortex-M0+ targets.");
    }

    // Configure Compiler
    let opt_level: &str;
    let mut debug_flag = None;
    match &*env::var("PROFILE").unwrap() {
        "release" => opt_level = "fast",
        "debug" => { opt_level = "g"; debug_flag = Some("-g") },
        "relwithdebinfo" => { opt_level = "fast"; debug_flag = Some("-g") },
        "minsizerel" => opt_level = "s",
        _ => { opt_level = "g"; debug_flag = Some("-g") },
    };

    let mut build = Build::new();
    // build.compiler("arm-none-eabi-gcc")
    //     .opt_level_str(opt_level)
    //     .flag("-mcpu=cortex-m0plus")
    //     .flag("-mthumb-interwork")
    //     .flag("-ffunction-sections")
    //     .flag("-fdata-sections")
    //     .flag("-fno-common")
    //     .flag("-fmessage-length=0")
    //     .flag("-w")
    build.compiler("clang")
        .opt_level_str(opt_level)
        .flag("--target=arm-none-eabi")
        .flag("-mcpu=cortex-m0plus")
        .flag("-mthumb")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-common")
        .flag("-fmessage-length=0")
        .flag("-w")
        //.flag(debug_flags)
        .files(vec![
            "PY32F0_Drivers/CMSIS/Device/PY32F0xx/Source/system_py32f0xx.c",
            "csrc/src/py32f030_wrapper.c",
        ])
        .include("csrc/inc")
        .include("PY32F0_Drivers/CMSIS/Include")
        .include("PY32F0_Drivers/CMSIS/Device/PY32F0xx/Include")
        .include("PY32F0_Drivers/PY32F0xx_HAL_Driver/Inc")
        .include("csrc/compile_inc")
        .define("DEBUG", None)
        .define(mcu_name, None);

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

    println!("cargo:rustc-link-lib=static=py32csdk_hal");
    println!("cargo:rustc-link-search={}", out_path.display());
}

#[cfg(feature = "regenerate-bindings")]
fn generate_bindings(out_path: &mut PathBuf, mcu_name: &str, mcu_series: &str){

    // binding
    let bindings = bindgen::Builder::default()
        .header("csrc/inc/wrapper.h")
        .clang_arg("--target=arm-none-eabi")
        //.clang_arg("-fpack-struct")
        .clang_arg("-Icsrc/inc")
        .clang_arg("-Icsrc/binding_inc")
        .clang_arg("-IPY32F0_Drivers/CMSIS/Include")
        .clang_arg("-IPY32F0_Drivers/CMSIS/Device/PY32F0xx/Include")
        .clang_arg("-IPY32F0_Drivers/PY32F0xx_HAL_Driver/Inc")
        .clang_arg(format!("-D{mcu_name}"))
        .clang_arg(format!("-D{mcu_series}"))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}