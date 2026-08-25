fn main() {
    // slint_build::compile("./src/resources/ui/window.slint").unwrap();
    slint_build::compile_with_config(
        "./src/resources/ui/window.slint",
        slint_build::CompilerConfiguration::new().with_style("native".to_string()),
    )
    .unwrap();
}
