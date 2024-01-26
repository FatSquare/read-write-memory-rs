#![allow(dead_code)]

extern crate process_read_write;
use process_read_write::{get_pid, read_addr, write_addr,watch_proc};
use colored::Colorize; 
fn main(){
    println!("{}",format!("process_id: {}",std::process::id()).green());
    let pid = get_pid(308268);
    loop {
        let kills = read_addr(pid,0xfaca69c,2);
        println!("kills: {:?}",kills);
    }
}


fn watch_proc_example(pid:i32){
    watch_proc(pid);
}

fn read_mem_example(pid:i32,adr:usize) {
    //let pid = get_pid_by_name("SomeRandomGame");
    let pid = get_pid(pid);


    let health = read_addr(pid,adr,4).unwrap();
    println!("READING MEMORY: {:?}",health);
}

fn write_mem_example(pid:i32) {
    //let pid = get_pid_by_name("SomeRandomGame");
    let pid = get_pid(pid);

    const HEALTH_ADDR:usize = 0x48a88cc;

    let new_health = [0xff,0xff];
    write_addr(pid,HEALTH_ADDR,&new_health);
}
