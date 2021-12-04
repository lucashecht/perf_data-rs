// perf.data struct definitions according to
// https://github.com/torvalds/linux/blob/master/tools/perf/Documentation/perf.data-file-format.txt

struct perf_file_section {
    offset: u64,
    size: u64,
}

struct perf_header {
    magic: [char; 8],
    size: u64,
    attr_size: u64,
    attrs: perf_file_section,
    data: perf_file_section,
    event_types: perf_file_section,
    flags: u64,
    flags1: [u64; 3],
}

struct perf_event_header {
    event_type: u32,
    misc: u16,
    size: u16,
}
