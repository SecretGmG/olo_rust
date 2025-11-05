use olo_rust::{OLOUnit, olo_unit};

#[test]
fn test_set_olo_unit() {
    // Set all messages to go to stdout (unit 6)
    olo_unit(OLOUnit::PrintAll, None);
    olo_unit(OLOUnit::Message, None);
    olo_unit(OLOUnit::Warning, None);
    // Just to verify we can call it without crashing
    println!("Successfully called set_unit via FFI");
}
