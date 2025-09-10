use crate::reader::parse_region;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_region() {
        // get path to test data relative to test code.
        // This ensure cargo test can be ran from anywhere.
        let test_path = Path::new(file!());
        let bedfile = test_path.parent().unwrap().join("data/region.bed");
        let bedgzfile = test_path.parent().unwrap().join("data/region.bed.gz");
        
        // Parse bed file
        let bedregions = parse_region(bedfile.to_string_lossy().into_owned(), "bed".to_string());
        let bedgzregions = parse_region(bedgzfile.to_string_lossy().into_owned(), "bedgz".to_string());
        // bed file.
        assert_eq!(bedregions.len(), 1);
        assert_eq!(bedregions[0].chrom, "chr1");
        assert_eq!(bedregions[0].start, vec![100]);
        assert_eq!(bedregions[0].end, vec![200]);
        assert_eq!(bedregions[0].name, "chr1:100-200");
        assert_eq!(bedregions[0].class, "bed");
        // bed gz file.
        assert_eq!(bedgzregions.len(), 1);
        assert_eq!(bedgzregions[0].chrom, "chr1");
        assert_eq!(bedgzregions[0].start, vec![100]);
        assert_eq!(bedgzregions[0].end, vec![200]);
        assert_eq!(bedgzregions[0].name, "chr1:100-200");
        assert_eq!(bedgzregions[0].class, "bedgz");
    }
}
