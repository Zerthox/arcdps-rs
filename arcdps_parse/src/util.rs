use crate::ParseError;
use byteorder::LittleEndian;
use std::io;

/// Endianness.
///
/// EVTC logs will be written on Windows and Windows uses little endian.
pub type Endian = LittleEndian;

/// Reads a fixed amount of bytes from the input into a buffer.
pub fn read_buffer<const SIZE: usize>(input: &mut impl io::Read) -> io::Result<[u8; SIZE]> {
    let mut buffer = [0; SIZE];
    input.read_exact(&mut buffer)?;
    Ok(buffer)
}

/// Reads a UTF-8 string from a char buffer.
pub fn read_string_buffer<const SIZE: usize>(
    input: &mut impl io::Read,
) -> Result<String, ParseError> {
    let buffer = read_buffer::<SIZE>(input)?;
    Ok(String::from_utf8(buffer.to_vec())?)
}

/// Reads a UTF-8 string from a char buffer.
pub fn write_string_buffer<const SIZE: usize>(
    output: &mut impl io::Write,
    string: impl AsRef<str>,
) -> Result<(), io::Error> {
    let bytes = string.as_ref().as_bytes();
    let mut buffer = [0; SIZE];
    buffer[..bytes.len()].copy_from_slice(bytes);
    output.write_all(&buffer)?;
    Ok(())
}
