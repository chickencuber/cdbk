use slint_build::CompilerConfiguration;

fn main() {
    slint_build::compile("./src/resources/ui/window.slint").unwrap();
}
