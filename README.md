# illumina_coordinates

[![Build Status](https://travis-ci.org/jimrybarski/illumina-coordinates.svg?branch=master)](https://travis-ci.org/jimrybarski/illumina-coordinates)

In FASTQ files created by the Illumina sequencing process, the sequence identifiers contain the coordinates of the DNA
cluster that produced the sequence (among other things). This library parses the identifiers.

### Usage

```rust
extern crate illumina_coordinates;

fn main() {
    let line = "@M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0";
    let seq_id = illumina_coordinates::parse_sequence_identifier(&line).unwrap();
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
    assert_eq!(seq_id.sample, illumina_coordinates::Sample::Number(0));
}
```

### Description of Fields

Take this example sequence identifier:

`@M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0`

| Value | Meaning |
| --- | --- |
| `M03745` | ID of the sequencing machine |
| `11` | run count for this machine |
| `000000000-B54L5` | ID of the flow cell. "B54L5" will be printed on the flow cell in this example |
| `1` | lane number. For MiSeqs, there's only one lane |
| `2` from `2108` | the side of the chip |
| `1` from `2108` | the swath (for MiSeqs, this is always 1. For HiSeqs, each lane is two tiles wide, and the first pass from left-to-right is swath one, then the returning pass on the other side of the lane is swath two |
| `08` from `2108` | the tile number. For MiSeqs, this is a number from 1 to 19 |
| `4127` | the x-position of the read in the tile, in arbitrary units |
| `8949` | the y-position of the read in the tile, in arbitrary units |
| `1`    | first (forward) read in a paired-end run |
| `N`    | Read was not filtered (sufficient quality) |
| `0`    | This was not a control |
| `0`    | This was the first sample on the sample sheet |

See [https://help.basespace.illumina.com/articles/descriptive/fastq-files/](https://help.basespace.illumina.com/articles/descriptive/fastq-files/) for more information.
