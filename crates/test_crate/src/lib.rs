#[macro_use]
extern crate helix;

ruby! {
    class TestCrate {
        def hello() -> String {
            return "Testing test_crate!".to_string();
        }
    }
}
