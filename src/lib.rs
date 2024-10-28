#![allow(dead_code)]

use nix::{unistd::Pid,sys::{ptrace,uio::{process_vm_readv,process_vm_writev,RemoteIoVec}},Error};
use std::io::{IoSliceMut,IoSlice};
use sysinfo::System;
use std::collections::HashMap;
use nix::{self,sys::wait::waitpid};
use serde_json;

const SYSCALL_DATA: &str = include_str!("../data/syscall.json");

pub fn get_proc_by_id(id: i32) -> Pid{
    Pid::from_raw(id)
}

fn syscalls_list() -> HashMap<u64,String>{
    let mut map = HashMap::new();
    let parse: serde_json::Value = serde_json::from_str(&SYSCALL_DATA).unwrap();
    for sysobj in parse["aaData"].as_array().unwrap() {
        map.insert(sysobj[0].as_u64().unwrap(),sysobj[1].as_str().unwrap().to_string());
    }
    map
}
fn printsyscall(syscall_names:&HashMap<u64,String>,regs:nix::libc::user_regs_struct,i:usize) {
    let syscall_name = syscall_names.get(&regs.orig_rax).unwrap();
    // use something like this if you wanna trigger an event only on specefic syscall `if syscall_name.as_str() != "process_vm_readv" {return}`
    eprint!("{} => ",i/2);
    eprintln!("{} ({},{:x},{:x},..) = {}",
        syscall_name,
        regs.rdi,
        regs.rsi,
        regs.rdx,
        regs.rax
     );
} 
/// `watch_proc` is used to monitor a process and get a real-time list of all the system calls it makes.
/// 
/// # Backend
///
/// this function invokes the `ptrace` syscall on specefic process
/// 
/// # Examples
///
/// ```
/// use process_read_write;
///
/// fn main(){
///     let pid = 1234;
///     process_read_write::watch_proc(pid);
/// }
/// ```

pub fn watch_proc(pid:i32){
    let syscall_names = syscalls_list();
    let childpid = Pid::from_raw(pid);
    ptrace::attach(childpid).expect("cant attach to pid");
    let _ = waitpid(childpid,None).expect("timeout waiting for pid");
    let mut i = 0;
    loop{
        ptrace::syscall(childpid,None).unwrap();
        let _ = waitpid(childpid,None).unwrap();
        let regs = ptrace::getregs(childpid).expect(&format!("{}","Error: process most likely terminated!"));
        if i%2 == 0{
            printsyscall(&syscall_names,regs,i);
        }
        i+=1;
    }
}

/// `get_proc_by_name` will take a name try to find a process with that name 
/// 
/// # Note 
///
/// this function will panic with an error message if it 0 or multipe process by that name were found. 
/// planning to make so it returns Result<T,E>
/// 
/// # Examples
///
/// ```
/// use process_read_write;
/// use nix::unistd::Pid;
///
/// fn main(){
///     let name = "MyGame-x86";
///     let pid:Pid = process_read_write::get_proc_by_name(name);
/// }
/// ```
pub fn get_proc_by_name(process_name: &str) -> Pid{
    let s = System::new_all();
    let pid:usize;
    let procs:Vec<_> = s.processes_by_exact_name(process_name).collect();
    match procs.len() {
        1 => pid = procs[0].pid().into(),
        0 => panic!("No process found!"),
        _ => {
            println!("Multipe processes found!, please try get_proc_by_id(pid) instead");
            for proc in procs{
                println!("{} => {}",proc.name(),proc.pid());
            }
            println!("Trying to use pidof"); 
            std::process::exit(1);
        },
    }
    Pid::from_raw(pid.try_into().unwrap())
}


/// `read_addr` is used to read `n` bytes from a process `pid` and starting from `addr`
/// 
/// # Note
///
/// the function will return Result<T,E>
///
/// Error examples:
/// - `EPERM`: make sure running as sudo
/// - `ESRCH`: make sure the process exist
/// - `ESFAULT`: make sure the address exist in the scope of the process 
///
/// # Backend
///
/// this function invokes the `process_vm_readv` syscall, enabling direct memory reading from a specified address in the target process.
/// 
/// # Examples
///
/// ```
/// use process_read_write;
///
/// fn main(){
///     let pid:i32 = 1234; // id of app
///     let addr:usize = 0x70eb856006c0; // address of value to read 
///
///     //let pid = get_proc_by_name("SomeRandomGame");
///     let pid = process_read_write::get_proc_by_id(pid);
///
///     let health = process_read_write::read_addr(pid,addr,4);
///     println!("READING MEMORY: {:?}",health);
/// }
/// ```
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

/// `write_addr` is used to write a buffer of `n` bytes into the memory of process `pid` at `addr` 
/// 
/// # Backend
///
/// this function invokes the `process_vm_writev` syscall, enabling direct memory writing into a specified address in the target process. 
/// 
/// # Examples
///
/// ```
/// use process_read_write;
///
/// fn main(){
///     let pid:i32 = 1234; // id of app
///     let addr:usize = 0x70eb856006c0; // address of value to change
///     let new_value = [0xff,0xff,0xff,0x7f]; // the value the insert into the new address
///
///     //let pid = process_read_write::get_proc_by_name("SomeRandomGame");
///     let pid = process_read_write::get_proc_by_id(pid);
///
///     process_read_write::write_addr(pid,addr,&new_value);
/// }
/// ```
pub fn write_addr(pid:Pid,addr:usize,data:&[u8]){
    let mut _data:&[u8] = data; 
    let local_iov = IoSlice::new(&mut _data);

    let remote_iov = RemoteIoVec {
        base : addr,
        len : data.len(),
    };

    process_vm_writev(pid,&mut [local_iov],&[remote_iov]).unwrap();
}
