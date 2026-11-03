fn main() {
    rangular_build::build(
        rangular_build::Config::new(
            "src/components",
            "style/rangular.generated.css",
        )
        .global_scss("style/serenade.scss"),
    );
}
