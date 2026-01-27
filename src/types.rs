use std::ffi::{CStr, CString};
use std::fmt;
use udmf_sys::meta::*;

/// Unified Data Types (UDT) supported by UDMF.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniformDataType {
    Entity,
    Object,
    CompositeObject,
    Text,
    PlainText,
    Html,
    Hyperlink,
    Xml,
    SourceCode,
    Script,
    ShellScript,
    CshScript,
    PerlScript,
    PhpScript,
    PythonScript,
    RubyScript,
    TypeScript,
    JavaScript,
    CHeader,
    CSource,
    CPlusPlusHeader,
    CPlusPlusSource,
    JavaSource,
    Ebook,
    Epub,
    Azw,
    Azw3,
    Kfx,
    Mobi,
    Media,
    Image,
    Jpeg,
    Png,
    RawImage,
    Tiff,
    Bmp,
    Ico,
    PhotoshopImage,
    AiImage,
    WordDoc,
    Excel,
    Ppt,
    Pdf,
    Postscript,
    EncapsulatedPostscript,
    Video,
    Avi,
    Mpeg,
    Mpeg4,
    Video3gpp,
    Video3gpp2,
    WindowsMediaWm,
    WindowsMediaWmv,
    WindowsMediaWmp,
    Audio,
    Aac,
    Aiff,
    Alac,
    Flac,
    Mp3,
    Ogg,
    Pcm,
    WindowsMediaWma,
    WaveformAudio,
    WindowsMediaWmx,
    WindowsMediaWvx,
    WindowsMediaWax,
    GeneralFile,
    Directory,
    Folder,
    Symlink,
    Archive,
    Bz2Archive,
    DiskImage,
    TarArchive,
    ZipArchive,
    JavaArchive,
    GnuTarArchive,
    GnuZipArchive,
    GnuZipTarArchive,
    Calendar,
    Contact,
    Database,
    Message,
    Vcard,
    Navigation,
    Location,
    OpenHarmonyForm,
    OpenHarmonyAppItem,
    OpenHarmonyPixelMap,
    OpenHarmonyAtomicService,
    OpenHarmonyPackage,
    OpenHarmonyHap,
    Smil,
    Markdown,
    Fax,
    JfxFax,
    EfxFax,
    XbitmapImage,
    TgaImage,
    SgiImage,
    OpenexrImage,
    FlashpixImage,
    Realmedia,
    AuAudio,
    AifcAudio,
    Sd2Audio,
    Realaudio,
    Openxml,
    WordprocessingmlDocument,
    SpreadsheetmlSheet,
    PresentationmlPresentation,
    Opendocument,
    OpendocumentText,
    OpendocumentSpreadsheet,
    OpendocumentPresentation,
    OpendocumentGraphics,
    OpendocumentFormula,
    StuffitArchive,
    Vcs,
    Ics,
    Executable,
    PortableExecutable,
    SunJavaClass,
    Font,
    TruetypeFont,
    TruetypeCollectionFont,
    OpentypeFont,
    PostscriptFont,
    PostscriptPfbFont,
    PostscriptPfaFont,
    OpenHarmonyHdoc,
    OpenHarmonyHinote,
    OpenHarmonyStyledString,
    OpenHarmonyWant,
    GeneralFileUri,
    GeneralContentForm,
    /// Any other custom type (which might be added in future OpenHarmony versions).
    Other(CString),
}

impl UniformDataType {
    /// Convert the enum variant to a C string constant.
    pub fn to_cstr(&self) -> &CStr {
        match self {
            Self::Entity => UDMF_META_ENTITY,
            Self::Object => UDMF_META_OBJECT,
            Self::CompositeObject => UDMF_META_COMPOSITE_OBJECT,
            Self::Text => UDMF_META_TEXT,
            Self::PlainText => UDMF_META_PLAIN_TEXT,
            Self::Html => UDMF_META_HTML,
            Self::Hyperlink => UDMF_META_HYPERLINK,
            Self::Xml => UDMF_META_XML,
            Self::SourceCode => UDMF_META_SOURCE_CODE,
            Self::Script => UDMF_META_SCRIPT,
            Self::ShellScript => UDMF_META_SHELL_SCRIPT,
            Self::CshScript => UDMF_META_CSH_SCRIPT,
            Self::PerlScript => UDMF_META_PERL_SCRIPT,
            Self::PhpScript => UDMF_META_PHP_SCRIPT,
            Self::PythonScript => UDMF_META_PYTHON_SCRIPT,
            Self::RubyScript => UDMF_META_RUBY_SCRIPT,
            Self::TypeScript => UDMF_META_TYPE_SCRIPT,
            Self::JavaScript => UDMF_META_JAVA_SCRIPT,
            Self::CHeader => UDMF_META_C_HEADER,
            Self::CSource => UDMF_META_C_SOURCE,
            Self::CPlusPlusHeader => UDMF_META_C_PLUS_PLUS_HEADER,
            Self::CPlusPlusSource => UDMF_META_C_PLUS_PLUS_SOURCE,
            Self::JavaSource => UDMF_META_JAVA_SOURCE,
            Self::Ebook => UDMF_META_EBOOK,
            Self::Epub => UDMF_META_EPUB,
            Self::Azw => UDMF_META_AZW,
            Self::Azw3 => UDMF_META_AZW3,
            Self::Kfx => UDMF_META_KFX,
            Self::Mobi => UDMF_META_MOBI,
            Self::Media => UDMF_META_MEDIA,
            Self::Image => UDMF_META_IMAGE,
            Self::Jpeg => UDMF_META_JPEG,
            Self::Png => UDMF_META_PNG,
            Self::RawImage => UDMF_META_RAW_IMAGE,
            Self::Tiff => UDMF_META_TIFF,
            Self::Bmp => UDMF_META_BMP,
            Self::Ico => UDMF_META_ICO,
            Self::PhotoshopImage => UDMF_META_PHOTOSHOP_IMAGE,
            Self::AiImage => UDMF_META_AI_IMAGE,
            Self::WordDoc => UDMF_META_WORD_DOC,
            Self::Excel => UDMF_META_EXCEL,
            Self::Ppt => UDMF_META_PPT,
            Self::Pdf => UDMF_META_PDF,
            Self::Postscript => UDMF_META_POSTSCRIPT,
            Self::EncapsulatedPostscript => UDMF_META_ENCAPSULATED_POSTSCRIPT,
            Self::Video => UDMF_META_VIDEO,
            Self::Avi => UDMF_META_AVI,
            Self::Mpeg => UDMF_META_MPEG,
            Self::Mpeg4 => UDMF_META_MPEG4,
            Self::Video3gpp => UDMF_META_VIDEO_3GPP,
            Self::Video3gpp2 => UDMF_META_VIDEO_3GPP2,
            Self::WindowsMediaWm => UDMF_META_WINDOWS_MEDIA_WM,
            Self::WindowsMediaWmv => UDMF_META_WINDOWS_MEDIA_WMV,
            Self::WindowsMediaWmp => UDMF_META_WINDOWS_MEDIA_WMP,
            Self::Audio => UDMF_META_AUDIO,
            Self::Aac => UDMF_META_AAC,
            Self::Aiff => UDMF_META_AIFF,
            Self::Alac => UDMF_META_ALAC,
            Self::Flac => UDMF_META_FLAC,
            Self::Mp3 => UDMF_META_MP3,
            Self::Ogg => UDMF_META_OGG,
            Self::Pcm => UDMF_META_PCM,
            Self::WindowsMediaWma => UDMF_META_WINDOWS_MEDIA_WMA,
            Self::WaveformAudio => UDMF_META_WAVEFORM_AUDIO,
            Self::WindowsMediaWmx => UDMF_META_WINDOWS_MEDIA_WMX,
            Self::WindowsMediaWvx => UDMF_META_WINDOWS_MEDIA_WVX,
            Self::WindowsMediaWax => UDMF_META_WINDOWS_MEDIA_WAX,
            Self::GeneralFile => UDMF_META_GENERAL_FILE,
            Self::Directory => UDMF_META_DIRECTORY,
            Self::Folder => UDMF_META_FOLDER,
            Self::Symlink => UDMF_META_SYMLINK,
            Self::Archive => UDMF_META_ARCHIVE,
            Self::Bz2Archive => UDMF_META_BZ2_ARCHIVE,
            Self::DiskImage => UDMF_META_DISK_IMAGE,
            Self::TarArchive => UDMF_META_TAR_ARCHIVE,
            Self::ZipArchive => UDMF_META_ZIP_ARCHIVE,
            Self::JavaArchive => UDMF_META_JAVA_ARCHIVE,
            Self::GnuTarArchive => UDMF_META_GNU_TAR_ARCHIVE,
            Self::GnuZipArchive => UDMF_META_GNU_ZIP_ARCHIVE,
            Self::GnuZipTarArchive => UDMF_META_GNU_ZIP_TAR_ARCHIVE,
            Self::Calendar => UDMF_META_CALENDAR,
            Self::Contact => UDMF_META_CONTACT,
            Self::Database => UDMF_META_DATABASE,
            Self::Message => UDMF_META_MESSAGE,
            Self::Vcard => UDMF_META_VCARD,
            Self::Navigation => UDMF_META_NAVIGATION,
            Self::Location => UDMF_META_LOCATION,
            Self::OpenHarmonyForm => UDMF_META_OPENHARMONY_FORM,
            Self::OpenHarmonyAppItem => UDMF_META_OPENHARMONY_APP_ITEM,
            Self::OpenHarmonyPixelMap => UDMF_META_OPENHARMONY_PIXEL_MAP,
            Self::OpenHarmonyAtomicService => UDMF_META_OPENHARMONY_ATOMIC_SERVICE,
            Self::OpenHarmonyPackage => UDMF_META_OPENHARMONY_PACKAGE,
            Self::OpenHarmonyHap => UDMF_META_OPENHARMONY_HAP,
            Self::Smil => UDMF_META_SMIL,
            Self::Markdown => UDMF_META_MARKDOWN,
            Self::Fax => UDMF_META_FAX,
            Self::JfxFax => UDMF_META_JFX_FAX,
            Self::EfxFax => UDMF_META_EFX_FAX,
            Self::XbitmapImage => UDMF_META_XBITMAP_IMAGE,
            Self::TgaImage => UDMF_META_TGA_IMAGE,
            Self::SgiImage => UDMF_META_SGI_IMAGE,
            Self::OpenexrImage => UDMF_META_OPENEXR_IMAGE,
            Self::FlashpixImage => UDMF_META_FLASHPIX_IMAGE,
            Self::Realmedia => UDMF_META_REALMEDIA,
            Self::AuAudio => UDMF_META_AU_AUDIO,
            Self::AifcAudio => UDMF_META_AIFC_AUDIO,
            Self::Sd2Audio => UDMF_META_SD2_AUDIO,
            Self::Realaudio => UDMF_META_REALAUDIO,
            Self::Openxml => UDMF_META_OPENXML,
            Self::WordprocessingmlDocument => UDMF_META_WORDPROCESSINGML_DOCUMENT,
            Self::SpreadsheetmlSheet => UDMF_META_SPREADSHEETML_SHEET,
            Self::PresentationmlPresentation => UDMF_META_PRESENTATIONML_PRESENTATION,
            Self::Opendocument => UDMF_META_OPENDOCUMENT,
            Self::OpendocumentText => UDMF_META_OPENDOCUMENT_TEXT,
            Self::OpendocumentSpreadsheet => UDMF_META_OPENDOCUMENT_SPREADSHEET,
            Self::OpendocumentPresentation => UDMF_META_OPENDOCUMENT_PRESENTATION,
            Self::OpendocumentGraphics => UDMF_META_OPENDOCUMENT_GRAPHICS,
            Self::OpendocumentFormula => UDMF_META_OPENDOCUMENT_FORMULA,
            Self::StuffitArchive => UDMF_META_STUFFIT_ARCHIVE,
            Self::Vcs => UDMF_META_VCS,
            Self::Ics => UDMF_META_ICS,
            Self::Executable => UDMF_META_EXECUTABLE,
            Self::PortableExecutable => UDMF_META_PORTABLE_EXECUTABLE,
            Self::SunJavaClass => UDMF_META_SUN_JAVA_CLASS,
            Self::Font => UDMF_META_FONT,
            Self::TruetypeFont => UDMF_META_TRUETYPE_FONT,
            Self::TruetypeCollectionFont => UDMF_META_TRUETYPE_COLLECTION_FONT,
            Self::OpentypeFont => UDMF_META_OPENTYPE_FONT,
            Self::PostscriptFont => UDMF_META_POSTSCRIPT_FONT,
            Self::PostscriptPfbFont => UDMF_META_POSTSCRIPT_PFB_FONT,
            Self::PostscriptPfaFont => UDMF_META_POSTSCRIPT_PFA_FONT,
            Self::OpenHarmonyHdoc => UDMF_META_OPENHARMONY_HDOC,
            Self::OpenHarmonyHinote => UDMF_META_OPENHARMONY_HINOTE,
            Self::OpenHarmonyStyledString => UDMF_META_OPENHARMONY_STYLED_STRING,
            Self::OpenHarmonyWant => UDMF_META_OPENHARMONY_WANT,
            Self::GeneralFileUri => UDMF_META_GENERAL_FILE_URI,
            Self::GeneralContentForm => UDMF_METE_GENERAL_CONTENT_FORM,
            Self::Other(c_s) => c_s.as_c_str(),
        }
    }
}

impl From<&CStr> for UniformDataType {
    fn from(s: &CStr) -> Self {
        if s == UDMF_META_ENTITY {
            Self::Entity
        } else if s == UDMF_META_OBJECT {
            Self::Object
        } else if s == UDMF_META_COMPOSITE_OBJECT {
            Self::CompositeObject
        } else if s == UDMF_META_TEXT {
            Self::Text
        } else if s == UDMF_META_PLAIN_TEXT {
            Self::PlainText
        } else if s == UDMF_META_HTML {
            Self::Html
        } else if s == UDMF_META_HYPERLINK {
            Self::Hyperlink
        } else if s == UDMF_META_XML {
            Self::Xml
        } else if s == UDMF_META_SOURCE_CODE {
            Self::SourceCode
        } else if s == UDMF_META_SCRIPT {
            Self::Script
        } else if s == UDMF_META_SHELL_SCRIPT {
            Self::ShellScript
        } else if s == UDMF_META_CSH_SCRIPT {
            Self::CshScript
        } else if s == UDMF_META_PERL_SCRIPT {
            Self::PerlScript
        } else if s == UDMF_META_PHP_SCRIPT {
            Self::PhpScript
        } else if s == UDMF_META_PYTHON_SCRIPT {
            Self::PythonScript
        } else if s == UDMF_META_RUBY_SCRIPT {
            Self::RubyScript
        } else if s == UDMF_META_TYPE_SCRIPT {
            Self::TypeScript
        } else if s == UDMF_META_JAVA_SCRIPT {
            Self::JavaScript
        } else if s == UDMF_META_C_HEADER {
            Self::CHeader
        } else if s == UDMF_META_C_SOURCE {
            Self::CSource
        } else if s == UDMF_META_C_PLUS_PLUS_HEADER {
            Self::CPlusPlusHeader
        } else if s == UDMF_META_C_PLUS_PLUS_SOURCE {
            Self::CPlusPlusSource
        } else if s == UDMF_META_JAVA_SOURCE {
            Self::JavaSource
        } else if s == UDMF_META_EBOOK {
            Self::Ebook
        } else if s == UDMF_META_EPUB {
            Self::Epub
        } else if s == UDMF_META_AZW {
            Self::Azw
        } else if s == UDMF_META_AZW3 {
            Self::Azw3
        } else if s == UDMF_META_KFX {
            Self::Kfx
        } else if s == UDMF_META_MOBI {
            Self::Mobi
        } else if s == UDMF_META_MEDIA {
            Self::Media
        } else if s == UDMF_META_IMAGE {
            Self::Image
        } else if s == UDMF_META_JPEG {
            Self::Jpeg
        } else if s == UDMF_META_PNG {
            Self::Png
        } else if s == UDMF_META_RAW_IMAGE {
            Self::RawImage
        } else if s == UDMF_META_TIFF {
            Self::Tiff
        } else if s == UDMF_META_BMP {
            Self::Bmp
        } else if s == UDMF_META_ICO {
            Self::Ico
        } else if s == UDMF_META_PHOTOSHOP_IMAGE {
            Self::PhotoshopImage
        } else if s == UDMF_META_AI_IMAGE {
            Self::AiImage
        } else if s == UDMF_META_WORD_DOC {
            Self::WordDoc
        } else if s == UDMF_META_EXCEL {
            Self::Excel
        } else if s == UDMF_META_PPT {
            Self::Ppt
        } else if s == UDMF_META_PDF {
            Self::Pdf
        } else if s == UDMF_META_POSTSCRIPT {
            Self::Postscript
        } else if s == UDMF_META_ENCAPSULATED_POSTSCRIPT {
            Self::EncapsulatedPostscript
        } else if s == UDMF_META_VIDEO {
            Self::Video
        } else if s == UDMF_META_AVI {
            Self::Avi
        } else if s == UDMF_META_MPEG {
            Self::Mpeg
        } else if s == UDMF_META_MPEG4 {
            Self::Mpeg4
        } else if s == UDMF_META_VIDEO_3GPP {
            Self::Video3gpp
        } else if s == UDMF_META_VIDEO_3GPP2 {
            Self::Video3gpp2
        } else if s == UDMF_META_WINDOWS_MEDIA_WM {
            Self::WindowsMediaWm
        } else if s == UDMF_META_WINDOWS_MEDIA_WMV {
            Self::WindowsMediaWmv
        } else if s == UDMF_META_WINDOWS_MEDIA_WMP {
            Self::WindowsMediaWmp
        } else if s == UDMF_META_AUDIO {
            Self::Audio
        } else if s == UDMF_META_AAC {
            Self::Aac
        } else if s == UDMF_META_AIFF {
            Self::Aiff
        } else if s == UDMF_META_ALAC {
            Self::Alac
        } else if s == UDMF_META_FLAC {
            Self::Flac
        } else if s == UDMF_META_MP3 {
            Self::Mp3
        } else if s == UDMF_META_OGG {
            Self::Ogg
        } else if s == UDMF_META_PCM {
            Self::Pcm
        } else if s == UDMF_META_WINDOWS_MEDIA_WMA {
            Self::WindowsMediaWma
        } else if s == UDMF_META_WAVEFORM_AUDIO {
            Self::WaveformAudio
        } else if s == UDMF_META_WINDOWS_MEDIA_WMX {
            Self::WindowsMediaWmx
        } else if s == UDMF_META_WINDOWS_MEDIA_WVX {
            Self::WindowsMediaWvx
        } else if s == UDMF_META_WINDOWS_MEDIA_WAX {
            Self::WindowsMediaWax
        } else if s == UDMF_META_GENERAL_FILE {
            Self::GeneralFile
        } else if s == UDMF_META_DIRECTORY {
            Self::Directory
        } else if s == UDMF_META_FOLDER {
            Self::Folder
        } else if s == UDMF_META_SYMLINK {
            Self::Symlink
        } else if s == UDMF_META_ARCHIVE {
            Self::Archive
        } else if s == UDMF_META_BZ2_ARCHIVE {
            Self::Bz2Archive
        } else if s == UDMF_META_DISK_IMAGE {
            Self::DiskImage
        } else if s == UDMF_META_TAR_ARCHIVE {
            Self::TarArchive
        } else if s == UDMF_META_ZIP_ARCHIVE {
            Self::ZipArchive
        } else if s == UDMF_META_JAVA_ARCHIVE {
            Self::JavaArchive
        } else if s == UDMF_META_GNU_TAR_ARCHIVE {
            Self::GnuTarArchive
        } else if s == UDMF_META_GNU_ZIP_ARCHIVE {
            Self::GnuZipArchive
        } else if s == UDMF_META_GNU_ZIP_TAR_ARCHIVE {
            Self::GnuZipTarArchive
        } else if s == UDMF_META_CALENDAR {
            Self::Calendar
        } else if s == UDMF_META_CONTACT {
            Self::Contact
        } else if s == UDMF_META_DATABASE {
            Self::Database
        } else if s == UDMF_META_MESSAGE {
            Self::Message
        } else if s == UDMF_META_VCARD {
            Self::Vcard
        } else if s == UDMF_META_NAVIGATION {
            Self::Navigation
        } else if s == UDMF_META_LOCATION {
            Self::Location
        } else if s == UDMF_META_OPENHARMONY_FORM {
            Self::OpenHarmonyForm
        } else if s == UDMF_META_OPENHARMONY_APP_ITEM {
            Self::OpenHarmonyAppItem
        } else if s == UDMF_META_OPENHARMONY_PIXEL_MAP {
            Self::OpenHarmonyPixelMap
        } else if s == UDMF_META_OPENHARMONY_ATOMIC_SERVICE {
            Self::OpenHarmonyAtomicService
        } else if s == UDMF_META_OPENHARMONY_PACKAGE {
            Self::OpenHarmonyPackage
        } else if s == UDMF_META_OPENHARMONY_HAP {
            Self::OpenHarmonyHap
        } else if s == UDMF_META_SMIL {
            Self::Smil
        } else if s == UDMF_META_MARKDOWN {
            Self::Markdown
        } else if s == UDMF_META_FAX {
            Self::Fax
        } else if s == UDMF_META_JFX_FAX {
            Self::JfxFax
        } else if s == UDMF_META_EFX_FAX {
            Self::EfxFax
        } else if s == UDMF_META_XBITMAP_IMAGE {
            Self::XbitmapImage
        } else if s == UDMF_META_TGA_IMAGE {
            Self::TgaImage
        } else if s == UDMF_META_SGI_IMAGE {
            Self::SgiImage
        } else if s == UDMF_META_OPENEXR_IMAGE {
            Self::OpenexrImage
        } else if s == UDMF_META_FLASHPIX_IMAGE {
            Self::FlashpixImage
        } else if s == UDMF_META_REALMEDIA {
            Self::Realmedia
        } else if s == UDMF_META_AU_AUDIO {
            Self::AuAudio
        } else if s == UDMF_META_AIFC_AUDIO {
            Self::AifcAudio
        } else if s == UDMF_META_SD2_AUDIO {
            Self::Sd2Audio
        } else if s == UDMF_META_REALAUDIO {
            Self::Realaudio
        } else if s == UDMF_META_OPENXML {
            Self::Openxml
        } else if s == UDMF_META_WORDPROCESSINGML_DOCUMENT {
            Self::WordprocessingmlDocument
        } else if s == UDMF_META_SPREADSHEETML_SHEET {
            Self::SpreadsheetmlSheet
        } else if s == UDMF_META_PRESENTATIONML_PRESENTATION {
            Self::PresentationmlPresentation
        } else if s == UDMF_META_OPENDOCUMENT {
            Self::Opendocument
        } else if s == UDMF_META_OPENDOCUMENT_TEXT {
            Self::OpendocumentText
        } else if s == UDMF_META_OPENDOCUMENT_SPREADSHEET {
            Self::OpendocumentSpreadsheet
        } else if s == UDMF_META_OPENDOCUMENT_PRESENTATION {
            Self::OpendocumentPresentation
        } else if s == UDMF_META_OPENDOCUMENT_GRAPHICS {
            Self::OpendocumentGraphics
        } else if s == UDMF_META_OPENDOCUMENT_FORMULA {
            Self::OpendocumentFormula
        } else if s == UDMF_META_STUFFIT_ARCHIVE {
            Self::StuffitArchive
        } else if s == UDMF_META_VCS {
            Self::Vcs
        } else if s == UDMF_META_ICS {
            Self::Ics
        } else if s == UDMF_META_EXECUTABLE {
            Self::Executable
        } else if s == UDMF_META_PORTABLE_EXECUTABLE {
            Self::PortableExecutable
        } else if s == UDMF_META_SUN_JAVA_CLASS {
            Self::SunJavaClass
        } else if s == UDMF_META_FONT {
            Self::Font
        } else if s == UDMF_META_TRUETYPE_FONT {
            Self::TruetypeFont
        } else if s == UDMF_META_TRUETYPE_COLLECTION_FONT {
            Self::TruetypeCollectionFont
        } else if s == UDMF_META_OPENTYPE_FONT {
            Self::OpentypeFont
        } else if s == UDMF_META_POSTSCRIPT_FONT {
            Self::PostscriptFont
        } else if s == UDMF_META_POSTSCRIPT_PFB_FONT {
            Self::PostscriptPfbFont
        } else if s == UDMF_META_POSTSCRIPT_PFA_FONT {
            Self::PostscriptPfaFont
        } else if s == UDMF_META_OPENHARMONY_HDOC {
            Self::OpenHarmonyHdoc
        } else if s == UDMF_META_OPENHARMONY_HINOTE {
            Self::OpenHarmonyHinote
        } else if s == UDMF_META_OPENHARMONY_STYLED_STRING {
            Self::OpenHarmonyStyledString
        } else if s == UDMF_META_OPENHARMONY_WANT {
            Self::OpenHarmonyWant
        } else if s == UDMF_META_GENERAL_FILE_URI {
            Self::GeneralFileUri
        } else if s == UDMF_METE_GENERAL_CONTENT_FORM {
            Self::GeneralContentForm
        } else {
            Self::Other(s.to_owned())
        }
    }
}

impl fmt::Display for UniformDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_cstr().to_string_lossy())
    }
}
