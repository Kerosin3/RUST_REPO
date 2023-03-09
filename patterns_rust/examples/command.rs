use std::{default, sync::Arc};
fn main() {
    // create
    let shema_generic = Schema::new();
    /*
    let mut c = CommandStore::new(1);
    let mut c1 = CommandStore::new(2);
    c.initialize().expect("error unwrapping");
    c1.initialize().expect("error unwrapping");
    let db0 = Arc::new(DbObj);
    let msg_back = c
        .preform_action(
            Box::new(execute_with_a_message),
            db0,
            "some message 0, ".to_string(),
        )
        .unwrap();
    println!("::{}", msg_back);
    c.finallize();
    c1.finallize();*/
}
#[allow(unused)]
fn execute_with_a_message(db: Arc<DbObj>, msg: String) -> String {
    format!("committing {msg}")
}

// impl Clone for Box<dyn FnOnce(Arc<DbObj>, String) -> String> {}

struct Schema {
    commands: Vec<Box<dyn Command>>,
}

impl Schema {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    pub fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }
    pub fn init_and_execute(&mut self, db: Arc<DbObj>, gmessage: String) {
        for c in self.commands.iter_mut() {
            c.initialize().expect("error initializing");
            let msg_c = gmessage.clone();
            let some_f =
                Box::new(move |db: Arc<DbObj>, msg: String| format!("commiting {}", msg_c));
            c.preform_action(some_f, Arc::clone(&db), gmessage.clone());
        }
    }
}

pub struct DbObj;

pub trait Command {
    fn initialize(&mut self) -> Result<(), ErrorR>;
    fn preform_action(
        &mut self,
        f: Box<dyn FnOnce(Arc<DbObj>, String) -> String>,
        db: Arc<DbObj>,
        msg: String,
    ) -> Result<String, ErrorR>;
    fn finallize(&mut self);
}
#[derive(Debug)]
pub enum ErrorR {
    OK,
    Error,
}
#[non_exhaustive]
pub enum CommandStore {
    CommandA(String),
    CommandB(String),
    CommandC(String),
}

impl CommandStore {
    fn new(arg: usize) -> Self {
        match arg {
            1 => CommandStore::CommandA("commandA".to_string()),
            2 => CommandStore::CommandB("commandB".to_string()),
            3 => CommandStore::CommandC("commandC".to_string()),
            _ => CommandStore::CommandA("commandA".to_string()),
        }
    }
}

impl Command for CommandStore {
    fn initialize(&mut self) -> Result<(), ErrorR> {
        match self {
            CommandStore::CommandA(c) => println!("initilized {:?}", c),
            CommandStore::CommandB(c) => println!("initilized {:?}", c),
            CommandStore::CommandC(c) => println!("initilized {:?}", c),
            _ => {
                println!("wrong command");
                return Err(ErrorR::Error);
            }
        }
        Ok(())
    }

    fn preform_action(
        &mut self,
        f: Box<dyn FnOnce(Arc<DbObj>, String) -> String>,
        db: Arc<DbObj>,
        msg: String,
    ) -> Result<String, ErrorR> {
        match self {
            CommandStore::CommandA(c) => Ok(f(db, msg + c.as_str())),
            CommandStore::CommandB(c) => Ok(f(db, msg + c.as_str())),
            CommandStore::CommandC(c) => Ok(f(db, msg + c.as_str())),
            _ => Err(ErrorR::Error),
        }
    }

    fn finallize(&mut self) {
        match self {
            CommandStore::CommandA(c) => println!("finitilized {:?}", c),
            CommandStore::CommandB(c) => println!("finitilized {:?}", c),
            CommandStore::CommandC(c) => println!("finitilized {:?}", c),
            _ => {
                println!("wrong command");
            }
        }
    }
}
