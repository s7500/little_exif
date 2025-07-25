use super::fourcc_constants::*;
use crate::endian::Endian;
use crate::u8conversion::U8conversion;
use crate::video_tag_format::*;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum VideoTag {
    // Basic video properties (stored in mvhd/tkhd)
    Duration(VINT64U),  // Movie duration in time units
    Width(VINT32U),     // Video width in pixels
    Height(VINT32U),    // Video height in pixels
    Timescale(VINT32U), // Time scale (units per second)

    // Video codec information (stored in stsd)
    VideoCodec(VFOURCC), // Video codec FourCC (e.g., 'avc1', 'hvc1')
    AudioCodec(VFOURCC), // Audio codec FourCC (e.g., 'mp4a')
    Bitrate(VINT32U),    // Average bitrate
    Framerate(VFLOAT),   // Frames per second

    // Transformation and display
    TransformMatrix(VMATRIX), // 3x3 transformation matrix for rotation/scaling
    PixelAspectRatio(VINT32U), // Pixel aspect ratio

    // iTunes-style metadata (stored in ilst)
    Title(VSTRING),       // Song/video title
    Artist(VSTRING),      // Artist/performer
    Album(VSTRING),       // Album name
    AlbumArtist(VSTRING), // Album artist
    Composer(VSTRING),    // Music composer
    Genre(VSTRING),       // Genre as string
    GenreID(VINT16U),     // Genre as ID3v1 ID
    ReleaseDate(VSTRING), // Release date (YYYY-MM-DD)
    Comment(VSTRING),     // Comment/description
    Grouping(VSTRING),    // Grouping
    Lyrics(VSTRING),      // Song lyrics

    // Track/Disc information
    TrackNumber(VINT32U), // Track number and total tracks
    DiscNumber(VINT32U),  // Disc number and total discs
    Compilation(VINT8U),  // Compilation flag (0 or 1)
    Tempo(VINT16U),       // BPM (beats per minute)

    // Timestamps
    CreationTime(VTIMESTAMP),     // File creation time
    ModificationTime(VTIMESTAMP), // File modification time

    // Binary data for complex structures
    VideoCodecConfig(VBINARY), // Video codec configuration (avcC, hvcC, etc.)
    AudioCodecConfig(VBINARY), // Audio codec configuration (esds, etc.)
    CoverArt(VBINARY),         // Album artwork/thumbnail

    // Unknown tags with their original atom IDs
    UnknownVINT8U(VINT8U, u32),
    UnknownVSTRING(VSTRING, u32),
    UnknownVINT16U(VINT16U, u32),
    UnknownVINT32U(VINT32U, u32),
    UnknownVINT64U(VINT64U, u32),
    UnknownVFLOAT(VFLOAT, u32),
    UnknownVDOUBLE(VDOUBLE, u32),
    UnknownVFOURCC(VFOURCC, u32),
    UnknownVMATRIX(VMATRIX, u32),
    UnknownVTIMESTAMP(VTIMESTAMP, u32),
    UnknownVBINARY(VBINARY, u32),
}

impl VideoTag {
    /// Gets the FourCC atom type identifier for this video tag
    /// Returns the 32-bit identifier used in video container formats
    pub fn as_u32(&self) -> u32 {
        match *self {
            // Basic video properties - these are typically found in specific atoms
            VideoTag::Duration(_) => MVHD, // Duration is in movie header
            VideoTag::Width(_) => TKHD,    // Width is in track header
            VideoTag::Height(_) => TKHD,   // Height is in track header
            VideoTag::Timescale(_) => MVHD, // Timescale is in movie header

            // Codec information - found in sample description
            VideoTag::VideoCodec(_) => STSD, // Video codec in sample description
            VideoTag::AudioCodec(_) => STSD, // Audio codec in sample description
            VideoTag::Bitrate(_) => STSD,    // Bitrate info in sample description
            VideoTag::Framerate(_) => MDHD,  // Framerate derived from media header

            // Transformation and display
            VideoTag::TransformMatrix(_) => TKHD, // Transform matrix in track header
            VideoTag::PixelAspectRatio(_) => PIXELASPECT, // Pixel aspect ratio atom

            // iTunes-style metadata - all stored in ilst container
            VideoTag::Title(_) => NAM,              // '©nam' - title
            VideoTag::Artist(_) => ART,             // '©ART' - artist
            VideoTag::Album(_) => ALB,              // '©alb' - album
            VideoTag::AlbumArtist(_) => 0x61415254, // 'aART' - album artist
            VideoTag::Composer(_) => WRT,           // '©wrt' - composer/writer
            VideoTag::Genre(_) => GEN,              // '©gen' - genre (string)
            VideoTag::GenreID(_) => GNRE,           // 'gnre' - genre (ID)
            VideoTag::ReleaseDate(_) => DAY,        // '©day' - release date
            VideoTag::Comment(_) => CMT,            // '©cmt' - comment
            VideoTag::Grouping(_) => GRP,           // '©grp' - grouping
            VideoTag::Lyrics(_) => LYR,             // '©lyr' - lyrics

            // Track/Disc information
            VideoTag::TrackNumber(_) => TRKN, // 'trkn' - track number
            VideoTag::DiscNumber(_) => DISK,  // 'disk' - disc number
            VideoTag::Compilation(_) => CPIL, // 'cpil' - compilation
            VideoTag::Tempo(_) => TMPO,       // 'tmpo' - tempo (BPM)

            // Timestamps
            VideoTag::CreationTime(_) => MVHD, // Creation time in movie header
            VideoTag::ModificationTime(_) => MVHD, // Modification time in movie header

            // Binary codec configurations
            VideoTag::VideoCodecConfig(_) => AVCCONFIG, // 'avcC' - AVC config (or hvcC for HEVC)
            VideoTag::AudioCodecConfig(_) => ESDS,      // 'esds' - elementary stream descriptor
            VideoTag::CoverArt(_) => 0x636F7672,        // 'covr' - cover art

            // Unknown tags return their stored atom ID
            VideoTag::UnknownVINT8U(_, atom_id) => atom_id,
            VideoTag::UnknownVSTRING(_, atom_id) => atom_id,
            VideoTag::UnknownVINT16U(_, atom_id) => atom_id,
            VideoTag::UnknownVINT32U(_, atom_id) => atom_id,
            VideoTag::UnknownVINT64U(_, atom_id) => atom_id,
            VideoTag::UnknownVFLOAT(_, atom_id) => atom_id,
            VideoTag::UnknownVDOUBLE(_, atom_id) => atom_id,
            VideoTag::UnknownVFOURCC(_, atom_id) => atom_id,
            VideoTag::UnknownVMATRIX(_, atom_id) => atom_id,
            VideoTag::UnknownVTIMESTAMP(_, atom_id) => atom_id,
            VideoTag::UnknownVBINARY(_, atom_id) => atom_id,
        }
    }

    /// Helper method to convert FourCC to human-readable string for debugging
    pub fn fourcc_as_string(&self) -> String {
        let fourcc = self.as_u32();
        let bytes = fourcc.to_be_bytes();

        // Handle special case of © symbol (0xA9) in iTunes metadata
        if bytes[0] == 0xA9 {
            format!("©{}", String::from_utf8_lossy(&bytes[1..]))
        } else {
            String::from_utf8_lossy(&bytes).to_string()
        }
    }

    /// Gets the value stored in the tag as a u8 vector for encoding
    pub fn value_as_u8_vec(&self, endian: &Endian) -> Vec<u8> {
        match self {
            VideoTag::Duration(value) => value.to_u8_vec(endian),
            VideoTag::Width(value) => value.to_u8_vec(endian),
            VideoTag::Height(value) => value.to_u8_vec(endian),
            VideoTag::Timescale(value) => value.to_u8_vec(endian),

            VideoTag::VideoCodec(value) => value.to_u8_vec(endian),
            VideoTag::AudioCodec(value) => value.to_u8_vec(endian),
            VideoTag::Bitrate(value) => value.to_u8_vec(endian),
            VideoTag::Framerate(value) => value.to_u8_vec(endian),

            VideoTag::TransformMatrix(value) => value.to_u8_vec(endian),
            VideoTag::PixelAspectRatio(value) => value.to_u8_vec(endian),

            // String metadata
            VideoTag::Title(value) => value.to_u8_vec(endian),
            VideoTag::Artist(value) => value.to_u8_vec(endian),
            VideoTag::Album(value) => value.to_u8_vec(endian),
            VideoTag::AlbumArtist(value) => value.to_u8_vec(endian),
            VideoTag::Composer(value) => value.to_u8_vec(endian),
            VideoTag::Genre(value) => value.to_u8_vec(endian),
            VideoTag::ReleaseDate(value) => value.to_u8_vec(endian),
            VideoTag::Comment(value) => value.to_u8_vec(endian),
            VideoTag::Grouping(value) => value.to_u8_vec(endian),
            VideoTag::Lyrics(value) => value.to_u8_vec(endian),

            // Numeric metadata
            VideoTag::GenreID(value) => value.to_u8_vec(endian),
            VideoTag::TrackNumber(value) => value.to_u8_vec(endian),
            VideoTag::DiscNumber(value) => value.to_u8_vec(endian),
            VideoTag::Compilation(value) => value.to_u8_vec(endian),
            VideoTag::Tempo(value) => value.to_u8_vec(endian),

            // Timestamps
            VideoTag::CreationTime(value) => value.to_u8_vec(endian),
            VideoTag::ModificationTime(value) => value.to_u8_vec(endian),

            // Binary data - return as-is (no endian conversion needed)
            VideoTag::VideoCodecConfig(value) => value.clone(),
            VideoTag::AudioCodecConfig(value) => value.clone(),
            VideoTag::CoverArt(value) => value.clone(),

            // Unknown tags
            VideoTag::UnknownVINT8U(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVSTRING(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVINT16U(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVINT32U(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVINT64U(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVFLOAT(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVDOUBLE(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVFOURCC(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVMATRIX(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVTIMESTAMP(value, _) => value.to_u8_vec(endian),
            VideoTag::UnknownVBINARY(value, _) => value.clone(),
        }
    }

    /// Check if a tag is essential and should be kept during minimize
    pub fn is_essential_tag(&self) -> bool {
        matches!(
            self,
            VideoTag::Duration(_)
                | VideoTag::Width(_)
                | VideoTag::Height(_)
                | VideoTag::Timescale(_)
        )
    }
}
