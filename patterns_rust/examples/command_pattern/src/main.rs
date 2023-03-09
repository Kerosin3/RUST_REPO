use std::sync::Arc;
// example Rust using Command pattern
fn main() {
    // create
    let mut shema_generic = Schema::new();
    let c = Box::new(CommandStore::new(1));
    let c1 = Box::new(CommandStore::new(2));
    let c2 = Box::new(CommandStore::new(3));
    shema_generic.add_command(c);
    shema_generic.add_command(c1);
    shema_generic.add_command(c2);
    let db0 = Arc::new(DbObj);
    let msg = "stage 0".to_string();
    shema_generic.init_and_execute(db0, msg);
    shema_generic.finish();
}

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

    #[allow(unused)]
    pub fn init_and_execute(&mut self, db: Arc<DbObj>, gmessage: String) {
        for c in self.commands.iter_mut() {
            c.initialize().expect("error initializing");
            let msg_c = gmessage.clone();
            let some_f = Box::new(move |db: Arc<DbObj>, msg: String| {
                format!("commiting with msg: {}", msg_c)
            });
            let result = c.preform_action(some_f, Arc::clone(&db), gmessage.clone());
            println!("::->{}", result.expect("error performing action"));
        }
    }
    pub fn finish(&mut self) {
        for c in self.commands.iter_mut() {
            c.finallize();
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
