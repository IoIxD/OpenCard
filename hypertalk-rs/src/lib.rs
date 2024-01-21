use std::collections::HashMap;

#[derive(Debug)]
pub struct Script {
    pub commands: Vec<Command>,
}

impl Script {
    pub fn parse(script: String) -> Self {
        let script = script.replace("\u{000D}", "\n").replace("¬\n", " ");
        println!("{}", script);
        let parts = script.split("\n");
        let mut main_commands = vec![];
        let mut map: HashMap<String, Vec<Command>> = HashMap::new();
        let mut selected_cmd: Option<String> = None;

        for part in parts {
            let mut commands = match selected_cmd {
                Some(ref a) => map.get_mut(a).unwrap(),
                None => &mut main_commands,
            };
            let mut words = part.split(" ");
            let cmd = words.nth(0).unwrap();
            // None of the standard Hypercard commands have capitals, so we can easily distinquish these as external functions.
            if cmd.to_lowercase() != cmd {
                commands.push(Command::ExternalFunction(ExternalFunction {
                    func_name: cmd.to_string(),
                    args: words.map(|f| f.to_string()).collect::<Vec<String>>()[1..].to_vec(),
                }))
            } else {
                // Start with the ones that always start with something.
                match cmd {
                    "on" => {
                        let name = words.clone().nth(0).unwrap().to_string();
                        selected_cmd = Some(name.clone());
                        map.insert(name, Vec::new());
                    }
                    "end" => {
                        let name = words.clone().nth(0).unwrap().to_string();
                        let cmds = commands.clone();
                        selected_cmd = None;
                        commands = match selected_cmd {
                            Some(ref a) => map.get_mut(a).unwrap(),
                            None => &mut main_commands,
                        };
                        commands.push(Command::Block(Block {
                            message: name,
                            commands: cmds.to_vec(),
                        }))
                    }
                    "global" => {}
                    _ => commands.push(Command::Unknown(
                        cmd.to_owned()
                            + &words
                                .map(|f| f.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                    )),
                }
            }
        }

        Self {
            commands: main_commands,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Block(Block),
    Go(Go),
    Put(Put),
    ExternalFunction(ExternalFunction),
    Global(Global),
    Unknown(String),
}

#[derive(Debug, Clone)]

pub struct Block {
    pub message: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]

pub struct Go {
    pub card_name: String,
}

#[derive(Debug, Clone)]

pub struct Put {
    pub thing: String,
    pub place: String,
}

#[derive(Debug, Clone)]

pub struct ExternalFunction {
    pub func_name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]

pub struct Global {
    pub variables: Vec<String>,
}
