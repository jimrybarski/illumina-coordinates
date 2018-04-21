use std::convert::From;
use std::result::Result;
use std::num;

pub type SequenceIdentifier = (String, u16, String, u8, u8, u8, u8, u16, u16);

#[derive(Debug)]
pub enum IlluminaError {
    ParseIntError,
    SplitError
}

impl From<num::ParseIntError> for IlluminaError {
    fn from(_: num::ParseIntError) -> IlluminaError {
        IlluminaError::ParseIntError
    }
}

pub fn parse_sequence_identifier(text: &str) -> Result<SequenceIdentifier, IlluminaError> {
    // Parses location information from an Illumina sequence identifier. This implementation is
    // about 3x faster than using a regular expression.
    //
    // Take this example identifier:
    // @M03745:11:000000000-B54L5:1:2108:4127:8949
    //
    // M03745              ID of the sequencing machine
    //
    // 11                  run count for this machine
    //
    // 000000000-B54L5     ID of the flow cell. "B54L5" will be printed on the flow cell in this example
    //
    // 1                   lane number. For MiSeqs, there's only one lane
    //
    // 2108                the first digit is the side of the chip
    //
    //                     the second digit is the swath (for MiSeqs, this is always 1. For HiSeqs, each lane is two tiles
    //                     wide, and the first pass from left-to-right is swath one, then the returning pass on the other
    //                     side of the lane is swath two
    //
    //                     the last two digits are the order of the tile. For MiSeqs, this is a number from 1 to 19
    //
    // 4127                the x-position of the read in the tile, in arbitrary units
    //
    // 8949                the y-position of the read in the tile, in arbitrary units
    //
    // See https://help.basespace.illumina.com/articles/descriptive/fastq-files/ for more information.
    let cap: Vec<&str> = text.split(":").collect();
    if cap.len() != 7 {
        return Err(IlluminaError::SplitError);
    }
    let sequencer_id = cap[0].split_at(1).1.to_string();
    let run_number = cap[1].parse::<u16>()?;
    let flow_cell_id = cap[2].to_string();
    let lane = cap[3].parse::<u8>()?;
    let (side, remainder) = cap[4].split_at(1);
    let (swath, tile_number) = remainder.split_at(1);
    let side = side.parse::<u8>()?;
    let swath = swath.parse::<u8>()?;
    let tile_number = tile_number.parse::<u8>()?;
    let x = cap[5].parse::<u16>()?;
    let y = cap[6].parse::<u16>()?;
    return Ok((sequencer_id, run_number, flow_cell_id, lane, side, swath, tile_number, x, y))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (sequencer_id, run_number, flow_cell_id, lane, side, swath, tile_number, x, y) = parse_sequence_identifier("@M03745:11:000000000-B54L5:1:2108:4127:8949").unwrap();
        assert_eq!(sequencer_id, "M03745".to_string());
        assert_eq!(run_number, 11);
        assert_eq!(flow_cell_id, "000000000-B54L5".to_string());
        assert_eq!(lane, 1);
        assert_eq!(side, 2);
        assert_eq!(swath, 1);
        assert_eq!(tile_number, 8);
        assert_eq!(x, 4127);
        assert_eq!(y, 8949);
    }

    #[test]
    fn test_parse_error() {
        let result = parse_sequence_identifier("CACGACGACTAGCTACGGACGCGGCACGACGCAG");
        assert!(result.is_err());
    }
}
