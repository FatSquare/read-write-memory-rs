extern crate process_read_write;

fn main() {
    let pid:i32 = 1234; // id of app
    let addr:usize = 0x70eb856006c0; // address of value to change
    let new_value = [0xff,0xff,0xff,0x7f]; // the value the insert into the new address

    //let pid = process_read_write::get_proc_by_name("SomeRandomGame");
    let proc = process_read_write::get_proc_by_id(pid);
    process_read_write::write_addr(proc,addr,&new_value);

}
