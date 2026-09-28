pub struct Translator {
    num: u128,
    filename: String,
    fn_name: String,
}

impl Translator {
    pub fn new(filename: &str) -> Self {
        Translator {
            num: 0,
            filename: filename.to_string(),
            fn_name: String::from("global"),
        }
    }

    fn get_num(&mut self) -> u128 {
        let n = self.num;
        self.num = n + 1;
        n
    }

    fn set_fn(&mut self, name: &str) {
        self.fn_name = name.replace('.', "_");
    }

    fn jmp(&mut self, cmd: &str) -> String {
        let id = self.get_num();
        let x86_jump = match cmd {
            "eq" => "je",
            "gt" => "jg",
            "lt" => "jl",
            _ => panic!("Unknown jump command"),
        };

        format!(
            "pop bx\n\
            \x20   pop ax\n\
            \x20   cmp ax, bx\n\
            \x20   {x86_jump} TRUE_{id}\n\
            \x20   push 0\n\
            \x20   jmp END_{id}\n\
            TRUE_{id}:\n\
            \x20   push -1\n\
            END_{id}:\n"
        )
    }

    fn ret(&mut self) -> String {
        "pop ax\nmov sp, bp\npop bp\nret\n".to_string()
    }

    fn look_up(&mut self, cmd: &str) -> String {

        match cmd {
            "add" | "sub" | "and" | "or" => format!("pop bx\npop ax\n{} ax, bx\npush ax\n", cmd),
            "mul" => "pop bx\npop ax\nimul bx\npush ax\n".to_string(),
            "div" => "pop bx\npop ax\nxor dx,dx\nidiv bx\npush ax\n".to_string(),
            "rem" => "pop bx\npop ax\nxor dx,dx\nidiv bx\npush dx\n".to_string(),
            "neg" | "not" => format!("pop ax\n{} ax\npush ax", cmd),
            "return" => self.ret(),
            _ => self.jmp(cmd)
        }
    }

    pub fn stack_op(&mut self, cmd: &str, seg: &str, iter: &str) -> String {
        let index: u16 = iter.parse().unwrap();

        match cmd {
            "push" => match seg {
                "constant" => format!("mov ax, {}\npush ax\n", index),

                "local" => format!("mov ax, [bp - {}]\npush ax\n", 2*index + 2),
                "argument" => format!("mov ax, [bp + {}]\npush ax\n", 2*index + 4),
                
                "this" => format!("mov ax, [si + {}]\npush ax\n", index*2),
                "that" => format!("mov ax, [di + {}]\npush ax\n", index*2),
                
                "temp" => format!("mov ax, [temp_segment + {}]\npush ax\n", index*2),
                "pointer" => {
                    let register =
                    if index == 0 {
                        "si"
                    } else {
                        "di"
                    };
                    format!("push {}\n", register)
                },

                "static" => format!(
                    "mov ax, [static_{}_{}]\npush ax\n",
                    self.filename, index
                ),

                _ => panic!("Unknown push segment {}", seg),
            },

            "pop" => match seg {

                "local" => format!("pop ax\nmov [bp - {}], ax\n", 2*index + 2),
                "argument" => format!("pop ax\nmov [bp + {}], ax\n", 2*index + 4),
                
                "this" => format!("pop ax\nmov [si + {}], ax\n", index*2),
                "that" => format!("pop ax\nmov [di + {}], ax\n", index*2),
                
                "temp" => format!("pop ax\nmov [temp_segment + {}], ax\n", index*2),
                "pointer" => {
                    let register =
                    if index == 0 {
                        "si"
                    } else {
                        "di"
                    };
                    format!("pop ax\nmov {}, ax\n", register)
                },

                "static" => format!(
                    "pop ax\nmov [static_{}_{}], ax\n",
                    self.filename, index
                ),

                _ => panic!("Unknown pop segment {}", seg),
            },

            "function" => self.function(seg, index),
            "call" => format!("call {}\nadd sp, {}\npush ax\n", seg.replace('.', "_"), index*2),

            _ => panic!("Unknown stack command {}", cmd),
        }
    }

    fn function(&mut self, name: &str, n_vars: u16) -> String {
        self.set_fn(name);
        let mut result = String::new();
        result.push_str(&format!("{}:\npush bp\nmov bp, sp\n", self.fn_name));
        result.push_str(&format!("sub sp, {}\n", 2* n_vars));
        result
    }

    fn branching(&mut self, cmd: &str, name: &str) -> String {
        let label = format!("l_{}_{}", self.fn_name, name);

        match cmd {
            "label" => format!("{}:\n", label),
            "goto" => format!("jmp {}\n", label), 
            "if-goto" => format!("pop ax\ntest ax,ax\njne {}\n", label),
            _ => panic!("Unknown stack command {}", cmd),
        }
    }

    pub fn translate(&mut self, line: &str) -> String {
        let (command, arg) = line.split_once(' ').unwrap_or((line, ""));

        if arg.is_empty() {
            self.look_up(command)
        } else {
            let (seg, iter) = arg.split_once(' ').unwrap_or((arg, ""));

            if iter.is_empty() {
                self.branching(command, seg)
            } else {
                self.stack_op(command, seg, iter)
            }
        }
    }
}