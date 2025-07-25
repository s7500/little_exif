// MP4/QuickTime Atom Types (FourCC codes)
// Container atoms
pub const FTYP: u32 = 0x66747970; // 'ftyp' - file type box
pub const MOOV: u32 = 0x6D6F6F76; // 'moov' - movie container
pub const MVHD: u32 = 0x6D766864; // 'mvhd' - movie header
pub const TRAK: u32 = 0x7472616B; // 'trak' - track container
pub const TKHD: u32 = 0x746B6864; // 'tkhd' - track header
pub const MDIA: u32 = 0x6D646961; // 'mdia' - media container
pub const MDHD: u32 = 0x6D646864; // 'mdhd' - media header
pub const MINF: u32 = 0x6D696E66; // 'minf' - media information
pub const STBL: u32 = 0x7374626C; // 'stbl' - sample table
pub const STSD: u32 = 0x73747364; // 'stsd' - sample description
pub const MDAT: u32 = 0x6D646174; // 'mdat' - media data

// Metadata container atoms
pub const META: u32 = 0x6D657461; // 'meta' - metadata container
pub const ILST: u32 = 0x696C7374; // 'ilst' - item list (iTunes-style)
pub const UDTA: u32 = 0x75647461; // 'udta' - user data

// iTunes-style metadata atoms (with © symbol = 0xA9)
pub const NAM: u32 = 0xA96E616D; // '©nam' - title
pub const ART: u32 = 0xA9415254; // '©ART' - artist
pub const ALB: u32 = 0xA9616C62; // '©alb' - album
pub const DAY: u32 = 0xA9646179; // '©day' - release date
pub const GEN: u32 = 0xA967656E; // '©gen' - genre
pub const CMT: u32 = 0xA9636D74; // '©cmt' - comment
pub const WRT: u32 = 0xA9777274; // '©wrt' - composer
pub const GRP: u32 = 0xA9677270; // '©grp' - grouping
pub const LYR: u32 = 0xA96C7972; // '©lyr' - lyrics

// Technical metadata atoms
pub const CPIL: u32 = 0x6370696C; // 'cpil' - compilation flag
pub const TMPO: u32 = 0x746D706F; // 'tmpo' - tempo/BPM
pub const TRKN: u32 = 0x74726B6E; // 'trkn' - track number
pub const DISK: u32 = 0x6469736B; // 'disk' - disk number
pub const GNRE: u32 = 0x676E7265; // 'gnre' - genre (ID3v1 style)

// Video-specific atoms
pub const AVCCONFIG: u32 = 0x61766343; // 'avcC' - AVC configuration
pub const HVCCCONFIG: u32 = 0x68766343; // 'hvcC' - HEVC configuration
pub const PIXELASPECT: u32 = 0x70617370; // 'pasp' - pixel aspect ratio

// Audio-specific atoms
pub const ESDS: u32 = 0x65736473; // 'esds' - elementary stream descriptor

// Custom/Unknown marker
pub const UNKNOWN: u32 = 0x00000000; // Used for unknown atoms
