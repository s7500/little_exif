use super::video_tag::VideoTag;
use crate::endian::Endian;
use std::io::Cursor;

// Video-specific metadata
pub trait VideoAtom {
    fn get_atom_type(&self) -> u32;
    fn get_size(&self) -> u64;
}

pub trait VMetadata: Sized {
    type AtomType: VideoAtom;
    type TagIterator<'a>: Iterator<Item = &'a VideoTag>
    where
        Self: 'a;

    // Core construction
    fn new() -> Self;

    // Create video containers (equivalent to create_ifd for images)
    fn create_atom(&mut self, atom_type: u32);

    // Decoding wrapper following the same pattern as image metadata
    fn general_decoding_wrapper(
        raw_pre_decode_general: Result<Vec<u8>, std::io::Error>,
    ) -> Result<Self, std::io::Error>;

    // Encoding
    fn encode(&self) -> Result<Vec<u8>, std::io::Error>;

    // Internal data management
    fn sort_data(&mut self);
    fn decode(
        data_cursor: &mut Cursor<&Vec<u8>>,
    ) -> Result<(Endian, Vec<Self::AtomType>), std::io::Error>;

    // Editing operations
    fn reduce_to_minimum(&mut self);

    // Getters - following the same patterns as image metadata
    fn get_endian(&self) -> Endian;
    fn get_atoms(&self) -> &Vec<Self::AtomType>;
    fn get_atom(&self, atom_type: u32) -> Option<&Self::AtomType>;
    fn get_atom_mut(&mut self, atom_type: u32) -> &mut Self::AtomType;

    // Tag operations with iterator pattern like image metadata
    fn get_tag(&self, tag: &VideoTag) -> Self::TagIterator<'_>;
    fn get_tag_by_id(&self, tag_id: u32, atom_type: Option<u32>) -> Self::TagIterator<'_>;

    // Setting tags
    fn set_tag(&mut self, tag: VideoTag);
}
