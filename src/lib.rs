#![allow(dead_code)]
#![allow(unused_imports)]

use nix::{unistd::Pid,sys::{ptrace,uio::{process_vm_readv,process_vm_writev,RemoteIoVec}},Error};
use std::io::{IoSliceMut,IoSlice};
use sysinfo::System;
use std::collections::HashMap;
use std::process::Command;
use std::os::unix::process::CommandExt;
use nix::{self,sys::wait::waitpid};
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use colored::Colorize;

pub fn get_pid(id: i32) -> Pid{
    Pid::from_raw(id)
}

fn syscalls_list() -> HashMap<u64,String>{
    let mut map = HashMap::new();
    let mut file = File::open("/home/sqrt/scripts/rust/game_hacking/process-read-write/syscall.json").unwrap(); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let parse: serde_json::Value = serde_json::from_str(&contents).unwrap();
    for sysobj in parse["aaData"].as_array().unwrap() {
        map.insert(sysobj[0].as_u64().unwrap(),sysobj[1].as_str().unwrap().to_string());
    }
    map
}
fn printsyscall(syscall_names:&HashMap<u64,String>,regs:nix::libc::user_regs_struct,i:usize) {
    let syscall_name = syscall_names.get(&regs.orig_rax).unwrap();
    // Printing rules
    // if syscall_name.as_str() != "process_vm_readv" {return}
    eprint!("{}",format!("{} => ",i/2).yellow());
    eprintln!("{} ({},{},{},..) = {}",
        syscall_name.green(),
        format!("{:x}",regs.rdi).blue(),
        format!("{:x}",regs.rsi).blue(),
        format!("{:x}",regs.rdx).blue(),
        format!("{:x}",regs.rax).yellow()
     );
} 

pub fn watch_proc(pid:i32){
    let syscall_names = syscalls_list();

    let childpid = nix::unistd::Pid::from_raw(pid);
    ptrace::attach(childpid).unwrap();
    let _ = waitpid(childpid,None).unwrap();

    let mut i = 0;
    loop{
        ptrace::syscall(childpid,None).unwrap();
        let _ = waitpid(childpid,None).unwrap();
        let regs = ptrace::getregs(childpid).expect(&format!("{}","Error: process most likely terminated!".red()));
        if i%2 == 0{
            printsyscall(&syscall_names,regs,i);
        }
        i+=1;
    }
}

// maybe i should use pidof instead since its more accurate
pub fn get_pid_by_name(process_name: &str) -> Pid{
    let s = System::new_all();
    let pid:usize;
    let procs:Vec<_> = s.processes_by_exact_name(process_name).collect();
    match procs.len() {
        1 => pid = procs[0].pid().into(),
        0 => panic!("No process found!"),
        _ => {
            println!("Multipe processes found!, please try get_pid(PID) instead");
            for proc in procs{
                println!("{} => {}",proc.name(),proc.pid());
            }
            println!("Trying to use pidof"); 
            std::process::exit(1);
        },
    }
    Pid::from_raw(pid.try_into().unwrap())
}


// process_vm_readv and process_vm_writev from libc which is implement by the linux kernel
pub fn read_addr(pid:Pid,addr:usize,length:usize) -> Result<Vec<u8>,Error>
{
    let mut data: Vec<u8> = vec![0;length]; 
    let local_iov = IoSliceMut::new(&mut data);

    let remote_iov = RemoteIoVec {
        base : addr,
        len : length,
    };

    process_vm_readv(pid,&mut [local_iov],&[remote_iov])?;
    Ok(data[..length].to_vec())
}

pub fn write_addr(pid:Pid,addr:usize,data:&[u8]){
    let mut _data:&[u8] = data; 
    let local_iov = IoSlice::new(&mut _data);

    let remote_iov = RemoteIoVec {
        base : addr,
        len : data.len(),
    };

    process_vm_writev(pid,&mut [local_iov],&[remote_iov]).unwrap();
}
