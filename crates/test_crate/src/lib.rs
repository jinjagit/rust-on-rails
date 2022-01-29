#[macro_use]
extern crate helix;

ruby! {
    class TestCrate {
        def hello() -> String {
            return "Hello from Rust test_crate!".to_string();
        }
    }
}
