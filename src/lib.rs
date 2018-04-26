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
    pub y: u16
}

#[derive(Debug)]
/// Errors encountered when parsing FASTQ files
pub enum IlluminaError {
    /// We expected an integer but did not find one
    ParseIntError,
    /// The line was not structured as expected
    SplitError
}

impl From<num::ParseIntError> for IlluminaError {
    fn from(_: num::ParseIntError) -> IlluminaError {
        IlluminaError::ParseIntError
    }
}

/// Parses location information from an Illumina sequence identifier. This implementation is
/// about 3x faster than using a regular expression.
///
/// The fields in the example identifier below have the following meaning:
/// @M03745:11:000000000-B54L5:1:2108:4127:8949
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
/// See https://help.basespace.illumina.com/articles/descriptive/fastq-files/ for more information.
///
/// # Example
///
/// ```rust
/// extern crate illumina_coordinates;
///
/// fn main() {
///     let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949";
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
/// }
/// ```
pub fn parse_sequence_identifier(text: &str) -> Result<SequenceIdentifier, IlluminaError> {
    let cap: Vec<&str> = text.trim().split(':').collect();
    if cap.len() != 7 {
        return Err(IlluminaError::SplitError);
    }
    let sequencer_id = cap[0].split_at(1).1.to_string();
    let run_count = cap[1].parse::<u16>()?;
    let flow_cell_id = cap[2].to_string();
    let lane = cap[3].parse::<u8>()?;
    let (side, remainder) = cap[4].split_at(1);
    let (swath, tile) = remainder.split_at(1);
    let side = side.parse::<u8>()?;
    let swath = swath.parse::<u8>()?;
    let tile = tile.parse::<u8>()?;
    let x = cap[5].parse::<u16>()?;
    let y = cap[6].parse::<u16>()?;
    Ok(SequenceIdentifier {
        sequencer_id,
        run_count,
        flow_cell_id,
        lane,
        side,
        swath,
        tile,
        x,
        y
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949";
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
    }


    #[test]
    fn test_parse_with_newline() {
        let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949\n";
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
    }

    #[test]
    fn test_parse_error() {
        let result = parse_sequence_identifier("CACGACGACTAGCTACGGACGCGGCACGACGCAG");
        assert!(result.is_err());
    }
}
