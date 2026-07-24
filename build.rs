// build.rs — required by esp-idf-sys to generate C bindings
fn main() {
    embuild::espidf::sysenv::output();
}
