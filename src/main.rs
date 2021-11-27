use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    let stdout = Command::new("qemu-system-x86_64")
        .args(["-m", "2048", "-M", "q35", "-cpu", "host,+vmx", "-enable-kvm", "-smp", "cores=4,threads=1,sockets=1"])
        .args(["-serial", "stdio", "-k", "en-us"])
        .args(["-kernel", "/home/lhecht/Downloads/bender"])
        .args(["-initrd", "/home/lhecht/git/supernova-core/build/share/hedron/hypervisor novga serial spinner,/home/lhecht/git/supernova-core/build/test/roottask/thesis-impl_roottasktest"])
        .spawn()
        .expect("command failed");
}
