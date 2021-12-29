// perf.data struct definitions according to
// https://github.com/torvalds/linux/blob/master/tools/perf/Documentation/perf.data-file-format.txt
// and
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/perf_event.h

use std::mem;
use bitflags::bitflags;

#[derive(Default, Clone)]
#[repr(C)]
pub struct PerfFileSection {
    pub offset: u64,
    pub size: u64,
}

#[derive(Default, Clone)]
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

#[repr(u32)]
enum PerfEventType {
    PerfRecordMmap = 1,
    PerfRecordSample = 9,
}


#[derive(Default, Clone)]
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

// Bits that can be set in attr.sample_type
bitflags! {
    struct PerfEventSampleFormat: u64 {
        const PERF_SAMPLE_IP				= 1 << 0;
        const PERF_SAMPLE_TID				= 1 << 1;
        const PERF_SAMPLE_TIME			= 1 << 2;
        const PERF_SAMPLE_ADDR			= 1 << 3;
        const PERF_SAMPLE_READ			= 1 << 4;
        const PERF_SAMPLE_CALLCHAIN			= 1 << 5;
        const PERF_SAMPLE_ID				= 1 << 6;
        const PERF_SAMPLE_CPU				= 1 << 7;
        const PERF_SAMPLE_PERIOD			= 1 << 8;
        const PERF_SAMPLE_STREAM_ID			= 1 << 9;
        const PERF_SAMPLE_RAW				= 1 << 10;
        const PERF_SAMPLE_BRANCH_STACK		= 1 << 11;
        const PERF_SAMPLE_REGS_USER			= 1 << 12;
        const PERF_SAMPLE_STACK_USER			= 1 << 13;
        const PERF_SAMPLE_WEIGHT			= 1 << 14;
        const PERF_SAMPLE_DATA_SRC			= 1 << 15;
        const PERF_SAMPLE_IDENTIFIER			= 1 << 16;
        const PERF_SAMPLE_TRANSACTION			= 1 << 17;
        const PERF_SAMPLE_REGS_INTR			= 1 << 18;
        const PERF_SAMPLE_PHYS_ADDR			= 1 << 19;
        const PERF_SAMPLE_AUX				= 1 << 20;
        const PERF_SAMPLE_CGROUP			= 1 << 21;
        const PERF_SAMPLE_DATA_PAGE_SIZE		= 1 << 22;
        const PERF_SAMPLE_CODE_PAGE_SIZE		= 1 << 23;
        const PERF_SAMPLE_WEIGHT_STRUCT		= 1 << 24;
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct PerfEventAttr {
    major_type: u32,    // hardware, software, tracepoint, etc.
    pub size: u32,
    config: u64,
    sample_freq: u64,   // can also be sampling period
    sample_type: u64,
    read_format: u64,
    bitfield: u64,
    wakeup_events: u32, // or wakeup_watermark
    bp_type: u32,
    bp_addr: u64,       // union
    bp_len: u64,        // union
    branch_sample_type: u64,
    sample_regs_user: u64,
    sample_stack_user: u32,
    clockid: i32,
    sample_regs_intr: u64,
    aux_watermark: u32,
    sample_max_stack: u16,
    __reserved_2: u16,
    aux_sample_size: u32,
    __reserved_3: u32,
}

impl Default for PerfEventAttr {
    fn default() -> PerfEventAttr {
        PerfEventAttr {
            major_type: 1,  // PERF_TYPE_SOFTWARE
            size: 120,
            config: 9,  // PERF_COUNT_SW_DUMMY
            sample_freq: 4000,
            sample_type: (PerfEventSampleFormat::PERF_SAMPLE_IP
                | PerfEventSampleFormat::PERF_SAMPLE_TID
                | PerfEventSampleFormat::PERF_SAMPLE_ID).bits(),
            read_format: 0,
            bitfield: 0b00000001010011000100000000000000000000 << 26,
            wakeup_events: 100,
            bp_type: 0,
            bp_addr: 0,
            bp_len: 0,
            branch_sample_type: 0,
            sample_regs_user: 0,
            sample_stack_user: 0,
            clockid: 0,
            sample_regs_intr: 0,
            aux_watermark: 0,
            sample_max_stack: 0,
            __reserved_2: 0,
            aux_sample_size: 0,
            __reserved_3: 0,
        }
    }
}


#[derive(Clone)]
#[repr(C)]
pub struct RecordSample {
    pub header: PerfEventHeader,
    pub ip: u64,
    pub pid: u32,
    pub tid: u32,
    pub id: u64,
}

impl Default for RecordSample {
    fn default() -> RecordSample {
        RecordSample {
            header: PerfEventHeader { 
                event_type: 9,  // PERF_RECORD_SAMPLE
                misc: 0,
                size:  mem::size_of::<RecordSample>() as u16
            },
            ip: 0,
            pid: 0,
            tid: 0,
            id: 0,
        }
    }
}

const PATH_MAX: usize = 32;

#[derive(Clone)]
#[repr(C)]
pub struct RecordMmap {
    pub header: PerfEventHeader,
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: [u8; PATH_MAX],
    pub sample_id_pid: u32,
    pub sample_id_tid: u32,
    pub sample_id_id: u32,
}

impl Default for RecordMmap {
    fn default() -> RecordMmap {
        RecordMmap {
            header: PerfEventHeader { 
                event_type: 1,  // PERF_RECORD_Mmap
                misc: 0,
                size:  mem::size_of::<RecordMmap>() as u16
            },
            pid: 0,
            tid: 0,
            addr: 0,
            len: 0,
            pgoff: 0,
            filename: [b'A';  PATH_MAX],
            sample_id_pid: 0,
            sample_id_tid: 0,
            sample_id_id: 0
        }
    }
}

#[derive(Default, Clone)]
#[repr(C)]
pub struct File {
    header: PerfHeader,
}
