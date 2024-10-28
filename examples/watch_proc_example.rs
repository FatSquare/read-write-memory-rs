use process_read_write;

fn main(){
    let pid = 1234;
    process_read_write::watch_proc(pid);
}
