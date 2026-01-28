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
        self.get_description_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    /// Get the description as a CStr.
    pub fn get_description_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_Utd.
        let c_ptr = unsafe { OH_Utd_GetDescription(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UTD and is valid for the lifetime of self.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn get_reference_url(&self) -> String {
        self.get_reference_url_cstr()
            .map(|c| c.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    /// Get the reference URL as a CStr.
    pub fn get_reference_url_cstr(&self) -> Option<&CStr> {
        // SAFETY: self.inner is a valid pointer to OH_Utd.
        let c_ptr = unsafe { OH_Utd_GetReferenceUrl(self.inner) };
        if c_ptr.is_null() {
            None
        } else {
            // SAFETY: c_ptr is a valid C string returned by UTD and is valid for the lifetime of self.
            Some(unsafe { CStr::from_ptr(c_ptr) })
        }
    }

    pub fn get_mime_types(&self) -> impl Iterator<Item = String> + '_ {
        self.get_mime_types_cstr()
            .map(|c| c.to_string_lossy().into_owned())
    }

    /// Get the MIME types associated with this type as a list of CStr.
    pub fn get_mime_types_cstr(&self) -> impl Iterator<Item = &CStr> + '_ {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetMimeTypes(self.inner, &mut count) };
        UtdListIter::new(list_ptr, count)
    }

    pub fn get_filename_extensions(&self) -> impl Iterator<Item = String> + '_ {
        self.get_filename_extensions_cstr()
            .map(|c| c.to_string_lossy().into_owned())
    }

    /// Get the filename extensions associated with this type as a list of CStr.
    pub fn get_filename_extensions_cstr(&self) -> impl Iterator<Item = &CStr> + '_ {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetFilenameExtensions(self.inner, &mut count) };
        UtdListIter::new(list_ptr, count)
    }

    pub fn get_belonging_to_types(&self) -> impl Iterator<Item = UniformDataType> + '_ {
        let mut count: u32 = 0;
        // SAFETY: self.inner is a valid pointer. count will be initialized by the FFI call.
        let list_ptr = unsafe { OH_Utd_GetBelongingToTypes(self.inner, &mut count) };
        UtdListIter::new(list_ptr, count).map(UniformDataType::from)
    }

    pub fn get_types_by_filename_extension(
        extension: &str,
    ) -> impl Iterator<Item = UniformDataType> {
        let mut count: u32 = 0;
        let list_ptr = if let Ok(c_ext) = CString::new(extension) {
            // SAFETY: c_ext is a valid C string. count will be initialized by the FFI call.
            unsafe { OH_Utd_GetTypesByFilenameExtension(c_ext.as_ptr(), &mut count) }
        } else {
            std::ptr::null_mut()
        };
        OwnedUtdListIter::new(list_ptr, count).map(|s| UniformDataType::from(s.as_c_str()))
    }

    pub fn get_types_by_mime_type(mime_type: &str) -> impl Iterator<Item = UniformDataType> {
        let mut count: u32 = 0;
        let list_ptr = if let Ok(c_mime) = CString::new(mime_type) {
            // SAFETY: c_mime is a valid C string. count will be initialized by the FFI call.
            unsafe { OH_Utd_GetTypesByMimeType(c_mime.as_ptr(), &mut count) }
        } else {
            std::ptr::null_mut()
        };
        OwnedUtdListIter::new(list_ptr, count).map(|s| UniformDataType::from(s.as_c_str()))
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

/// An iterator over a list of strings provided by the UDMF framework.
struct UtdListIter<'a> {
    list: &'a [*const std::os::raw::c_char],
    index: usize,
}

impl<'a> UtdListIter<'a> {
    fn new(list_ptr: *mut *const std::os::raw::c_char, count: u32) -> Self {
        if list_ptr.is_null() || count == 0 {
            return Self {
                list: &[],
                index: 0,
            };
        }
        Self {
            // SAFETY: list_ptr is valid and count is correct from FFI.
            list: unsafe { std::slice::from_raw_parts(list_ptr, count as usize) },
            index: 0,
        }
    }
}

impl<'a> Iterator for UtdListIter<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.list.len() {
            let ptr = self.list[self.index];
            self.index += 1;
            if !ptr.is_null() {
                // SAFETY: ptr is a valid C string and its lifetime is tied to the parent list.
                return Some(unsafe { CStr::from_ptr(ptr) });
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.list.len() - self.index;
        (0, Some(remaining))
    }
}

/// An iterator over a list of strings that must be destroyed after use.
struct OwnedUtdListIter {
    list_ptr: *mut *const std::os::raw::c_char,
    count: u32,
    index: u32,
}

impl OwnedUtdListIter {
    fn new(list_ptr: *mut *const std::os::raw::c_char, count: u32) -> Self {
        Self {
            list_ptr,
            count,
            index: 0,
        }
    }
}

impl Iterator for OwnedUtdListIter {
    type Item = CString;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.count {
            // SAFETY: list_ptr is valid and index is within bounds.
            let s_ptr = unsafe { *self.list_ptr.add(self.index as usize) };
            self.index += 1;
            if !s_ptr.is_null() {
                // SAFETY: s_ptr is a valid C string.
                return Some(unsafe { CStr::from_ptr(s_ptr) }.to_owned());
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (0, Some(remaining))
    }
}

impl Drop for OwnedUtdListIter {
    fn drop(&mut self) {
        if !self.list_ptr.is_null() {
            // SAFETY: list_ptr was returned by a function that explicitly says it must be destroyed.
            unsafe { OH_Utd_DestroyStringList(self.list_ptr, self.count) };
        }
    }
}
