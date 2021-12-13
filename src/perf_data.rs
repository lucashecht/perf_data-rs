// perf.data struct definitions according to
// https://github.com/torvalds/linux/blob/master/tools/perf/Documentation/perf.data-file-format.txt

#[derive(Debug)]
#[repr(C)]
pub struct PerfFileSection {
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct PerfHeader {
    pub magic: [u8; 8],
    pub size: u64,
    pub attr_size: u64,
    pub attrs: PerfFileSection,
    pub data: PerfFileSection,
    pub event_types: PerfFileSection,
    pub flags: u64,
    pub flags1: [u64; 3],
}

#[repr(C)]
pub struct PerfEventHeader {
    pub event_type: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct BuildIdEvent {
    pub header: PerfEventHeader,
    pub pid: i32,
    pub build_id: [u8; 24],
    pub filename: [char],
}


#[repr(C)]
pub struct IdIndexEntry {
    pub id: u64,
    pub idx: u64,
    pub cpu: u64,
    pub tid: u64,
}

#[repr(C)]
pub struct IdIndexEvent {
    pub header: PerfEventHeader,
    pub nr: u64,
    pub entries: [IdIndexEntry],
}
