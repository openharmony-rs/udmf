use crate::error::{Result, UdmfError};
use crate::types::UniformDataType;
use std::ffi::{CStr, CString};
use udmf_sys::type_descriptor::*;

pub struct TypeDescriptor {
    pub(crate) inner: *mut OH_Utd,
    owned: bool,
}

impl TypeDescriptor {
    /// Create a new TypeDescriptor from a type ID.
    pub fn new(type_id: &UniformDataType) -> Result<Self> {
        // SAFETY: type_id.to_cstr() returns a valid C string.
        let inner = unsafe { OH_Utd_Create(type_id.to_cstr().as_ptr()) };

        if inner.is_null() {
            return Err(UdmfError::InternalError(0));
        }
        Ok(Self { inner, owned: true })
    }

    /// Get the type ID.
    pub fn get_type_id(&self) -> UniformDataType {
        // SAFETY: self.inner is a valid pointer to OH_Utd.
        let c_str = unsafe { OH_Utd_GetTypeId(self.inner) };
        if c_str.is_null() {
            return UniformDataType::Other(CString::new("").unwrap());
        }
        // SAFETY: c_str is a valid C string returned by UTD.
        unsafe { CStr::from_ptr(c_str) }.into()
    }

    pub fn get_description(&self) -> String {
        // SAFETY: self.inner is a valid pointer to OH_Utd.
        let c_str = unsafe { OH_Utd_GetDescription(self.inner) };
        if c_str.is_null() {
            return String::new();
        }
        // SAFETY: c_str is a valid C string returned by UTD.
        unsafe { CStr::from_ptr(c_str) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn get_reference_url(&self) -> String {
        // SAFETY: self.inner is a valid pointer to OH_Utd.
        let c_str = unsafe { OH_Utd_GetReferenceUrl(self.inner) };
        if c_str.is_null() {
            return String::new();
        }
        // SAFETY: c_str is a valid C string returned by UTD.
        unsafe { CStr::from_ptr(c_str) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn get_mime_types(&self) -> Vec<String> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetMimeTypes(self.inner, &mut count) };
        Self::parse_list_unowned(list_ptr, count, Self::map_string)
    }

    pub fn get_filename_extensions(&self) -> Vec<String> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetFilenameExtensions(self.inner, &mut count) };
        Self::parse_list_unowned(list_ptr, count, Self::map_string)
    }

    pub fn get_belonging_to_types(&self) -> Vec<UniformDataType> {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetBelongingToTypes(self.inner, &mut count) };
        Self::parse_list_unowned(list_ptr, count, Self::map_type)
    }

    pub fn get_types_by_filename_extension(extension: &str) -> Vec<UniformDataType> {
        if let Ok(c_ext) = CString::new(extension) {
            let mut count: u32 = 0;
            // SAFETY: c_ext is a valid C string. count will be initialized by the FFI call.
            let list_ptr =
                unsafe { OH_Utd_GetTypesByFilenameExtension(c_ext.as_ptr(), &mut count) };
            return Self::parse_list_owned(list_ptr, count, Self::map_type);
        }
        Vec::new()
    }

    pub fn get_types_by_mime_type(mime_type: &str) -> Vec<UniformDataType> {
        if let Ok(c_mime) = CString::new(mime_type) {
            let mut count: u32 = 0;
            // SAFETY: c_mime is a valid C string. count will be initialized by the FFI call.
            let list_ptr = unsafe { OH_Utd_GetTypesByMimeType(c_mime.as_ptr(), &mut count) };
            return Self::parse_list_owned(list_ptr, count, Self::map_type);
        }
        Vec::new()
    }

    fn parse_list_unowned<T>(
        list_ptr: *mut *const ::core::ffi::c_char,
        count: u32,
        map: impl Fn(&CStr) -> T,
    ) -> Vec<T> {
        if list_ptr.is_null() || count == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(count as usize);
        for i in 0..count {
            // SAFETY: list_ptr is valid and count is correct from FFI.
            let s_ptr = unsafe { *list_ptr.add(i as usize) };
            if !s_ptr.is_null() {
                // SAFETY: s_ptr is a valid C string.
                let c_str = unsafe { CStr::from_ptr(s_ptr) };
                result.push(map(c_str));
            }
        }
        result
    }

    fn parse_list_owned<T>(
        list_ptr: *mut *const ::core::ffi::c_char,
        count: u32,
        map: impl Fn(&CStr) -> T,
    ) -> Vec<T> {
        let result = Self::parse_list_unowned(list_ptr, count, map);
        if !list_ptr.is_null() {
            // SAFETY: list_ptr was returned by a function that explicitly says it must be destroyed.
            unsafe { OH_Utd_DestroyStringList(list_ptr, count) };
        }
        result
    }

    fn map_string(c: &CStr) -> String {
        c.to_string_lossy().into_owned()
    }

    fn map_type(c: &CStr) -> UniformDataType {
        c.into()
    }

    pub fn belongs_to(&self, other_type_id: impl Into<UniformDataType>) -> bool {
        let u_other = other_type_id.into();
        let u_self = self.get_type_id();
        // SAFETY: both u_self.to_cstr() and u_other.to_cstr() return valid C strings.
        unsafe { OH_Utd_BelongsTo(u_self.to_cstr().as_ptr(), u_other.to_cstr().as_ptr()) }
    }
}

impl Drop for TypeDescriptor {
    fn drop(&mut self) {
        if self.owned && !self.inner.is_null() {
            // SAFETY: self.inner is a valid pointer and we own it.
            unsafe { OH_Utd_Destroy(self.inner) };
        }
    }
}
