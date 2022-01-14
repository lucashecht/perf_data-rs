use std::io::{stdin, Error};
use std::fs::{File};
use std::io::prelude::*;
use std::mem;

mod perf_data;

fn main() -> Result<(), Error> {
    let samples: Vec<u64> = vec![0x1110, 0x1116, 0x111a, 0x111c];

    let samples = collect_samples();
    create_perf_file(samples);

    Ok(())
}

fn collect_samples() -> Vec<u64> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().filter_map(|line| line.ok());
    let mut samples = Vec::new();
    while lines.next().unwrap().find("Start profiling").is_none() {}

    println!("Profiling started");
    for line in lines {
        if line.find("Stop profiling").is_some() {
            println!("Profiling stopped");
            break;
        } else if line.find("IP: ").is_some() {
            let sample = u64::from_str_radix(&line[6..], 16).unwrap();
            println!("{:#16x}", sample);
            samples.push(sample);
        }
    }
    
    samples
}

fn create_perf_file(samples: Vec<u64>) {
    let header_size = mem::size_of::<perf_data::PerfHeader>();

    let attr = perf_data::PerfEventAttr::default();
    let mut data_mmap = vec![perf_data::RecordMmap::default(); 1];
    let filename = "matrix_multiply";
    data_mmap[0].filename.copy_from_slice(format!("{:\0<width$}", filename, width = perf_data::PATH_MAX).as_bytes()); 
    let data_sample: Vec<perf_data::RecordSample> = samples.iter()
                             .enumerate()
                             .map(|(i, ip)| perf_data::RecordSample { ip: *ip, id: i as u64, ..Default::default() })
                             .collect();
    let comm = perf_data::RecordComm { comm: data_mmap[0].filename, ..Default::default() };

    let header = perf_data::PerfHeader {
        magic: *b"PERFILE2",
        size: header_size as u64,
        attr_size: mem::size_of::<perf_data::PerfEventAttr>() as u64,
        attrs: perf_data::PerfFileSection {
            offset: header_size as u64,
            size: mem::size_of::<perf_data::PerfEventAttr>() as u64,
        },
        data: perf_data::PerfFileSection {
            offset: (header_size + mem::size_of::<perf_data::PerfEventAttr>()) as u64,
            size: (mem::size_of_val(&*data_mmap)+mem::size_of_val(&*data_sample)+mem::size_of_val(&comm)) as u64,
        },
        ..Default::default()
    };

    let mut file = File::create("perf.data").expect("Cannot create perf.data");

    unsafe fn write_struct<T>(buf: &mut File, obj: &T) {
        buf.write_all(std::slice::from_raw_parts(obj as *const T as *const _, mem::size_of::<T>()))
            .expect("failed to write object to file");
    }
        
    unsafe fn write_vector<T>(buf: &mut File, vector: Vec<T>) {
        for i in vector {
            write_struct(buf, &i); 
        }
    }

    unsafe {
        // write header, attribute section and data section to file
        write_struct(&mut file, &header);
        write_struct(&mut file, &attr);
        write_vector(&mut file, data_mmap);
        write_struct(&mut file, &comm);
        write_vector(&mut file, data_sample);
    }


}
