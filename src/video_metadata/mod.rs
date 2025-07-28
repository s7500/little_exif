mod fourcc_constants;
pub mod iterator;
mod metadata_trait;
mod video_tag;
use crate::{
    endian::*,
    video_metadata::fourcc_constants::{FTYP, META, MOOV, MVHD, TRAK},
};
use log::{error, warn};
use metadata_trait::VMetadata;
use std::io::Cursor;
use video_tag::VideoTag;

#[derive(Clone)]
pub struct VideoMetadata {
    endian: Endian,
    atoms: Vec<VideoAtomContainer>,
}

#[derive(Clone, PartialEq)]
pub struct VideoAtomContainer {
    atom_type: u32,
    size: u64,
    tags: Vec<VideoTag>,
}

impl VMetadata for VideoMetadata {
    type AtomType = VideoAtomContainer;
    type TagIterator<'a> = std::iter::Empty<&'a VideoTag>;

    fn new() -> Self {
        VideoMetadata {
            endian: Endian::Big,
            atoms: Vec::new(),
        }
    }

    fn create_atom(&mut self, atom_type: u32) {
        let atom = VideoAtomContainer {
            atom_type,
            size: 0,
            tags: Vec::new(),
        };
        self.atoms.push(atom);
    }

    fn general_decoding_wrapper(
        raw_pre_decode_general: Result<Vec<u8>, std::io::Error>,
    ) -> Result<Self, std::io::Error> {
        if let Ok(pre_decode_general) = raw_pre_decode_general {
            let mut pre_decode_cursor = Cursor::new(&pre_decode_general);
            let decoding_result = Self::decode(&mut pre_decode_cursor);
            if let Ok((endian, video_atoms)) = decoding_result {
                let mut data = VideoMetadata {
                    endian,
                    atoms: video_atoms,
                };
                data.sort_data();
                return Ok(data);
            } else {
                error!("{}", decoding_result.err().unwrap());
            }
        } else {
            error!(
                "Error during decoding: {:?}",
                raw_pre_decode_general.err().unwrap()
            );
        }

        warn!("Can't read metadata - Create new & empty struct");
        return Ok(VideoMetadata::new());
    }

    fn encode(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut encode_vec = Vec::new();

        // Write file type box (ftyp) first - required for MP4 containers
        self.write_ftyp_atom(&mut encode_vec)?;

        // Encode all atoms in order
        for atom in &self.atoms {
            let atom_data = self.encode_atom(atom)?;

            // Write atom header (size + type)
            let atom_size = atom_data.len() + 8; // +8 for size and type fields
            encode_vec.extend(&(atom_size as u32).to_be_bytes()); // Size (big-endian)
            encode_vec.extend(&atom.atom_type.to_be_bytes()); // Type (FourCC)

            // Write atom data
            encode_vec.extend(atom_data);
        }

        Ok(encode_vec)
    }

    fn sort_data(&mut self) {
        // Sort atoms by type - important atoms first
        self.atoms.sort_by(|a, b| {
            let priority_a = a.get_atom_priority();
            let priority_b = b.get_atom_priority();
            priority_a.cmp(&priority_b)
        });
    }

    fn decode(
        _data_cursor: &mut Cursor<&Vec<u8>>,
    ) -> Result<(Endian, Vec<Self::AtomType>), std::io::Error> {
        // TODO: Implement decoding logic
        // For now, return empty result
        Ok((Endian::Big, Vec::new()))
    }

    fn reduce_to_minimum(&mut self) {
        // Remove non-essential atoms and tags
        self.atoms.retain(|atom| atom.is_essential_atom());

        for atom in &mut self.atoms {
            atom.tags.retain(|tag| tag.is_essential_tag());
        }
    }

    fn get_endian(&self) -> Endian {
        self.endian.clone()
    }

    fn get_atoms(&self) -> &Vec<Self::AtomType> {
        &self.atoms
    }

    fn get_atom(&self, atom_type: u32) -> Option<&Self::AtomType> {
        self.atoms.iter().find(|atom| atom.atom_type == atom_type)
    }

    fn get_atom_mut(&mut self, atom_type: u32) -> &mut Self::AtomType {
        // Create atom if it doesn't exist
        if self.get_atom(atom_type).is_none() {
            self.create_atom(atom_type);
        }

        self.atoms
            .iter_mut()
            .find(|atom| atom.atom_type == atom_type)
            .unwrap()
    }

    fn get_tag(&self, _tag: &VideoTag) -> Self::TagIterator<'_> {
        // TODO: Implement tag iterator
        std::iter::empty()
    }

    fn get_tag_by_id(&self, _tag_id: u32, _atom_type: Option<u32>) -> Self::TagIterator<'_> {
        // TODO: Implement tag iterator by ID
        std::iter::empty()
    }

    fn set_tag(&mut self, tag: VideoTag) {
        let atom_type = tag.as_u32();
        let atom = self.get_atom_mut(atom_type);

        // Remove existing tag of same type, then add new one
        atom.tags
            .retain(|existing_tag| existing_tag.as_u32() != tag.as_u32());
        atom.tags.push(tag);
    }
}

impl VideoMetadata {
    /// Write the file type atom (ftyp) - required for MP4 containers
    fn write_ftyp_atom(&self, encode_vec: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let ftyp_data = vec![
            // Major brand: 'mp41'
            0x6D, 0x70, 0x34, 0x31, // Minor version: 0
            0x00, 0x00, 0x00, 0x00, // Compatible brands: 'mp41', 'isom'
            0x6D, 0x70, 0x34, 0x31, 0x69, 0x73, 0x6F, 0x6D,
        ];

        // Write ftyp atom header
        let total_size = ftyp_data.len() + 8; // +8 for size and type
        encode_vec.extend(&(total_size as u32).to_be_bytes()); // Size
        encode_vec.extend(&0x66747970u32.to_be_bytes()); // 'ftyp'
        encode_vec.extend(ftyp_data);

        Ok(())
    }

    /// Encode a single atom container
    fn encode_atom(&self, atom: &VideoAtomContainer) -> Result<Vec<u8>, std::io::Error> {
        let mut atom_data = Vec::new();

        match atom.atom_type {
            MOOV => {
                // 'moov' - movie container
                self.encode_basic_atom(atom, &mut atom_data)?;
            }
            MVHD => {
                // 'mvhd' - movie header
                self.encode_mvhd_atom(atom, &mut atom_data)?;
            }
            TRAK => {
                // 'trak' - track container
                self.encode_basic_atom(atom, &mut atom_data)?;
            }
            META => {
                // 'meta' - metadata container
                self.encode_meta_atom(atom, &mut atom_data)?;
            }
            _ => {
                // Generic atom encoding - just write the tag data
                for tag in &atom.tags {
                    atom_data.extend(tag.value_as_u8_vec(&self.endian));
                }
            }
        }

        Ok(atom_data)
    }

    /// Encode movie container atom
    fn encode_basic_atom(
        &self,
        atom: &VideoAtomContainer,
        data: &mut Vec<u8>,
    ) -> Result<(), std::io::Error> {
        // moov atom contains other atoms (mvhd, trak, etc.)
        for tag in &atom.tags {
            data.extend(tag.value_as_u8_vec(&self.endian));
        }
        Ok(())
    }

    /// Encode movie header atom
    fn encode_mvhd_atom(
        &self,
        atom: &VideoAtomContainer,
        data: &mut Vec<u8>,
    ) -> Result<(), std::io::Error> {
        // mvhd structure: version(1) + flags(3) + creation_time(4) + modification_time(4) +
        // timescale(4) + duration(4) + rate(4) + volume(2) + reserved(10) + matrix(36) +
        // pre_defined(24) + next_track_id(4)

        // Version and flags
        data.extend(&[0x00, 0x00, 0x00, 0x00]); // version 0, flags 0

        // Extract values from tags
        let mut creation_time = 0u64;
        let mut modification_time = 0u64;
        let mut timescale = 1000u32; // Default 1000 units per second
        let mut duration = 0u64;
        let mut matrix = [0x00010000i32, 0, 0, 0, 0x00010000, 0, 0, 0, 0x40000000]; // Identity matrix

        for tag in &atom.tags {
            match tag {
                VideoTag::CreationTime(time) => {
                    if *time != 0u64 {
                        creation_time = *time;
                    }
                }
                VideoTag::ModificationTime(time) => {
                    if *time != 0u64 {
                        modification_time = *time;
                    }
                }
                VideoTag::Timescale(scale) => {
                    if !scale.is_empty() {
                        timescale = scale[0];
                    }
                }
                VideoTag::Duration(dur) => {
                    if !dur.is_empty() {
                        duration = dur[0];
                    }
                }
                VideoTag::TransformMatrix(m) => {
                    matrix = *m;
                }
                _ => {}
            }
        }

        // Write the mvhd data
        data.extend(&creation_time.to_be_bytes()); // Creation time
        data.extend(&modification_time.to_be_bytes()); // Modification time
        data.extend(&timescale.to_be_bytes()); // Timescale
        data.extend(&duration.to_be_bytes()); // Duration
        data.extend(&0x00010000u32.to_be_bytes()); // Preferred rate (1.0)
        data.extend(&0x0100u16.to_be_bytes()); // Preferred volume (1.0)
        data.extend(&vec![0u8; 10]); // Reserved

        // Transform matrix (9 x 4 bytes)
        for value in &matrix {
            data.extend(&value.to_be_bytes());
        }

        data.extend(&vec![0u8; 24]); // Pre-defined
        data.extend(&0x00000001u32.to_be_bytes()); // Next track ID

        Ok(())
    }

    /// Encode metadata container atom
    fn encode_meta_atom(
        &self,
        atom: &VideoAtomContainer,
        data: &mut Vec<u8>,
    ) -> Result<(), std::io::Error> {
        // meta atom structure for iTunes-style metadata
        data.extend(&[0x00, 0x00, 0x00, 0x00]); // Version and flags

        for tag in &atom.tags {
            data.extend(tag.value_as_u8_vec(&self.endian));
        }
        Ok(())
    }
}

// Implement VideoAtom trait for VideoAtomContainer
impl crate::video_metadata::metadata_trait::VideoAtom for VideoAtomContainer {
    fn get_atom_type(&self) -> u32 {
        self.atom_type
    }

    fn get_size(&self) -> u64 {
        self.size
    }
}

impl VideoAtomContainer {
    fn get_atom_priority(&self) -> u32 {
        match self.atom_type {
            FTYP => 0, // 'ftyp' - file type (highest priority)
            MOOV => 1, // 'moov' - movie container
            MVHD => 2, // 'mvhd' - movie header
            TRAK => 3, // 'trak' - track
            META => 4, // 'meta' - metadata
            _ => 999,  // Unknown atoms (lowest priority)
        }
    }

    /// Check if an atom is essential and should be kept during minimize
    fn is_essential_atom(&self) -> bool {
        matches!(
            self.atom_type,
            FTYP | // 'ftyp'
               MOOV | // 'moov'
               MVHD | // 'mvhd'
               TRAK // 'trak'
        )
    }

    fn get_tags(&self) -> &Vec<VideoTag> {
        &self.tags
    }
}
