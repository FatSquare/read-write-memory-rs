use process_read_write;

fn main(){
    let pid:i32 = 1234; // id of app
    let addr:usize = 0x70eb856006c0; // address of value to read 

    //let pid = get_proc_by_name("SomeRandomGame");
    let pid = process_read_write::get_proc_by_id(pid);

    let health = process_read_write::read_addr(pid,addr,4);
    println!("READING MEMORY: {:?}",health);
}


