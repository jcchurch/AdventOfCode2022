use std::io;
use std::io::prelude::*;

struct MyFile {
    name: String,
    size: u32,
    nodes: Vec<MyFile>,
    parent: Option<*mut MyFile>,
    is_dir: bool
}

fn read_all_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push( line.unwrap() );
    }

    lines
}

unsafe fn parse_cd(filesystem_pointer: &mut *mut MyFile, lines: &Vec<String>, i: &mut usize) {
    let parts: Vec<&str> = lines[*i].split_whitespace().collect();
    *i += 1;

    if parts[2] == "/" {
        while (*(*filesystem_pointer)).parent != None {
            *filesystem_pointer = (*(*filesystem_pointer)).parent.unwrap();
        }
    }
    else if parts[2] == ".." {
        *filesystem_pointer = (*(*filesystem_pointer)).parent.unwrap();
    }
    else {
        for x in 0..(*(*filesystem_pointer)).nodes.len() {
            if (*(*filesystem_pointer)).nodes[x].name == parts[2] {
                *filesystem_pointer = &mut (*(*filesystem_pointer)).nodes[x]; 
                break;
            }
        }
    }
}

unsafe fn parse_ls(filesystem_pointer: &*mut MyFile, lines: &Vec<String>, i: &mut usize) {
    *i += 1;
    let mut parts: Vec<&str> = lines[*i].split_whitespace().collect();
    while *i < lines.len() && parts[0] != "$" {
        if parts[0] == "dir" {
            (*(*filesystem_pointer)).nodes.push(MyFile {
                name: parts[1].to_string(),
                size: 0,
                nodes: Vec::new(),
                parent: Some(*filesystem_pointer),
                is_dir: true
            });
        }
        else {
            (*(*filesystem_pointer)).nodes.push(MyFile {
                name: parts[1].to_string(),
                size: parts[0].parse().unwrap(),
                nodes: Vec::new(),
                parent: Some(*filesystem_pointer),
                is_dir: false
            });
        }
        *i += 1;

        if *i < lines.len() {
            parts = lines[*i].split_whitespace().collect();
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> MyFile {
    let mut root = MyFile {
        name: "/".to_string(),
        size: 0,
        nodes: Vec::new(),
        parent: None,
        is_dir: true
    };

    let mut filesystem_pointer: *mut MyFile = &mut root;
    let mut i: usize = 0;

    while i < lines.len() {
        let parts: Vec<&str> = lines[i].split_whitespace().collect();
        if parts[0] == "$" && parts[1] == "cd" {
            unsafe {
                parse_cd(&mut filesystem_pointer, &lines, &mut i);
            }
        }
        else if parts[0] == "$" && parts[1] == "ls" {
            unsafe {
                parse_ls(&mut filesystem_pointer, &lines, &mut i);
            }
        }
    }

    root
}

fn print_levels(file: &MyFile, level: u32) {
    for _ in 0..level {
        print!("    ");
    }
    println!("/{} {}", file.name, file.size);

    if file.is_dir {
        for x in 0..file.nodes.len() {
            print_levels(&file.nodes[x], level+1);
        }
    }
}

fn add_up_directories(file: &mut MyFile) -> u32 {
    let mut total: u32 = 0;
    for x in 0..file.nodes.len() {
        if file.nodes[x].is_dir {
            total += add_up_directories(&mut file.nodes[x] );
        }
        else {
            total += file.nodes[x].size;
        }
    }
    file.size = total;
    total
}

fn get_all_directories(file: &MyFile, directories_by_size: &mut Vec<u32>) {
    directories_by_size.push(file.size);

    for x in 0..file.nodes.len() {
        if file.nodes[x].is_dir {
            get_all_directories(&file.nodes[x], directories_by_size);
        }
    }
}

fn main() {
    let mut directories_by_size: Vec<u32> = Vec::new();
    let lines = read_all_lines();
    let mut root: MyFile = parse_lines(&lines);
    let _ = add_up_directories(&mut root);
    get_all_directories(&root, &mut directories_by_size);
    print_levels(&root, 0);

    let disk_size: u32 = 70000000;
    let disk_used: u32 = root.size;
    let disk_free: u32 = disk_size - disk_used;
    let disk_needed: u32 = 30000000 - disk_free;

    directories_by_size.sort();
    let mut i: usize = 0;
    while directories_by_size[i] < disk_needed {
        i += 1;
    }
    println!("{}", directories_by_size[i]);
}
