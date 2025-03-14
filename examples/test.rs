fn main() {
    println!(
        "{:#?}",
        treesheets::Document::read(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/test.cts"))
    );
}
