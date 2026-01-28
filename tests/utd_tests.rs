use udmf::{TypeDescriptor, UniformDataType};

#[test]
fn test_type_descriptor_basic() {
    // UniformDataType::PlainText is a standard UTD type
    let utd =
        TypeDescriptor::new(&UniformDataType::PlainText).expect("Failed to create TypeDescriptor");

    assert_eq!(utd.get_type_id(), UniformDataType::PlainText);
    assert!(!utd.get_description().is_empty());

    // Check relationships
    assert!(utd.belongs_to(UniformDataType::Text));
}

#[test]
fn test_type_descriptor_lists() {
    let utd =
        TypeDescriptor::new(&UniformDataType::PlainText).expect("Failed to create TypeDescriptor");

    let mut mimes = utd.get_mime_types();
    assert!(mimes.any(|x| x == "text/plain"));

    let mut extensions = utd.get_filename_extensions();
    assert!(extensions.any(|x| x == ".txt"));

    let mut belongs = utd.get_belonging_to_types();
    assert!(belongs.any(|x| x == UniformDataType::Text));
}

#[test]
fn test_type_descriptor_url() {
    // Some types might have a reference URL
    let utd =
        TypeDescriptor::new(&UniformDataType::PlainText).expect("Failed to create TypeDescriptor");
    let url = utd.get_reference_url();
    // It might be empty for plain-text, but we test the API call doesn't crash
    println!("Reference URL: {}", url);
}

#[test]
fn test_type_descriptor_search() {
    let mut types = TypeDescriptor::get_types_by_filename_extension(".txt");
    assert!(types.any(|x| x == UniformDataType::PlainText));

    let mut types = TypeDescriptor::get_types_by_mime_type("text/plain");
    assert!(types.any(|x| x == UniformDataType::PlainText));
}
