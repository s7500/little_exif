// Video-specific data types (following your image metadata pattern)
pub type VINT8U = Vec<u8>;
pub type VSTRING = String;
pub type VINT16U = Vec<u16>;
pub type VINT32U = Vec<u32>;
pub type VINT64U = Vec<u64>;
pub type VFLOAT = Vec<f32>;
pub type VDOUBLE = Vec<f64>;
pub type VFOURCC = u32; // Common in video (like 'mp4v', 'avc1')
pub type VMATRIX = [i32; 9]; // Transformation matrix in video
pub type VTIMESTAMP = u64; // Video timestamps
pub type VBINARY = Vec<u8>; // Raw binary data

#[derive(Clone, Debug, PartialEq)]
pub enum VideoTagFormat {
    VINT8U,
    VSTRING,
    VINT16U,
    VINT32U,
    VINT64U,
    VFLOAT,
    VDOUBLE,
    VFOURCC,
    VMATRIX,
    VTIMESTAMP,
    VBINARY,
}
