use std::process::{Command, Child, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::fs;
use std::mem;

mod perf_data;

fn main() -> Result<(), Error> {
    let mut proc = run_qemu();

    let stdout = proc.stdout.take().ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);
    let mut lines = reader.lines().filter_map(|line| line.ok());

    while lines.next().unwrap().find("Start profiling").is_none() {}

    println!("Profiling started");
    for line in lines {
        if line.find("Stop profiling").is_some() {
            println!("Profiling stopped");
            break;
        } else if line.find("IP: ").is_some() {
            println!("{:#16x}", i64::from_str_radix(&line[6..], 16).unwrap());
        }
    }
    proc.kill();

    let header_size = mem::size_of::<perf_data::PerfHeader>();

    let testdata = perf_data::PerfHeader {
        magic: *b"PERFILE2",
        size: header_size as u64,
        attr_size: 10,
        attrs: perf_data::PerfFileSection {
            offset: 0,
            size: 0,
        },
        data: perf_data::PerfFileSection {
            offset: 0,
            size: 0,
        },
        event_types: perf_data::PerfFileSection {
            offset: 0,
            size: 0,
        },
        flags: 0,
        flags1: [0; 3],
    };

    let data_ptr = &testdata as *const perf_data::PerfHeader as *const _;

    unsafe {
        fs::write("perf.data", std::slice::from_raw_parts(data_ptr, header_size))?;
    }
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

