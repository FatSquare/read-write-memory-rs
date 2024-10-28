extern crate process_read_write;
#[allow(unused_imports)]
use process_read_write::{get_proc_by_id,get_proc_by_name, read_addr, write_addr,watch_proc};

fn main(){
    let args : Vec<String> = std::env::args().collect();
    println!("current process id: {}",std::process::id());
    assert!(args.len() > 1,"example usage:\n\tsudo cargo run read\n\tsudo cargo run write\n\tsudo cargo run watch");
    match args[1].as_str() {
        "read" => {
            assert!(args.len() > 3,"example usage:\n\tsudo cargo run read <pid:1234> <addr:0xffff>");
            loop {
                read_mem_example(args[2].parse().unwrap(), usize::from_str_radix(&args[3][2..],16).unwrap());
                std::thread::sleep(std::time::Duration::from_secs(3))
            }
        }
        "write" => {
            assert!(args.len() > 3,"example usage:\n\tsudo cargo run write <pid:1234> <addr:0xffff>");
            write_mem_example(args[2].parse().unwrap(), usize::from_str_radix(&args[3][2..],16).unwrap());
        }
        "watch" => {
            assert!(args.len() > 2,"example usage:\n\tsudo cargo run watch <pid:1234>");
            watch_proc_example(args[2].parse().unwrap())
        }
        "test" => {
            watch_proc(21606)
        }
        _ => {}
    }

}


fn watch_proc_example(pid:i32){
    watch_proc(pid);
}

fn read_mem_example(pid:i32,adr:usize) {
    //let pid = get_proc_by_name("SomeRandomGame");
    let pid = get_proc_by_id(pid);


    let health = read_addr(pid,adr,4);
    println!("READING MEMORY: {:?}",health);
}

fn write_mem_example(pid:i32,addr:usize) {
    //let pid = get_proc_by_name("SomeRandomGame");
    let proc = get_proc_by_id(pid);


    let new_value = [0xff,0xff,0xff,0x7f];
    write_addr(proc,addr,&new_value);

}
