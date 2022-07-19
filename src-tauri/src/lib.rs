pub mod request;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write, Error},
};

///
/// # read_hosts_file
///
/// 获取 hosts 文件内容
///  
pub fn read_hosts_file(os_type: &str) -> String {
    let mut file = File::open(path(&os_type)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

pub fn write_hosts_file(os_type: &str, hosts: &str) {
    let mut file = File::options().write(true).open(path(&os_type)).unwrap();
    file.write_all(hosts.as_bytes());
    file.sync_all();
    
}

fn path(os_type: &str) -> String {
    // osType 'Linux' on Linux, 'Darwin' on macOS, and 'Windows_NT' on Windows.
    let hosts_file_path: HashMap<&str, &str> = HashMap::from([
        ("Linux", "/etc/hosts"),
        ("Darwin", "/private/etc/hosts"),
        ("Windows_NT", "C:\\Windows\\System32\\drivers\\etc\\hosts"),
    ]);
    return hosts_file_path.get(&os_type).unwrap().to_string();
}
