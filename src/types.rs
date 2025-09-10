pub struct Region {
    pub chrom: String,
    pub start: Vec<u32>,
    pub end: Vec<u32>,
    pub name: String,
    pub class: String,
}

pub struct CoolRegion {
    pub chrom: String,
    pub pos: u32,
    pub meth: u32,
    pub total: u32,
}