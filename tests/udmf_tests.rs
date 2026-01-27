use std::ffi::CString;
use udmf::{
    AppItem, ArrayBuffer, ContentForm, FileUri, Html, Hyperlink, PixelMap, PlainText, UnifiedData,
    UnifiedRecord, UniformDataType,
};

#[test]
fn test_plain_text() {
    let mut pt = PlainText::new().expect("Failed to create PlainText");
    pt.set_content("Hello UDMF").expect("Failed to set content");
    pt.set_abstract("Topic: Hello")
        .expect("Failed to set abstract");

    assert_eq!(pt.get_content(), "Hello UDMF");
    assert_eq!(pt.get_abstract(), "Topic: Hello");
}

#[test]
fn test_hyperlink() {
    let mut hl = Hyperlink::new().expect("Failed to create Hyperlink");
    hl.set_url("https://openharmony.io")
        .expect("Failed to set URL");
    hl.set_description("OpenHarmony Homepage")
        .expect("Failed to set description");

    assert_eq!(hl.get_url(), "https://openharmony.io");
    assert_eq!(hl.get_description(), "OpenHarmony Homepage");
}

#[test]
fn test_html() {
    let mut html = Html::new().expect("Failed to create Html");
    html.set_content("<html><body>Hello</body></html>")
        .expect("Failed to set content");
    html.set_plain_content("Hello")
        .expect("Failed to set plain content");

    assert_eq!(html.get_content(), "<html><body>Hello</body></html>");
    assert_eq!(html.get_plain_content(), "Hello");
}

#[test]
fn test_app_item() {
    let mut app = AppItem::new().expect("Failed to create AppItem");
    app.set_bundle_name("com.example.app")
        .expect("Failed to set bundle name");
    app.set_ability_name("EntryAbility")
        .expect("Failed to set ability name");

    assert_eq!(app.get_bundle_name(), "com.example.app");
    assert_eq!(app.get_ability_name(), "EntryAbility");
}

#[test]
fn test_file_uri() {
    let mut file_uri = FileUri::new().expect("Failed to create FileUri");
    file_uri
        .set_file_uri("file://data/test.txt")
        .expect("Failed to set file URI");
    file_uri
        .set_file_type("text/plain")
        .expect("Failed to set file type");

    assert_eq!(file_uri.get_file_uri(), "file://data/test.txt");
    assert_eq!(file_uri.get_file_type(), "text/plain");
}

#[test]
fn test_array_buffer() {
    let mut buffer = ArrayBuffer::new().expect("Failed to create ArrayBuffer");
    let data = vec![1, 2, 3, 4, 5];
    buffer.set_data(&data).expect("Failed to set data");

    assert_eq!(buffer.get_data().expect("Failed to get data"), data);
}

#[test]
fn test_content_form() {
    let mut form = ContentForm::new().expect("Failed to create ContentForm");
    form.set_title("Test Title").expect("Failed to set title");
    form.set_description("Test Description")
        .expect("Failed to set description");
    form.set_app_name("Test App")
        .expect("Failed to set app name");
    form.set_link_uri("https://example.com")
        .expect("Failed to set link URI");

    let thumb = vec![0xAA, 0xBB, 0xCC];
    form.set_thumb_data(&thumb)
        .expect("Failed to set thumb data");

    let icon = vec![0x11, 0x22, 0x33];
    form.set_app_icon(&icon).expect("Failed to set app icon");

    assert_eq!(form.get_title(), "Test Title");
    assert_eq!(form.get_description(), "Test Description");
    assert_eq!(form.get_app_name(), "Test App");
    assert_eq!(form.get_link_uri(), "https://example.com");
    assert_eq!(form.get_thumb_data().expect("Failed to get thumb"), thumb);
    assert_eq!(form.get_app_icon().expect("Failed to get icon"), icon);
}

// It's a bit unclear how we would get a valid PixelMap pointer to test this.
#[test]
fn test_pixel_map_get_set() {
    let mut pm = PixelMap::new().expect("Failed to create PixelMap");

    // SAFETY: pm is valid.
    let initial_ptr = unsafe { pm.get_pixel_map() };
    println!("Initial PixelMap pointer: {:?}", initial_ptr);

    // Test setting a NULL pointer
    let null_ptr = std::ptr::null_mut();
    println!("Setting PixelMap to NULL...");
    unsafe { pm.set_pixel_map(null_ptr) }.expect_err("Pixelmap nullptr should fail");

    let after_null_ptr = unsafe { pm.get_pixel_map() };
    println!("PixelMap pointer after NULL set: {:?}", after_null_ptr);
    assert_eq!(
        after_null_ptr, initial_ptr,
        "Pixelmap ptr should not change after failed set call"
    );
}

#[test]
fn test_unified_record_general_entry() {
    let mut record = UnifiedRecord::new().expect("Failed to create UnifiedRecord");
    let data = vec![1, 3, 3, 7];

    let custom_type = UniformDataType::Other(CString::new("custom.type").unwrap());
    record
        .add_general_entry(&custom_type, &data)
        .expect("Failed to add general entry");

    let types = record.get_types();
    assert!(types.contains(&custom_type));

    let got_data = record
        .get_general_entry(&custom_type)
        .expect("Failed to get general entry");
    assert_eq!(got_data, data);
}

#[test]
fn test_unified_record_general_entry_custom_type() {
    let mut record = UnifiedRecord::new().expect("Failed to create UnifiedRecord");
    let data = vec![1, 3, 3, 7];

    {
        let custom_type = UniformDataType::Other(CString::new("custom.type").unwrap());
        record
            .add_general_entry(&custom_type, &data)
            .expect("Failed to add general entry");
    }

    // Create a new instance of the same type, but with a different backing CString!
    let custom_type_instance2 = UniformDataType::Other(CString::new("custom.type").unwrap());
    let types = record.get_types();
    assert!(types.contains(&custom_type_instance2));
    assert!(!types.contains(&UniformDataType::PlainText));

    let got_data = record
        .get_general_entry(&custom_type_instance2)
        .expect("Failed to get general entry");
    assert_eq!(got_data, data);
}

#[test]
fn test_unified_record_uds_types() {
    let mut record = UnifiedRecord::new().expect("Failed to create UnifiedRecord");

    let mut pt = PlainText::new().expect("New PlainText");
    pt.set_content("text").expect("Set content");
    record.add_plain_text(&pt).expect("Add PlainText");

    let mut hl = Hyperlink::new().expect("New Hyperlink");
    hl.set_url("url").expect("Set url");
    record.add_hyperlink(&hl).expect("Add Hyperlink");

    let mut html = Html::new().expect("New Html");
    html.set_content("html").expect("Set html");
    record.add_html(&html).expect("Add Html");

    let mut app = AppItem::new().expect("New AppItem");
    app.set_bundle_name("app").expect("Set bundle");
    record.add_app_item(&app).expect("Add AppItem");

    let types = record.get_types();
    assert!(types.len() >= 4);

    assert_eq!(
        record.get_plain_text().expect("get pt").get_content(),
        "text"
    );
    assert_eq!(record.get_hyperlink().expect("get hl").get_url(), "url");
    assert_eq!(record.get_html().expect("get html").get_content(), "html");
    assert_eq!(
        record.get_app_item().expect("get app").get_bundle_name(),
        "app"
    );
}

#[test]
fn test_unified_data_auxiliary() {
    let mut data = UnifiedData::new().expect("New UnifiedData");
    let some_type = UniformDataType::Other(CString::new("some.type").unwrap());
    assert!(!data.has_type(&some_type));

    let mut record = UnifiedRecord::new().expect("New Record");
    record
        .add_general_entry(&some_type, &[1])
        .expect("Add entry");

    data.add_record(&record).expect("Add record");

    assert!(data.has_type(&some_type));
    let types = data.get_types();
    assert!(types.contains(&some_type));

    let records = data.get_records();
    assert_eq!(records.len(), 1);
}
