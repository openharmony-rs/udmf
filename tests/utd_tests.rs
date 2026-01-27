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

    let mimes = utd.get_mime_types();
    assert!(mimes.contains(&"text/plain".to_string()));

    let extensions = utd.get_filename_extensions();
    assert!(extensions.contains(&".txt".to_string()));

    let belongs = utd.get_belonging_to_types();
    assert!(belongs.contains(&UniformDataType::Text));
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
    let types = TypeDescriptor::get_types_by_filename_extension(".txt");
    assert!(types.contains(&UniformDataType::PlainText));

    let types = TypeDescriptor::get_types_by_mime_type("text/plain");
    assert!(types.contains(&UniformDataType::PlainText));
}
