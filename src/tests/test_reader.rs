use crate::reader::{parse_region, is_gzipped, decide_methtype, read_meth, parse_chromsizes};
use crate::types::{MethFileType, MethRegion};
use std::path::Path;
use std::io::BufRead;


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

    #[test]
    fn test_is_gzipped() {
        let test_path = Path::new(file!());
        let bedfile = test_path.parent().unwrap().join("data/region.bed");
        let bedgzfile = test_path.parent().unwrap().join("data/region.bed.gz");
        assert_eq!(is_gzipped(&bedfile.to_string_lossy()).unwrap(), false);
        assert_eq!(is_gzipped(&bedgzfile.to_string_lossy()).unwrap(), true);
    }

    #[test]
    fn test_decide_methtype() {
        let test_path = Path::new(file!());
        let allcoolsf = test_path.parent().unwrap().join("data/methf_allcools");
        let bismarkcov = test_path.parent().unwrap().join("data/methf_bismarkcov");
        let cpgrep = test_path.parent().unwrap().join("data/methf_cpgrep");
        let methyldackel = test_path.parent().unwrap().join("data/methf_methyldackel");
        let pairs: Vec<(&str, MethFileType)> = vec![
            (allcoolsf.to_str().unwrap(), MethFileType::AllCools),
            (bismarkcov.to_str().unwrap(), MethFileType::BismarkCov),
            (cpgrep.to_str().unwrap(), MethFileType::BismarkCpGReport),
            (methyldackel.to_str().unwrap(), MethFileType::MethylDackel),
        ];
        for (f, expected) in pairs {
            let reader = std::io::BufReader::new(std::fs::File::open(f).unwrap());
            let firstline = reader.lines()
                .filter_map(|l| l.ok())
                .find(|line| !line.trim_start().starts_with('#'));
            let methtype = decide_methtype(firstline);
            assert_eq!(methtype, expected);
        }
    }

    #[test]
    fn test_read_meth() {
        let test_path = Path::new(file!());
        let allcoolsf = test_path.parent().unwrap().join("data/methf_allcools");
        let bismarkcov = test_path.parent().unwrap().join("data/methf_bismarkcov");
        let cpgrep = test_path.parent().unwrap().join("data/methf_cpgrep");
        let methyldackel = test_path.parent().unwrap().join("data/methf_methyldackel");

        let exp_mr = vec![
            MethRegion { chrom: "chr1".to_string(), pos: 0, meth: 1, total: 1 },
            MethRegion { chrom: "chr1".to_string(), pos: 2, meth: 0, total: 1 },
        ];

        for f in vec![
            allcoolsf.to_str().unwrap(),
            bismarkcov.to_str().unwrap(),
            cpgrep.to_str().unwrap(),
            methyldackel.to_str().unwrap(),
        ] {
            let methregions = read_meth(f);
            assert_eq!(methregions, exp_mr);
        }
    }

    #[test]
    fn test_parse_chromsizes() {
        let test_path = Path::new(file!());
        let chromsizef = test_path.parent().unwrap().join("data/chromsizes.txt");

        let regions = parse_chromsizes(&chromsizef.to_string_lossy(), 1000);
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[1].chrom, "chr1");
        assert_eq!(regions[1].start, vec![1000]);
        assert_eq!(regions[1].end, vec![2000]);
        assert_eq!(regions[1].name, "chr1:1000-2000");
        assert_eq!(regions[1].class, "bin");
    }
}
