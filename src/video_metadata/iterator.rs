use super::VideoMetadata;
use super::VideoTag;

impl<'a> IntoIterator for &'a VideoMetadata {
    type Item = &'a VideoTag;
    type IntoIter = VideoMetadataIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        VideoMetadataIterator {
            v_metadata: self,
            current_tag_index: 0,
        }
    }
}

pub struct VideoMetadataIterator<'a> {
    v_metadata: &'a VideoMetadata,
    current_tag_index: usize,
}

impl<'a> Iterator for VideoMetadataIterator<'a> {
    type Item = &'a VideoTag;

    fn next(&mut self) -> Option<Self::Item> {
        for atom in self.v_metadata.atoms.iter() {
            if self.current_tag_index < atom.get_tags().len() {
                self.current_tag_index += 1;
                return Some(&atom.get_tags()[self.current_tag_index - 1]);
            } else {
                self.current_tag_index = 0
            }
        }
        return None;
    }
}
