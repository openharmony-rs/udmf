use crate::error::{Result, UdmfError, to_result};
use ohos_sys_opaque_types::*;
use std::ffi::{CStr, CString};
use udmf_sys::data_struct::*;

macro_rules! uds_wrapper {
    ($name:ident, $raw:ident, $create:ident, $destroy:ident) => {
        pub struct $name {
            pub(crate) inner: *mut $raw,
            owned: bool,
        }

        impl $name {
            pub fn new() -> Result<Self> {
                // SAFETY: $create() is an FFI function that creates a new UDS instance.
                let inner = unsafe { $create() };
                if inner.is_null() {
                    return Err(UdmfError::InternalError(0));
                }
                Ok(Self { inner, owned: true })
            }

            #[allow(dead_code)]
            pub(crate) unsafe fn from_ptr(inner: *mut $raw) -> Self {
                Self {
                    inner,
                    owned: false,
                }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.owned && !self.inner.is_null() {
                    // SAFETY: we own the inner pointer and it is valid.
                    unsafe { $destroy(self.inner) };
                }
            }
        }
    };
}

uds_wrapper!(
    PlainText,
    OH_UdsPlainText,
    OH_UdsPlainText_Create,
    OH_UdsPlainText_Destroy
);
impl PlainText {
    pub fn get_content(&self) -> String {
        self.get_content_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_content_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsPlainText.
        let c_ptr = unsafe { OH_UdsPlainText_GetContent(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_content(&mut self, content: &str) -> Result<()> {
        let c_content = CString::new(content).map_err(|_| UdmfError::InvalidParam)?;
        self.set_content_cstr(&c_content)
    }

    pub fn set_content_cstr(&mut self, content: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and content is a valid C string.
        let res = unsafe { OH_UdsPlainText_SetContent(self.inner, content.as_ptr()) };
        to_result(res)
    }

    pub fn get_abstract(&self) -> String {
        self.get_abstract_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_abstract_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsPlainText.
        let c_ptr = unsafe { OH_UdsPlainText_GetAbstract(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_abstract(&mut self, abstract_text: &str) -> Result<()> {
        let c_abstract = CString::new(abstract_text).map_err(|_| UdmfError::InvalidParam)?;
        self.set_abstract_cstr(&c_abstract)
    }

    pub fn set_abstract_cstr(&mut self, abstract_text: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_abstract is a valid C string.
        let res = unsafe { OH_UdsPlainText_SetAbstract(self.inner, abstract_text.as_ptr()) };
        to_result(res)
    }
}

uds_wrapper!(
    Hyperlink,
    OH_UdsHyperlink,
    OH_UdsHyperlink_Create,
    OH_UdsHyperlink_Destroy
);
impl Hyperlink {
    pub fn get_url(&self) -> String {
        self.get_url_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_url_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsHyperlink.
        let c_ptr = unsafe { OH_UdsHyperlink_GetUrl(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_url(&mut self, url: &str) -> Result<()> {
        let c_url = CString::new(url).map_err(|_| UdmfError::InvalidParam)?;
        self.set_url_cstr(&c_url)
    }

    pub fn set_url_cstr(&mut self, url: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_url is a valid C string.
        let res = unsafe { OH_UdsHyperlink_SetUrl(self.inner, url.as_ptr()) };
        to_result(res)
    }

    pub fn get_description(&self) -> String {
        self.get_description_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_description_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsHyperlink.
        let c_ptr = unsafe { OH_UdsHyperlink_GetDescription(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_desc = CString::new(description).map_err(|_| UdmfError::InvalidParam)?;
        self.set_description_cstr(&c_desc)
    }

    pub fn set_description_cstr(&mut self, description: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_desc is a valid C string.
        let res = unsafe { OH_UdsHyperlink_SetDescription(self.inner, description.as_ptr()) };
        to_result(res)
    }
}

uds_wrapper!(Html, OH_UdsHtml, OH_UdsHtml_Create, OH_UdsHtml_Destroy);
impl Html {
    pub fn get_content(&self) -> String {
        self.get_content_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_content_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsHtml.
        let c_ptr = unsafe { OH_UdsHtml_GetContent(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_content(&mut self, content: &str) -> Result<()> {
        let c_content = CString::new(content).map_err(|_| UdmfError::InvalidParam)?;
        self.set_content_cstr(&c_content)
    }

    pub fn set_content_cstr(&mut self, content: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_content is a valid C string.
        let res = unsafe { OH_UdsHtml_SetContent(self.inner, content.as_ptr()) };
        to_result(res)
    }

    pub fn get_plain_content(&self) -> String {
        self.get_plain_content_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_plain_content_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsHtml.
        let c_ptr = unsafe { OH_UdsHtml_GetPlainContent(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_plain_content(&mut self, plain_content: &str) -> Result<()> {
        let c_plain = CString::new(plain_content).map_err(|_| UdmfError::InvalidParam)?;
        self.set_plain_content_cstr(&c_plain)
    }

    pub fn set_plain_content_cstr(&mut self, plain_content: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_plain is a valid C string.
        let res = unsafe { OH_UdsHtml_SetPlainContent(self.inner, plain_content.as_ptr()) };
        to_result(res)
    }
}

uds_wrapper!(
    AppItem,
    OH_UdsAppItem,
    OH_UdsAppItem_Create,
    OH_UdsAppItem_Destroy
);
// Simplified AppItem implementation, focusing on bundle name and ability name
impl AppItem {
    pub fn get_bundle_name(&self) -> String {
        self.get_bundle_name_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_bundle_name_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsAppItem.
        let c_ptr = unsafe { OH_UdsAppItem_GetBundleName(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_bundle_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| UdmfError::InvalidParam)?;
        self.set_bundle_name_cstr(&c_name)
    }

    pub fn set_bundle_name_cstr(&mut self, name: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer and c_name is a valid C string.
        let res = unsafe { OH_UdsAppItem_SetBundleName(self.inner, name.as_ptr()) };
        to_result(res)
    }

    pub fn get_ability_name(&self) -> String {
        self.get_ability_name_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_ability_name_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsAppItem.
        let c_ptr = unsafe { OH_UdsAppItem_GetAbilityName(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_ability_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| UdmfError::InvalidParam)?;
        self.set_ability_name_cstr(&c_name)
    }

    pub fn set_ability_name_cstr(&mut self, name: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsAppItem, and c_name is a valid C string.
        let res = unsafe { OH_UdsAppItem_SetAbilityName(self.inner, name.as_ptr()) };
        to_result(res)
    }
}

uds_wrapper!(
    FileUri,
    OH_UdsFileUri,
    OH_UdsFileUri_Create,
    OH_UdsFileUri_Destroy
);
impl FileUri {
    pub fn get_file_uri(&self) -> String {
        self.get_file_uri_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_file_uri_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsFileUri.
        let c_ptr = unsafe { OH_UdsFileUri_GetFileUri(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_file_uri(&mut self, uri: &str) -> Result<()> {
        let c_uri = CString::new(uri).map_err(|_| UdmfError::InvalidParam)?;
        self.set_file_uri_cstr(&c_uri)
    }

    pub fn set_file_uri_cstr(&mut self, uri: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsFileUri, and c_uri is a valid C string.
        let res = unsafe { OH_UdsFileUri_SetFileUri(self.inner, uri.as_ptr()) };
        to_result(res)
    }

    pub fn get_file_type(&self) -> String {
        self.get_file_type_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_file_type_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsFileUri.
        let c_ptr = unsafe { OH_UdsFileUri_GetFileType(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_file_type(&mut self, file_type: &str) -> Result<()> {
        let c_type = CString::new(file_type).map_err(|_| UdmfError::InvalidParam)?;
        self.set_file_type_cstr(&c_type)
    }

    pub fn set_file_type_cstr(&mut self, file_type: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsFileUri, and c_type is a valid C string.
        let res = unsafe { OH_UdsFileUri_SetFileType(self.inner, file_type.as_ptr()) };
        to_result(res)
    }
}

uds_wrapper!(
    PixelMap,
    OH_UdsPixelMap,
    OH_UdsPixelMap_Create,
    OH_UdsPixelMap_Destroy
);
impl PixelMap {
    /// Get the pixel map from the UDS structure.
    /// Note: This returns a raw pointer to OH_PixelmapNative as we don't have a safe wrapper for it yet.
    ///
    /// # Safety
    ///
    /// The caller is responsible for the lifetime of the returned pointer.
    pub unsafe fn get_pixel_map(&self) -> *mut OH_PixelmapNative {
        let mut pixel_map = std::ptr::null_mut();
        // SAFETY: self.inner is a valid pointer to OH_UdsPixelMap.
        // The FFI function expects a pointer to a pointer to OH_PixelmapNative.
        // We provide a mutable reference to our `pixel_map` variable, which is a `*mut OH_PixelmapNative`,
        // and cast its address to `*mut *mut OH_PixelmapNative` to match the FFI signature.
        // The final cast to `*mut OH_PixelmapNative` is likely a workaround for an FFI signature
        // that might be declared as `*mut OH_PixelmapNative` but expects a `**mut OH_PixelmapNative`
        // for an output parameter. This is common in C APIs where `void**` is used for generic output.
        unsafe {
            OH_UdsPixelMap_GetPixelMap(
                self.inner,
                &mut pixel_map as *mut *mut OH_PixelmapNative as *mut OH_PixelmapNative,
            )
        };
        pixel_map
    }

    /// Set the pixel map in the UDS structure.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `pixel_map` is a valid pointer to an `OH_PixelmapNative` instance.
    pub unsafe fn set_pixel_map(&mut self, pixel_map: *mut OH_PixelmapNative) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsPixelMap.
        // pixel_map is assumed to be valid as per the function's safety requirements.
        let res = unsafe { OH_UdsPixelMap_SetPixelMap(self.inner, pixel_map) };
        to_result(res)
    }
}

uds_wrapper!(
    ArrayBuffer,
    OH_UdsArrayBuffer,
    OH_UdsArrayBuffer_Create,
    OH_UdsArrayBuffer_Destroy
);
impl ArrayBuffer {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        let mut len: u32 = 0;
        // SAFETY: self.inner is a valid pointer to OH_UdsArrayBuffer.
        // data_ptr and len are pointers to values that will be initialized by the FFI call.
        let res = unsafe { OH_UdsArrayBuffer_GetData(self.inner, &mut data_ptr, &mut len) };
        to_result(res)?;
        if data_ptr.is_null() || len == 0 {
            return Ok(Vec::new());
        }
        // SAFETY: data_ptr and len were returned by UDMF and are valid for the duration of this call.
        // We copy the data into a Vec.
        let data = unsafe { std::slice::from_raw_parts(data_ptr, len as usize) };
        Ok(data.to_vec())
    }

    pub fn set_data(&mut self, data: &[u8]) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsArrayBuffer.
        // data.as_ptr() and data.len() represent a valid slice of memory.
        let res = unsafe {
            OH_UdsArrayBuffer_SetData(self.inner, data.as_ptr() as *mut u8, data.len() as u32)
        };
        to_result(res)
    }
}

uds_wrapper!(
    ContentForm,
    OH_UdsContentForm,
    OH_UdsContentForm_Create,
    OH_UdsContentForm_Destroy
);
impl ContentForm {
    pub fn get_title(&self) -> String {
        self.get_title_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_title_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        let c_ptr = unsafe { OH_UdsContentForm_GetTitle(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let c_title = CString::new(title).map_err(|_| UdmfError::InvalidParam)?;
        self.set_title_cstr(&c_title)
    }

    pub fn set_title_cstr(&mut self, title: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm, and c_title is a valid C string.
        let res = unsafe { OH_UdsContentForm_SetTitle(self.inner, title.as_ptr()) };
        to_result(res)
    }

    pub fn get_description(&self) -> String {
        self.get_description_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_description_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        let c_ptr = unsafe { OH_UdsContentForm_GetDescription(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_desc = CString::new(description).map_err(|_| UdmfError::InvalidParam)?;
        self.set_description_cstr(&c_desc)
    }

    pub fn set_description_cstr(&mut self, description: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm, and c_desc is a valid C string.
        let res = unsafe { OH_UdsContentForm_SetDescription(self.inner, description.as_ptr()) };
        to_result(res)
    }

    pub fn get_app_name(&self) -> String {
        self.get_app_name_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_app_name_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        let c_ptr = unsafe { OH_UdsContentForm_GetAppName(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_app_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| UdmfError::InvalidParam)?;
        self.set_app_name_cstr(&c_name)
    }

    pub fn set_app_name_cstr(&mut self, name: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm, and c_name is a valid C string.
        let res = unsafe { OH_UdsContentForm_SetAppName(self.inner, name.as_ptr()) };
        to_result(res)
    }

    pub fn get_link_uri(&self) -> String {
        self.get_link_uri_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_link_uri_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        let c_ptr = unsafe { OH_UdsContentForm_GetLinkUri(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UDMF and it is valid as long as self is alive.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn set_link_uri(&mut self, uri: &str) -> Result<()> {
        let c_uri = CString::new(uri).map_err(|_| UdmfError::InvalidParam)?;
        self.set_link_uri_cstr(&c_uri)
    }

    pub fn set_link_uri_cstr(&mut self, uri: &CStr) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm, and c_uri is a valid C string.
        let res = unsafe { OH_UdsContentForm_SetLinkUri(self.inner, uri.as_ptr()) };
        to_result(res)
    }

    pub fn get_thumb_data(&self) -> Result<Vec<u8>> {
        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        let mut len: u32 = 0;
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        // data_ptr and len are pointers to values that will be initialized by the FFI call.
        let res = unsafe { OH_UdsContentForm_GetThumbData(self.inner, &mut data_ptr, &mut len) };
        to_result(res)?;
        if data_ptr.is_null() || len == 0 {
            return Ok(Vec::new());
        }
        // SAFETY: data_ptr and len were returned by UDMF and are valid for the duration of this call.
        let data = unsafe { std::slice::from_raw_parts(data_ptr, len as usize) };
        Ok(data.to_vec())
    }

    pub fn set_thumb_data(&mut self, data: &[u8]) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        // data.as_ptr() and data.len() represent a valid slice of memory.
        let res =
            unsafe { OH_UdsContentForm_SetThumbData(self.inner, data.as_ptr(), data.len() as u32) };
        to_result(res)
    }

    pub fn get_app_icon(&self) -> Result<Vec<u8>> {
        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        let mut len: u32 = 0;
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        // data_ptr and len are pointers to values that will be initialized by the FFI call.
        let res = unsafe { OH_UdsContentForm_GetAppIcon(self.inner, &mut data_ptr, &mut len) };
        to_result(res)?;
        if data_ptr.is_null() || len == 0 {
            return Ok(Vec::new());
        }
        // SAFETY: data_ptr and len were returned by UDMF and are valid for the duration of this call.
        let data = unsafe { std::slice::from_raw_parts(data_ptr, len as usize) };
        Ok(data.to_vec())
    }

    pub fn set_app_icon(&mut self, data: &[u8]) -> Result<()> {
        // SAFETY: self.inner is a valid pointer to OH_UdsContentForm.
        // data.as_ptr() and data.len() represent a valid slice of memory.
        let res =
            unsafe { OH_UdsContentForm_SetAppIcon(self.inner, data.as_ptr(), data.len() as u32) };
        to_result(res)
    }
}
