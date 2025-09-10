use std::fs::File;
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;
use crate::types::CoolRegion;
use crate::types::{Region};

pub fn read_meth(_f: &str) -> Vec<CoolRegion> {
    let mut coolregions: Vec<CoolRegion> = Vec::new();
    let reader = BufReader::new(GzDecoder::new(File::open(_f).unwrap()));
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split('\t').collect();
                let chrom = fields[0].to_string();
                let pos = fields[1].parse::<u32>().unwrap();
                let meth = fields[4].parse::<u32>().unwrap();
                let total = fields[5].parse::<u32>().unwrap();
                coolregions.push(
                    CoolRegion{
                        chrom,
                        pos,
                        meth,
                        total,
                    }
                );
            }
            Err(_e) => {
                panic!("Error reading file {}", _f);
            }
        }
    }
    coolregions
}

pub fn parse_chromsizes(file: &str, binsize: u32) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let reader = BufReader::new(File::open(file).unwrap());
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split('\t').collect();
                let chrom = fields[0].to_string();
                let chromsize = fields[1].parse::<u32>().unwrap();
                let mut start = 0;
                let mut end = start + binsize;
                while end < chromsize {
                    regions.push(
                        Region{
                            chrom: chrom.clone(),
                            start: vec![start],
                            end: vec![end],
                            name: format!("{}:{}-{}", chrom, start, end),
                            class: "bin".to_string(),
                        }
                    );
                    start = end;
                    end += binsize;
                    // Capture chromosome end
                    if end >= chromsize {
                        regions.push(
                            Region{
                                chrom: chrom.clone(),
                                start: vec![start],
                                end: vec![chromsize],
                                name: format!("{}:{}-{}", chrom, start, end),
                                class: "bin".to_string(),
                            }
                        );
                    }
                }
            },
            Err(_e) => {
                panic!("Error reading file {}", file);
            }
        }
    }
    regions
}

pub fn parse_region(reg: String, class: String) -> Vec<Region> {
    let mut regions = Vec::new();
    let sample = reg.clone();

    // Get suffix from reg
    let suffix = reg.split('.').next_back().unwrap();
    // Two options: gz (bed.gz), bed(bed)
    let reader: Box<dyn BufRead> = match suffix {
        "gz" => {
            println!("Region parse match gz");
            Box::new(BufReader::new(GzDecoder::new(File::open(reg).unwrap())))
        },
        "bed" => {
            Box::new(BufReader::new(File::open(reg).unwrap()))
        },
        _ => panic!("File format not supported"),
    };

    let lines = reader.lines();
    for line in lines {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split('\t').collect();
                let chrom = fields[0].to_string();
                let start = fields[1].to_string();
                let end = fields[2].to_string();
                let name: String = if fields.len() > 3 {
                    fields[3].to_string()
                } else {
                    format!("{}:{}-{}", chrom, start, end)
                };
                // check if start, end have commas
                if start.contains(",") {
                    let start: Vec<u32> = start.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    let end: Vec<u32> = end.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    regions.push(
                        Region{
                            chrom,
                            start,
                            end,
                            name,
                            class: class.to_string()
                        }
                    );
                } else {
                    let start = start.parse::<u32>().unwrap();
                    let end = end.parse::<u32>().unwrap();
                    regions.push(
                        Region{
                            chrom,
                            start: vec![start],
                            end: vec![end],
                            name,
                            class: class.to_string()
                        }
                    );
                }

            }
            Err(_e) => {
                panic!("Error reading file {}", sample);
            }
        }
    }
    regions
}