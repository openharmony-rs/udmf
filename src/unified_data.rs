use crate::UniformDataType;
use crate::error::{Result, UdmfError, to_result};
use ohos_sys_opaque_types::*;
use std::ffi::CStr;
use std::ptr;
use udmf_sys::data_management_framework::*;

pub struct UnifiedData {
    pub(crate) inner: *mut OH_UdmfData,
    owned: bool,
}

impl UnifiedData {
    pub fn new() -> Result<Self> {
        // SAFETY: OH_UdmfData_Create creates a new instance of UnifiedData.
        let inner = unsafe { OH_UdmfData_Create() };
        if inner.is_null() {
            return Err(UdmfError::InternalError(0));
        }
        Ok(Self { inner, owned: true })
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn from_ptr(inner: *mut OH_UdmfData) -> Self {
        Self {
            inner,
            owned: false,
        }
    }

    pub fn add_record(&mut self, record: &UnifiedRecord) -> Result<()> {
        // SAFETY: self.inner and record.inner are valid pointers to UdmfData and UdmfRecord respectively.
        let res = unsafe { OH_UdmfData_AddRecord(self.inner, record.inner) };
        to_result(res)
    }

    pub fn has_type(&self, type_id: &UniformDataType) -> bool {
        // SAFETY: self.inner is a valid pointer and type_id.to_cstr() is a valid C string.
        unsafe { OH_UdmfData_HasType(self.inner, type_id.to_cstr().as_ptr()) }
    }

    pub fn get_types(&self) -> Vec<crate::UniformDataType> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let types_ptr = unsafe { OH_UdmfData_GetTypes(self.inner, &mut count) };
        if types_ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let mut types = Vec::with_capacity(count as usize);
        for i in 0..count {
            // SAFETY: types_ptr is a valid pointer returned by UDMF and count is correct.
            let s_ptr = unsafe { *types_ptr.add(i as usize) };
            if !s_ptr.is_null() {
                // SAFETY: s_ptr is a valid C string.
                let c_str = unsafe { CStr::from_ptr(s_ptr) };
                types.push(c_str.into());
            }
        }
        // Note: We don't free types_ptr as there's no corresponding destroy function for strings in UdmfData
        types
    }

    pub fn get_records(&self) -> Vec<UnifiedRecord> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let records_ptr = unsafe { OH_UdmfData_GetRecords(self.inner, &mut count) };
        if records_ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let mut records = Vec::with_capacity(count as usize);
        for i in 0..count {
            // SAFETY: records_ptr is a valid pointer returned by UDMF and count is correct.
            let r_ptr = unsafe { *records_ptr.add(i as usize) };
            if !r_ptr.is_null() {
                // These records are belonging to the UnifiedData, so they are not owned by the wrapper
                // SAFETY: r_ptr is a valid pointer to OH_UdmfRecord.
                records.push(unsafe { UnifiedRecord::from_ptr(r_ptr) });
            }
        }
        records
    }
}

impl Drop for UnifiedData {
    fn drop(&mut self) {
        if self.owned && !self.inner.is_null() {
            // SAFETY: we own the inner pointer and it is valid.
            unsafe { OH_UdmfData_Destroy(self.inner) };
        }
    }
}

pub struct UnifiedRecord {
    pub(crate) inner: *mut OH_UdmfRecord,
    owned: bool,
}

impl UnifiedRecord {
    pub fn new() -> Result<Self> {
        // SAFETY: OH_UdmfRecord_Create creates a new instance of UdmfRecord.
        let inner = unsafe { OH_UdmfRecord_Create() };
        if inner.is_null() {
            return Err(UdmfError::InternalError(0));
        }
        Ok(Self { inner, owned: true })
    }

    pub(crate) unsafe fn from_ptr(inner: *mut OH_UdmfRecord) -> Self {
        Self {
            inner,
            owned: false,
        }
    }

    pub fn get_types(&self) -> Vec<crate::UniformDataType> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let types_ptr = unsafe { OH_UdmfRecord_GetTypes(self.inner, &mut count) };
        if types_ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let mut types = Vec::with_capacity(count as usize);
        for i in 0..count {
            // SAFETY: types_ptr is a valid pointer returned by UDMF and count is correct.
            let s_ptr = unsafe { *types_ptr.add(i as usize) };
            if !s_ptr.is_null() {
                // SAFETY: s_ptr is a valid C string.
                let c_str = unsafe { CStr::from_ptr(s_ptr) };
                types.push(c_str.into());
            }
        }
        types
    }

    pub fn add_general_entry(&mut self, type_id: &UniformDataType, data: &[u8]) -> Result<()> {
        // SAFETY: self.inner and type_id.to_cstr() are valid pointers.
        let res = unsafe {
            OH_UdmfRecord_AddGeneralEntry(
                self.inner,
                type_id.to_cstr().as_ptr(),
                data.as_ptr() as *mut u8,
                data.len() as u32,
            )
        };
        to_result(res)
    }

    pub fn get_general_entry(&self, type_id: &UniformDataType) -> Result<Vec<u8>> {
        let mut data_ptr: *mut u8 = ptr::null_mut();
        let mut count: u32 = 0;
        // SAFETY: self.inner, type_id.to_cstr() are valid pointers. data_ptr and count will be initialized by FFI.
        let res = unsafe {
            OH_UdmfRecord_GetGeneralEntry(
                self.inner,
                type_id.to_cstr().as_ptr(),
                &mut data_ptr,
                &mut count,
            )
        };
        to_result(res)?;

        if data_ptr.is_null() || count == 0 {
            return Ok(Vec::new());
        }

        // SAFETY: data_ptr is a valid pointer with count elements as returned by FFI.
        let data = unsafe { std::slice::from_raw_parts(data_ptr, count as usize) };
        Ok(data.to_vec())
    }

    pub fn add_plain_text(&mut self, plain_text: &crate::uds::PlainText) -> Result<()> {
        // SAFETY: self.inner and plain_text.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddPlainText(self.inner, plain_text.inner) };
        to_result(res)
    }

    pub fn get_plain_text(&self) -> Result<crate::uds::PlainText> {
        let plain_text = crate::uds::PlainText::new()?;
        // SAFETY: self.inner and plain_text.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetPlainText(self.inner, plain_text.inner) };
        to_result(res)?;
        Ok(plain_text)
    }

    pub fn add_hyperlink(&mut self, hyperlink: &crate::uds::Hyperlink) -> Result<()> {
        // SAFETY: self.inner and hyperlink.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddHyperlink(self.inner, hyperlink.inner) };
        to_result(res)
    }

    pub fn get_hyperlink(&self) -> Result<crate::uds::Hyperlink> {
        let hyperlink = crate::uds::Hyperlink::new()?;
        // SAFETY: self.inner and hyperlink.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetHyperlink(self.inner, hyperlink.inner) };
        to_result(res)?;
        Ok(hyperlink)
    }

    pub fn add_html(&mut self, html: &crate::uds::Html) -> Result<()> {
        // SAFETY: self.inner and html.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddHtml(self.inner, html.inner) };
        to_result(res)
    }

    pub fn get_html(&self) -> Result<crate::uds::Html> {
        let html = crate::uds::Html::new()?;
        // SAFETY: self.inner and html.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetHtml(self.inner, html.inner) };
        to_result(res)?;
        Ok(html)
    }

    pub fn add_app_item(&mut self, app_item: &crate::uds::AppItem) -> Result<()> {
        // SAFETY: self.inner and app_item.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddAppItem(self.inner, app_item.inner) };
        to_result(res)
    }

    pub fn get_app_item(&self) -> Result<crate::uds::AppItem> {
        let app_item = crate::uds::AppItem::new()?;
        // SAFETY: self.inner and app_item.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetAppItem(self.inner, app_item.inner) };
        to_result(res)?;
        Ok(app_item)
    }

    pub fn add_file_uri(&mut self, file_uri: &crate::uds::FileUri) -> Result<()> {
        // SAFETY: self.inner and file_uri.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddFileUri(self.inner, file_uri.inner) };
        to_result(res)
    }

    pub fn get_file_uri(&self) -> Result<crate::uds::FileUri> {
        let file_uri = crate::uds::FileUri::new()?;
        // SAFETY: self.inner and file_uri.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetFileUri(self.inner, file_uri.inner) };
        to_result(res)?;
        Ok(file_uri)
    }

    pub fn add_pixel_map(&mut self, pixel_map: &crate::uds::PixelMap) -> Result<()> {
        // SAFETY: self.inner and pixel_map.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddPixelMap(self.inner, pixel_map.inner) };
        to_result(res)
    }

    pub fn get_pixel_map(&self) -> Result<crate::uds::PixelMap> {
        let pixel_map = crate::uds::PixelMap::new()?;
        // SAFETY: self.inner and pixel_map.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetPixelMap(self.inner, pixel_map.inner) };
        to_result(res)?;
        Ok(pixel_map)
    }

    pub fn add_array_buffer(
        &mut self,
        type_id: &UniformDataType,
        buffer: &crate::uds::ArrayBuffer,
    ) -> Result<()> {
        // SAFETY: self.inner, type_id.to_cstr(), and buffer.inner are valid pointers.
        let res = unsafe {
            OH_UdmfRecord_AddArrayBuffer(self.inner, type_id.to_cstr().as_ptr(), buffer.inner)
        };
        to_result(res)
    }

    pub fn get_array_buffer(&self, type_id: &UniformDataType) -> Result<crate::uds::ArrayBuffer> {
        let buffer = crate::uds::ArrayBuffer::new()?;
        // SAFETY: self.inner, type_id.to_cstr(), and buffer.inner are valid pointers.
        let res = unsafe {
            OH_UdmfRecord_GetArrayBuffer(self.inner, type_id.to_cstr().as_ptr(), buffer.inner)
        };
        to_result(res)?;
        Ok(buffer)
    }

    pub fn add_content_form(&mut self, content_form: &crate::uds::ContentForm) -> Result<()> {
        // SAFETY: self.inner and content_form.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_AddContentForm(self.inner, content_form.inner) };
        to_result(res)
    }

    pub fn get_content_form(&self) -> Result<crate::uds::ContentForm> {
        let content_form = crate::uds::ContentForm::new()?;
        // SAFETY: self.inner and content_form.inner are valid pointers.
        let res = unsafe { OH_UdmfRecord_GetContentForm(self.inner, content_form.inner) };
        to_result(res)?;
        Ok(content_form)
    }
}

impl Drop for UnifiedRecord {
    fn drop(&mut self) {
        if self.owned && !self.inner.is_null() {
            // SAFETY: we own the inner pointer and it is valid.
            unsafe { OH_UdmfRecord_Destroy(self.inner) };
        }
    }
}
