use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

mod perf_data;

fn main() -> Result<(), Error> {
    let stdout = Command::new("qemu-system-x86_64")
        .args(["-m", "2048", "-M", "q35", "-cpu", "host,+vmx", "-enable-kvm", "-smp", "cores=4,threads=1,sockets=1"])
        .args(["-serial", "stdio", "-k", "en-us"])
        .args(["-kernel", "/home/lhecht/Downloads/bender"])
        .args(["-initrd", "/home/lhecht/git/supernova-core/build/share/hedron/hypervisor novga serial spinner,/home/lhecht/git/supernova-core/build/test/roottask/thesis-impl_roottasktest"])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

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
    Ok(())
}
