fn main() {
    cc::Build::new()
        .cpp(true)
        .file("../src/pwm_qhash.cpp")
        .include("/usr/include/qt5/")
        .include("/usr/include/qt5/QtCore/")
        .compile("pwm_qhash");
    println!("cargo:rustc-link-lib=Qt5Core");
}
