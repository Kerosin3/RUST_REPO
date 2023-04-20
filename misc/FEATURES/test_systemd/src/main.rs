use daemonizr::{Daemonizr, DaemonizrError, Group, Stderr, Stdout, User};
use log::warn;
use signal_hook::flag;
use simple_logger::SimpleLogger;
use std::io::Error;
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::{path::PathBuf, process::exit, thread::sleep, time::Duration};
use systemd::{journal, sd_journal_log};
fn main() {
    if !systemd::daemon::booted().is_ok() {
        return;
    }

    let root_path = project_root::get_project_root().unwrap();

    let dem_dir = Path::new(&root_path).join("deamon");
    match Daemonizr::new()
        .work_dir(PathBuf::from(&dem_dir))
        .expect("invalid path")
        .as_user(User::by_name("ker0").expect("invalid user"))
        .as_group(Group::by_name("users").expect("invalid group"))
        .pidfile(PathBuf::from("dmnzr.pid"))
        .stdout(Stdout::Redirect(PathBuf::from("dmnzr.out")))
        .stderr(Stderr::Redirect(PathBuf::from("dmnzr.err")))
        .umask(0o027)
        .expect("invalid umask")
        .spawn()
    {
        Err(DaemonizrError::AlreadyRunning) => {
            /* search for the daemon's PID  */
            match Daemonizr::new()
                .work_dir(PathBuf::from(dem_dir))
                .unwrap()
                .pidfile(PathBuf::from("dmnzr.pid"))
                .search()
            {
                Err(x) => eprintln!("error: {}", x),
                Ok(pid) => {
                    eprintln!("another daemon with pid {} is already running", pid);
                    exit(1);
                }
            };
        }
        Err(e) => eprintln!("DaemonizrError: {}", e),
        Ok(()) => { /* We are in daemon process now */ }
    };
    let pid = process::id();
    sd_journal_log!(4, "Launched daemon with pid {}", pid);
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
    while !term.load(Ordering::Relaxed) {
        println!("Doing work...");
        thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("Received SIGTERM kill signal. Exiting...");
    //     println!("write something to stdout");
    //     eprintln!("write something to stderr");
    println!("Daemon exits");
    sd_journal_log!(4, "daemon with pid {} termnated", pid);
}
//use systemd::daemon::{notify, STATE_WATCHDOG};
//notify(false, [(STATE_WATCHDOG, "1")].iter()).unrwap();
