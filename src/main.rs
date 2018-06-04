extern crate psutil;
#[macro_use]
extern crate structopt;

use psutil::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "autokill", about = "Kill processes that lived to long.")]
struct Opt {
    #[structopt(short = "v", long = "verbose", help = "Activate verbose output")]
    verbose: bool,
    #[structopt(short = "d", long = "dry-run", help = "Don't actually kill processes")]
    dry_run: bool,
    #[structopt(short = "t", long = "time", help = "Minimum CPU time of processes to kill in seconds", default_value = "600")]
    time: f64,
    #[structopt(help = "Names of processes to kill")]
    names: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let processes = processes().unwrap();
    let to_kill= processes
        .into_iter()
        .filter(|ref p| opt.names.contains(&p.comm))
        .filter(|ref p| process_seconds(&p) >= opt.time);
    for p in to_kill {
        print(&opt, format!("Trying to kill pid {} with cpu time {:.2} ... ", p.pid, process_seconds(&p)));
        if !opt.dry_run {
            match p.kill() {
                Ok(_) => println(&opt, format!("ok")),
                Err(e) => println(&opt, format!("error '{}'", e)),
            }
        } else {
            println(&opt, format!("skipped"));
        }
    }
    println(&opt, format!("Done."));
}

fn print(opt: &Opt, s: String) {
    if opt.verbose {
        print!("{}", s);
    }
}

fn println(opt: &Opt, s: String) {
    if opt.verbose {
        println!("{}", s);
    }
}

fn process_seconds(p: &process::Process) -> f64 {
    (p.utime + p.stime + p.cutime + p.cstime)
}

fn processes() -> std::io::Result<Vec<process::Process>> {
    loop {
        match psutil::process::all() {
            Ok(procs) => return Ok(procs),
            Err(why) => {
                if why.kind() != std::io::ErrorKind::NotFound {
                    return Err(why);
                }
            }
        }
    }
}
