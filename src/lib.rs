//! # illumina_coordinates
//!
//! This crate provides a single function to parse sequence identifiers from FASTQ files created
//! by Illumina sequencers. Sequence identifiers contain information about each read, including
//! the physical location of the DNA cluster on the flow cell surface that contained the
//! associated sequence.
//!
//! Illumina was not involved in the creation of this library in any way.

#![crate_type="lib"]
#![deny(warnings, missing_docs)]
use std::convert::From;
use std::result::Result;
use std::num;


#[derive(Debug, PartialOrd, PartialEq)]
/// Sample numbers are either the number from the sample sheet or a sequence if the read was from
/// the Undetermined Reads
pub enum Sample {
    /// Sample number
    Number(u8),
    /// Sequence from Undetermined Reads
    Sequence(String)
}

/// A parsed sequence identifier
pub struct SequenceIdentifier {
    /// ID of the sequencing machine
    pub sequencer_id: String,
    /// The number of sequencing runs this machine has performed
    pub run_count: u16,
    /// ID of the flow cell, printed on the side of the glass slide
    pub flow_cell_id: String,
    /// Lane number. For MiSeqs, this is always 1
    pub lane: u8,
    /// The near or far side off the flow cell surface
    pub side: u8,
    /// The row within a lane, if wide enough. For MiSeqs, this is always 1
    pub swath: u8,
    /// The positional order of the region where the cluster is located
    pub tile: u8,
    /// The x-coordinate of the cluster
    pub x: u16,
    /// The y-coordinate of the cluster
    pub y: u16,
    /// The read number
    pub read: u8,
    /// Whether the read was filtered for low quality (Y=filtered)
    pub is_filtered: bool,
    /// Indicates the type of control, 0 = not a control read
    pub control_number: u8,
    /// Number from sample sheet, or the sequence if the read is in Undetermined Reads
    pub sample: Sample
}

#[derive(Debug)]
/// Errors encountered when parsing FASTQ files
pub enum IlluminaError {
    /// We expected an integer but did not find one
    ParseError,
    /// The line was not structured as expected
    SplitError
}

impl From<num::ParseIntError> for IlluminaError {
    fn from(_: num::ParseIntError) -> IlluminaError {
        IlluminaError::ParseError
    }
}

/// Parses location information from an Illumina sequence identifier. This implementation is
/// about 3x faster than using a regular expression.
///
/// The fields in the example identifier below have the following meaning:
/// @M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0
///
/// M03745              ID of the sequencing machine
///
/// 11                  run count for this machine
///
/// 000000000-B54L5     ID of the flow cell. "B54L5" will be printed on the flow cell in this example
///
/// 1                   lane number. For MiSeqs, there's only one lane
///
/// 2108                the first digit is the side of the chip
///                     the second digit is the swath. For MiSeqs, this is always 1. For HiSeqs, each lane is two tiles
///                     wide, and the first pass from left-to-right is swath one, then the returning pass on the other
///                     side of the lane is swath two
///                     the last two digits are the order of the tile. For MiSeqs, this is a number from 1 to 19
///
/// 4127                the x-position of the read in the tile, in arbitrary units
///
/// 8949                the y-position of the read in the tile, in arbitrary units
///
/// 1                   First (forward) read in a paired-end run
///
/// N                   Read was not filtered (sufficient quality)
///
/// 0                   This was not a control
///
/// 0                   This was the first sample on the sample sheet
///
/// See https://help.basespace.illumina.com/articles/descriptive/fastq-files/ for more information.
///
/// # Example
///
/// ```rust
/// extern crate illumina_coordinates;
/// use illumina_coordinates::Sample;
///
/// fn main() {
///     let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0";
///     let seq_id = illumina_coordinates::parse_sequence_identifier(&line).unwrap();
///     assert_eq!(seq_id.sequencer_id, "M03745".to_string());
///     assert_eq!(seq_id.run_count, 11);
///     assert_eq!(seq_id.flow_cell_id, "000000000-B54L5".to_string());
///     assert_eq!(seq_id.lane, 1);
///     assert_eq!(seq_id.side, 2);
///     assert_eq!(seq_id.swath, 1);
///     assert_eq!(seq_id.tile, 8);
///     assert_eq!(seq_id.x, 4127);
///     assert_eq!(seq_id.y, 8949);
///     assert_eq!(seq_id.read, 1);
///     assert_eq!(seq_id.is_filtered, false);
///     assert_eq!(seq_id.control_number, 0);
///     assert_eq!(seq_id.sample, Sample::Number(0));
/// }
/// ```
pub fn parse_sequence_identifier(text: &str) -> Result<SequenceIdentifier, IlluminaError> {
    let halves: Vec<&str> = text.trim().split(' ').collect();
    if halves.len() != 2 {
        return Err(IlluminaError::SplitError)
    }
    let left: Vec<&str> = halves[0].split(':').collect();
    let right: Vec<&str> = halves[1].split(':').collect();
    if left.len() != 7 {
        return Err(IlluminaError::SplitError);
    }
    if right.len() != 4 {
        return Err(IlluminaError::SplitError);
    }
    let sequencer_id = left[0].split_at(1).1.to_string();
    let run_count = left[1].parse::<u16>()?;
    let flow_cell_id = left[2].to_string();
    let lane = left[3].parse::<u8>()?;
    let (side, remainder) = left[4].split_at(1);
    let (swath, tile) = remainder.split_at(1);
    let side = side.parse::<u8>()?;
    let swath = swath.parse::<u8>()?;
    let tile = tile.parse::<u8>()?;
    let x = left[5].parse::<u16>()?;
    let y = left[6].parse::<u16>()?;

    let read = right[0].parse::<u8>()?;
    let is_filtered = match right[1] {
        "Y" => true,
        "N" => false,
        _ => return Err(IlluminaError::ParseError)
    };
    let control_number= right[2].parse::<u8>()?;
    let sample = right[3].parse::<u8>();
    let sample = match sample {
        Ok(n) => Sample::Number(n),
        Err(_) => Sample::Sequence(String::from(right[3]))
    };

    Ok(SequenceIdentifier {
        sequencer_id,
        run_count,
        flow_cell_id,
        lane,
        side,
        swath,
        tile,
        x,
        y,
        read,
        is_filtered,
        control_number,
        sample
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0";
        let seq_id = parse_sequence_identifier(&line).unwrap();
        assert_eq!(seq_id.sequencer_id, "M03745".to_string());
        assert_eq!(seq_id.run_count, 11);
        assert_eq!(seq_id.flow_cell_id, "000000000-B54L5".to_string());
        assert_eq!(seq_id.lane, 1);
        assert_eq!(seq_id.side, 2);
        assert_eq!(seq_id.swath, 1);
        assert_eq!(seq_id.tile, 8);
        assert_eq!(seq_id.x, 4127);
        assert_eq!(seq_id.y, 8949);
        assert_eq!(seq_id.read, 1);
        assert_eq!(seq_id.is_filtered, false);
        assert_eq!(seq_id.control_number, 0);
        assert_eq!(seq_id.sample, Sample::Number(0));
    }
    
    #[test]
    fn test_parse_with_newline() {
        let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949 1:Y:0:0\n";
        let seq_id = parse_sequence_identifier(&line).unwrap();
        assert_eq!(seq_id.sequencer_id, "M03745".to_string());
        assert_eq!(seq_id.run_count, 11);
        assert_eq!(seq_id.flow_cell_id, "000000000-B54L5".to_string());
        assert_eq!(seq_id.lane, 1);
        assert_eq!(seq_id.side, 2);
        assert_eq!(seq_id.swath, 1);
        assert_eq!(seq_id.tile, 8);
        assert_eq!(seq_id.x, 4127);
        assert_eq!(seq_id.y, 8949);
        assert_eq!(seq_id.read, 1);
        assert_eq!(seq_id.is_filtered, true);
        assert_eq!(seq_id.control_number, 0);
        assert_eq!(seq_id.sample, Sample::Number(0));
    }

    #[test]
    fn test_parse_error() {
        let result = parse_sequence_identifier("CACGACGACTAGCTACGGACGCGGCACGACGCAG");
        assert!(result.is_err());
    }
}
