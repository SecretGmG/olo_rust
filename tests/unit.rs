use olo_rust::{OLOUnit, set_log_level};

#[test]
fn test_set_olo_unit() {
    // Set all messages to go to stdout (unit 6)
    set_log_level(OLOUnit::PrintAll, None);
    set_log_level(OLOUnit::Message, None);
    set_log_level(OLOUnit::Warning, None);
    // Just to verify we can call it without crashing
    println!("Successfully called set_unit via FFI");
}
