pub struct Region {
    pub chrom: String,
    pub start: Vec<u32>,
    pub end: Vec<u32>,
    pub name: String,
    pub class: String,
}

pub struct MethRegion {
    pub chrom: String,
    pub pos: u32,
    pub meth: u32,
    pub total: u32,
}

#[derive(Debug)]
pub enum MethFileType {
    AllCools,
    MethylDackel,
    //BismarkBedgraph,
    BismarkCov,
    BismarkCpGReport,
}

impl MethFileType {
    pub fn parse_line(&self, line: &str) -> Result<Option<MethRegion>, String> {
        match self {
            MethFileType::AllCools => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 7 {
                    return Err(format!("Invalid AllCools line: {}", line));
                }
                let chrom = fields[0].to_string();
                // Allcool files is 1-based. Convert to 0-based.
                let pos = fields[1].parse::<u32>().map_err(|e| format!("Invalid position in AllCools line: {}: {}", line, e))? - 1;
                let meth = fields[4].parse::<u32>().map_err(|e| format!("Invalid methylated count in AllCools line: {}: {}", line, e))?;
                let total = fields[5].parse::<u32>().map_err(|e| format!("Invalid total count in AllCools line: {}: {}", line, e))?;
                Ok(Some(MethRegion { chrom, pos, meth, total }))
            }
            MethFileType::MethylDackel => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 6 {
                    return Err(format!("Invalid MethylDackel line: {}", line));
                }
                let chrom = fields[0].to_string();
                let pos = fields[1].parse::<u32>().map_err(|e| format!("Invalid position in MethylDackel line: {}: {}", line, e))?;
                let meth = fields[4].parse::<u32>().map_err(|e| format!("Invalid methylated count in MethylDackel line: {}: {}", line, e))?;
                let total = fields[5].parse::<u32>().map_err(|e| format!("Invalid total count in MethylDackel line: {}: {}", line, e))?;
                Ok(Some(MethRegion { chrom, pos, meth, total }))
            }
            //MethFileType::BismarkBedgraph => {}
            MethFileType::BismarkCov => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 6 {
                    return Err(format!("Invalid BismarkCov line: {}", line));
                }
                let chrom = fields[0].to_string();
                // Bismark Coverage is 1-based by default. Even though 0-based can be specified.
                // We assume it's 1-based.
                let pos = fields[1].parse::<u32>().map_err(|e| format!("Invalid position in BismarkCov line: {}: {}", line, e))? - 1;
                let meth = fields[4].parse::<u32>().map_err(|e| format!("Invalid methylated count in BismarkCov line: {}: {}", line, e))?;
                let unmeth = fields[5].parse::<u32>().map_err(|e| format!("Invalid unmethylated count in BismarkCov line: {}: {}", line, e))?;
                let total = meth + unmeth;
                Ok(Some(MethRegion { chrom, pos, meth, total }))
            }
            MethFileType::BismarkCpGReport => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 7 {
                    return Err(format!("Invalid BismarkCpGReport line: {}", line));
                }
                let chrom = fields[0].to_string();
                // Bismark CpG Report is 0-based.
                let pos = fields[1].parse::<u32>().map_err(|e| format!("Invalid position in BismarkCpGReport line: {}: {}", line, e))?;
                let meth = fields[3].parse::<u32>().map_err(|e| format!("Invalid methylated count in BismarkCpGReport line: {}: {}", line, e))?;
                let unmeth = fields[4].parse::<u32>().map_err(|e| format!("Invalid unmethylated count in BismarkCpGReport line: {}: {}", line, e))?;
                if meth != 0 || unmeth != 0 {
                    let total = meth + unmeth;
                    Ok(Some(MethRegion { chrom, pos, meth, total }))    
                } else {
                    Ok(None)
                }
            }
        }
    }
}