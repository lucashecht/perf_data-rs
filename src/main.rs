use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

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

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("IP: ").is_some())
        .for_each(|line| println!("{:#16x}", i64::from_str_radix(&line[6..], 16).unwrap()));

    Ok(())
}
