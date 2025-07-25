use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::endian::Endian;
use crate::general_file_io::*;
use crate::metadata::Metadata;
use crate::u8conversion::*;

const MOV_ENDIAN: Endian = Endian::Big;
// MOV signature: 'ftyp' atom typically at offset 4
const MOV_SIGNATURES: &[&[u8]] = &[b"ftyp", b"moov", b"mdat", b"wide"];

// Common MOV brands
const MOV_BRANDS: &[&[u8]] = &[b"qt  ", b"mp41", b"mp42", b"isom", b"M4V ", b"M4A "];

// Common

#[derive(Debug, Clone)]
struct Atom {
    size: u32,
    atom_type: [u8; 4],
    data_offset: u64,
    data_size: u32,
}

impl Atom {
    fn read_from<R: Read + Seek>(reader: &mut R) -> Result<Self, std::io::Error> {
        let start_pos = reader.stream_position()?;

        //Read size (4 bytes, big-endian)
        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;
        let size = from_u8_vec_macro!(u32, &size_buf.to_vec(), &MOV_ENDIAN);

        //Read atom type (4 bytes)
        let mut type_buf = [0u8; 4];
        reader.read_exact(&mut type_buf)?;

        if size < 8 {
            return io_error!(InvalidData, "Invalid atom size");
        }

        Ok(Atom {
            size,
            atom_type: type_buf,
            data_offset: start_pos + 8,
            data_size: size - 8,
        })
    }
}

pub fn read_metadata(data: &Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let mut reader = Cursor::new(data);

    if !check_file_signature(&mut reader)? {
        return io_error!(InvalidData, "Not a MOV file, or unsupported brand.");
    }

    // Reset cursor to the beginning of the buffer after check
    reader.seek(SeekFrom::Start(0))?;

    // Get buffer size to know when to stop reading atoms
    let buf_size = data.len() as u64;

    while reader.position() < buf_size {
        let atom_start_pos = reader.position();
        let atom = match Atom::read_from(&mut reader) {
            Ok(atom) => atom,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // This is a common, expected error when we reach the end of the buffer
                    // while trying to read an atom header. We can just stop.
                    break;
                }
                return io_error!(
                    InvalidData,
                    format!("Error reading atom at position {}: {}", atom_start_pos, e)
                );
                // break;
            }
        };

        if &atom.atom_type == b"moov" {
            let mut moov_data = vec![0u8; atom.data_size as usize];
            reader.read_exact(&mut moov_data)?;
            return Ok(moov_data);
        }

        // Seek to the beginning of the next atom.
        let next_atom_pos = atom_start_pos + atom.size as u64;
        if next_atom_pos > buf_size {
            // Next atom position is beyond buffer size
            break;
        }
        reader.seek(SeekFrom::Start(next_atom_pos))?;
    }

    io_error!(NotFound, "'moov' atom is not found.")
}

/// Checks if the file is a supported MOV/MP4 format.
fn check_file_signature<R: Read + Seek>(reader: &mut R) -> Result<bool, std::io::Error> {
    // Go to beginning of the stream to be safe.
    reader.seek(SeekFrom::Start(0))?;

    // The 'ftyp' atom should be the first atom after the 4-byte size.
    let mut size_buf = [0u8; 4];
    if reader.read_exact(&mut size_buf).is_err() {
        return Ok(false); // Data too small
    }

    let mut type_buf = [0u8; 4];
    if reader.read_exact(&mut type_buf).is_err() {
        return Ok(false); // Data too small
    }

    if &type_buf != b"ftyp" {
        return Ok(false);
    }

    // Now check the major brand inside 'ftyp'
    let mut brand_buf = [0u8; 4];
    if reader.read_exact(&mut brand_buf).is_err() {
        return Ok(false); // 'ftyp' atom is too small
    }

    if MOV_BRANDS.contains(&&brand_buf[..]) || &brand_buf == b"qt  " {
        return Ok(true);
    }

    println!(
        "Unrecognized MOV brand: {}",
        String::from_utf8_lossy(&brand_buf)
    );

    Ok(false)
}

mod tests {
    use super::*;
    use std::fs::read;

    #[test]
    fn test_is_mov() {
        let mut file = File::open("test.mov").unwrap();
        assert!(check_file_signature(&mut file).unwrap());
    }

    #[test]
    fn test_read_metadata() {
        let mut file = read("/Users/rodion-stetsurin/Desktop/pets/test.mov").unwrap();
        let metadata = read_metadata(&mut file).unwrap();
        // assert_eq!(metadata.width, 1920);
        // assert_eq!(metadata.height, 1080);
    }
}
