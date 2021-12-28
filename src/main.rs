use std::process::{Command, Child, Stdio};
use std::io::{BufRead, BufReader, Error};
use std::fs::{File};
use std::io::prelude::*;
use std::mem;

mod perf_data;

fn main() -> Result<(), Error> {
    //let samples = collect_samples();

    let samples: Vec<i64> = vec![0x1111, 0x1118, 0x111a, 0x111b];
    create_perf_file(samples);

    Ok(())
}

fn run_qemu() -> Child {
     Command::new("qemu-system-x86_64")
        .args(["-m", "2048", "-M", "q35", "-cpu", "host,+vmx", "-enable-kvm", "-smp", "cores=4,threads=1,sockets=1"])
        .args(["-serial", "stdio", "-k", "en-us"])
        .args(["-kernel", "/home/lhecht/Downloads/bender"])
        .args(["-initrd", "/home/lhecht/git/supernova-core/build/share/hedron/hypervisor novga serial spinner,/home/lhecht/git/supernova-core/build/test/roottask/thesis-impl_roottasktest"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute QEMU")
}

fn collect_samples() -> Vec<i64> {
    let mut proc = run_qemu();
    let stdout = proc.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines().filter_map(|line| line.ok());
    let mut samples = Vec::new();

    while lines.next().unwrap().find("Start profiling").is_none() {}

    println!("Profiling started");
    for line in lines {
        if line.find("Stop profiling").is_some() {
            println!("Profiling stopped");
            break;
        } else if line.find("IP: ").is_some() {
            let sample = i64::from_str_radix(&line[6..], 16).unwrap();
            println!("{:#16x}", sample);
            samples.push(sample);
        }
    }
    proc.kill();
    
    samples
}

fn create_perf_file(samples: Vec<i64>) {
    let header_size = mem::size_of::<perf_data::PerfHeader>();

    let sample_count = samples.len();

    let attrs = vec![perf_data::PerfEventAttr::default(); sample_count];
    let data_mmap = vec![perf_data::RecordMmap::default(); sample_count];
    let data_sample = vec![perf_data::RecordSample::default(); sample_count];

    //for i in 0..sample_count {
    //    let sample = samples[i];
    //}

    let header = perf_data::PerfHeader {
        magic: *b"PERFILE2",
        size: header_size as u64,
        attr_size: mem::size_of::<perf_data::PerfEventAttr>() as u64,
        attrs: perf_data::PerfFileSection {
            offset: header_size as u64,
            size: mem::size_of_val(&*attrs) as u64,
        },
        data: perf_data::PerfFileSection {
            offset: (header_size + mem::size_of_val(&*attrs)) as u64,
            size: (mem::size_of_val(&data_mmap)+mem::size_of_val(&*data_sample)) as u64,
        },
        event_types: perf_data::PerfFileSection {
            offset: 0,
            size: 0,
        },
        flags: 0,
        flags1: [0; 3],
    };

    let header_ptr = &header as *const perf_data::PerfHeader as *const _;
    
    let mut file = File::create("perf.data").expect("Cannot create perf.data");

    unsafe fn write_vector<T>(buf: &mut File, vector: Vec<T>) {
        for i in vector {
            buf.write_all(std::slice::from_raw_parts(&i as *const T as *const _, mem::size_of::<T>())).expect("failed to write perf.data");
        }
    }

    unsafe {
        // write header, attribute section and data section to file
        file.write_all(std::slice::from_raw_parts(header_ptr, header_size)).expect("failed to write perf.data");
        write_vector(&mut file, attrs);
        write_vector(&mut file, data_mmap);
        write_vector(&mut file, data_sample);
    }


}
