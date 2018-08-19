#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    extern crate illumina_coordinates;
    use test::Bencher;

    #[bench]
    fn bench_parse_sequence_identifier(b: &mut Bencher) {
        let sequence_identifier = "@M03745:11:000000000-B54L5:1:2108:4127:8949 1:N:0:0";
        b.iter(|| illumina_coordinates::parse_sequence_identifier(&sequence_identifier).unwrap());
    }
}
