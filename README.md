# illumina-coordinates

[![Build Status](https://travis-ci.org/jimrybarski/illumina-coordinates.svg?branch=master)](https://travis-ci.org/jimrybarski/illumina-coordinates)

In FASTQ files created by the Illumina sequencing process, the sequence identifiers contain the coordinates of the DNA
cluster that produced the sequence. This library parses the identifiers.

### Usage

```
extern crate illumina_coordinates;
use illumina_coordinates::parse_sequence_identifier;

let sequence_identifier = "@M03745:11:000000000-B54L5:1:2108:4127:8949";
let (sequencer_id, run_number, flow_cell_id, lane, side, swath, tile_number, x, y) = parse_sequence_identifier(&sequence_identifier).unwrap();

```

### Description of Fields

Take this example sequence identifier:

`@M03745:11:000000000-B54L5:1:2108:4127:8949`

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

See [https://help.basespace.illumina.com/articles/descriptive/fastq-files/](https://help.basespace.illumina.com/articles/descriptive/fastq-files/) for more information.
