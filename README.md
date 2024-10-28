# read-write-memory-rs

read-wirte-memory-rs is a fast memory read/write, capable of reading and writing memory to any process <br>

it also allow process inspection. you can't attach it to a process and it will give you a real-time list of all the system calls that process made.



## Adding the library

```
cargo add process-read-write
```


please check [the examples](/examples/).

```rs

// all examples can be found at https://github.com/FatSquare/read-write-memory-rs/tree/main/examples
use process_read_write;

fn main(){
    let pid:i32 = 1234; // id of app
    let addr:usize = 0x70eb856006c0; // address of value to read 

    //let pid = get_proc_by_name("SomeRandomGame");
    let pid = process_read_write::get_proc_by_id(pid);

    let health = process_read_write::read_addr(pid,addr,4);
    println!("READING MEMORY: {:?}",health);
}
```


## Install the example binary
```
cargo install process-read-write
```

#### > NOTE

`pid`  is a number example 1234

`addr` is a number in format of hex example 0x1234567



#### Read memory 

```
sudo cargo run read <pid> <addr>
```

#### Write memory

```
sudo cargo run write <pid> <addr>
```

#### Watch process
```
sudo ~/.cargo/bin/process-read-write watch <pid>
```
