use support::{project, execs};
use hamcrest::{assert_that, existing_file};
use cargo::util::process;

fn setup() {}

test!(cargo_test_simple {
    let p = project("foo")
        .file("Cargo.toml", basic_bin_manifest("foo").as_slice())
        .file("src/foo.rs", r#"
            fn hello() -> &'static str {
                "hello"
            }

            pub fn main() {
                println!("{}", hello())
            }

            #[test]
            fn test_hello() {
                assert_eq!(hello(), "hello")
            }"#);

    assert_that(p.cargo_process("cargo-build"), execs());
    assert_that(&p.bin("foo"), existing_file());

    assert_that(
        process(p.bin("foo")),
        execs().with_stdout("hello\n"));

    assert_that(p.cargo_process("cargo-test"),
        execs().with_stdout(format!("   Compiling foo v0.5.0 (file:{})\n\n\
                                    running 1 test\n\
                                    test test_hello ... ok\n\n\
                                    test result: ok. 1 passed; 0 failed; \
                                    0 ignored; 0 measured\n\n",
                                    p.root().display())));
    
    assert_that(&p.bin("tests/foo"), existing_file());
})

fn basic_bin_manifest(name: &str) -> String {
    format!(r#"
        [project]

        name = "{}"
        version = "0.5.0"
        authors = ["wycats@example.com"]

        [[bin]]

        name = "{}"
    "#, name, name)
}
