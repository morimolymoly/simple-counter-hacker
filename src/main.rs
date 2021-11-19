mod engine;
use engine::*;

use std::io;

use memflow::*;
use memflow_win32::*;

use clap::*;
use log::{info, Level};

use crate::engine::stacksearch::StackSearch;

fn main() {
    println!("Simple counter hacker!");

    let matches = App::new("dump offsets example")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("verbose").short("v").multiple(true))
        .arg(
            Arg::with_name("connector")
                .long("connector")
                .short("c")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("args")
                .long("args")
                .short("a")
                .takes_value(true)
                .default_value(""),
        )
        .arg(
            Arg::with_name("collectors")
                .long("collectors")
                .short("col")
                .takes_value(true)
                .use_delimiter(true)
                .default_value("interfaces,recvprops,convars"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .default_value("./dump"),
        )
        .get_matches();

    // set log level
    let level = match matches.occurrences_of("verbose") {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        4 => Level::Trace,
        _ => Level::Trace,
    };
    simple_logger::SimpleLogger::new()
        .with_level(level.to_level_filter())
        .init()
        .unwrap();

    // create inventory + connector
    let inventory = unsafe { ConnectorInventory::scan() };
    let connector = unsafe {
        inventory.create_connector(
            matches.value_of("connector").unwrap(),
            &ConnectorArgs::parse(matches.value_of("args").unwrap()).unwrap(),
        )
    }
    .unwrap();

    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()
        .unwrap();
    
    let process_info = kernel
        .process_info("simplecounter.exe")
        .expect("unsable to find simplecounter.exe process");

    let mut process = Win32Process::with_kernel(kernel, process_info);
    info!("found process: {:?}", process);

    let module_info = process.module_info("simplecounter.exe").unwrap();
    info!("found module: {:?}", module_info);

    let process_info = process.clone().proc_info;
    let stackbase = process.virt_mem
        .virt_read_addr64(process_info.teb.unwrap_or_default() + 8).unwrap();
    info!("stackbase 0x{:x}", stackbase);
    let stacklimit = process.virt_mem
        .virt_read_addr64(process_info.teb.unwrap_or_default() + 16).unwrap();
    info!("stacklimit 0x{:x}", &stacklimit);

    let mut sc = StackSearch::new(stackbase, stacklimit, process.clone());

    loop {
        println!("memoy search!");
        println!("input target!");

        let mut target = String::new();
        io::stdin().read_line(&mut target).expect("Failed to get target value!");

        println!("{}", target);
        let target = target.trim().parse::<u32>().unwrap();

        let size = sc.search_u32(target);

        if size == 1 {
            println!("memoy write cheat!");
            println!("input value!");
    
            let mut value = String::new();
            io::stdin().read_line(&mut value).expect("Failed to get cheat value!");
            let value = value.trim().parse::<i32>().unwrap();
            sc.cheat(value);
        }
    }
}
