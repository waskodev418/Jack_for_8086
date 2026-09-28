use std::collections::HashMap;

// ==========================================
// 1. FIXED SYMBOL TABLE & MANAGER
// ==========================================

#[derive(Clone, Debug)]
struct SymbolicEntry {
    name: String,
    typed: String,
    kind: String,
    index: u8,
}

impl SymbolicEntry {
    fn new(name: &str, typed: &str, kind: &str) -> Self {
        SymbolicEntry { 
            name: name.to_string(), 
            typed: typed.to_string(), 
            kind: kind.to_string(), 
            index: 0,
        }
    }
}

struct SymbolicTable {
    body: HashMap<String, SymbolicEntry>,
    counter: HashMap<String, u8>,
}

impl SymbolicTable {
    fn new() -> Self {
        SymbolicTable { body: HashMap::new(), counter: HashMap::new() }
    }

    fn find(&self, name: &str) -> Option<&SymbolicEntry> {
        self.body.get(name)
    }

    fn add(&mut self, mut entry: SymbolicEntry) {
        // Fix: Use entry to fetch current count, assign it as index, then increment counter
        let current_index = *self.counter.get(&entry.kind).unwrap_or(&0);
        entry.index = current_index;
        self.counter.insert(entry.kind.clone(), current_index + 1);
        self.body.insert(entry.name.clone(), entry);
    }
}

struct SymbolManager {
    class_table: SymbolicTable,
    subroutine_table: SymbolicTable,
}

impl SymbolManager {
    fn new() -> Self {
        SymbolManager { 
            class_table: SymbolicTable::new(), 
            subroutine_table: SymbolicTable::new() 
        }
    }

    fn find(&self, name: &str) -> Option<(bool, SymbolicEntry)> {
        if let Some(entry) = self.subroutine_table.find(name) {
            return Some((false, entry.clone())); // false = local scope
        }
        if let Some(entry) = self.class_table.find(name) {
            return Some((true, entry.clone())); // true = class scope
        }
        None
    }

    fn add_subroutine(&mut self, entry: SymbolicEntry) {
        self.subroutine_table.add(entry);
    }

    fn add_class(&mut self, entry: SymbolicEntry) {
        self.class_table.add(entry);
    }

    fn dump_subroutine(&mut self) {
        self.subroutine_table = SymbolicTable::new();
    }

    fn count_kind_class(&self, kind: &str) -> u8 {
        *self.class_table.counter.get(kind).unwrap_or(&0)
    }

    fn count_kind_subroutine(&self, kind: &str) -> u8 {
        *self.subroutine_table.counter.get(kind).unwrap_or(&0)
    }
}

// ==========================================
// 2. STACK DATA STRUCTURE
// ==========================================

struct Stack<T> {
    body: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { body: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.body.push(value);
    }
}

// ==========================================
// 3. COMPILATION ENGINE
// ==========================================

pub struct CompilationEngine {
    result: Stack<String>,
    tokens: Vec<String>,
    current: usize,
    table: SymbolManager,
    class_name: String,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<String>) -> Self {
        CompilationEngine {
            result: Stack::new(),
            tokens,
            current: 0,
            table: SymbolManager::new(),
            class_name: String::new(),
        }
    }

    pub fn get_result(&self) -> Vec<String> {
        self.result.body.clone()
    }

    // --- Helpers ---

    fn token_value(&self, token: &str) -> String {
        let parts: Vec<&str> = token.split_whitespace().collect();
        if parts.len() >= 2 { parts[1].to_string() } else { String::new() }
    }

    fn token_type(&self, token: &str) -> String {
        if let Some(start) = token.find('<') {
            if let Some(end) = token.find('>') {
                return token[start + 1..end].to_string();
            }
        }
        String::new()
    }

    fn peek_val(&self) -> String {
        if self.current < self.tokens.len() {
            self.token_value(&self.tokens[self.current])
        } else {
            String::new()
        }
    }

    fn peek_type(&self) -> String {
        if self.current < self.tokens.len() {
            self.token_type(&self.tokens[self.current])
        } else {
            String::new()
        }
    }

    fn peek_ahead_val(&self) -> String {
        if self.current + 1 < self.tokens.len() {
            self.token_value(&self.tokens[self.current + 1])
        } else {
            String::new()
        }
    }

    fn is_identifier(&self) -> bool { self.peek_type() == "identifier" }
    fn is_integer_constant(&self) -> bool { self.peek_type() == "integerConstant" }
    fn is_string_constant(&self) -> bool { self.peek_type() == "stringConstant" }
    fn is_keyword_constant(&self) -> bool { matches!(self.peek_val().as_str(), "true" | "false" | "null" | "this") }
    fn is_op(&self) -> bool { matches!(self.peek_val().as_str(), "+" | "-" | "*" | "/" | "&" | "|" | "<" | ">" | "=") }
    fn is_unary_op(&self) -> bool { matches!(self.peek_val().as_str(), "-" | "~") }

    fn eat(&mut self, expected: &str) {
        let val = self.peek_val();
        if val != expected {
            panic!("Syntax Error: Expected '{}', got '{}' at token {}", expected, val, self.current);
        }
        self.current += 1;
    }

    fn eat_identifier(&mut self) -> String {
        if !self.is_identifier() {
            panic!("Syntax Error: Expected identifier, got '{}'", self.peek_val());
        }
        let val = self.peek_val(); // Fix: extract BEFORE advancing
        self.current += 1;
        val
    }

    fn advance(&mut self) { self.current += 1; }
    fn compile_type(&mut self) { self.advance(); }

    // --- Subroutine & Class Compilation ---

    pub fn compile_class(&mut self) {
        self.eat("class");
        self.class_name = self.eat_identifier();
        self.eat("{");

        while matches!(self.peek_val().as_str(), "static" | "field") {
            self.compile_class_var_dec();
        }

        while matches!(self.peek_val().as_str(), "constructor" | "function" | "method") {
            self.compile_subroutine();
        }

        self.eat("}");
    }

    pub fn compile_class_var_dec(&mut self) {
        let kind = self.peek_val();
        self.advance();
        let typed = self.peek_val();
        self.advance();
        let mut name = self.eat_identifier();

        self.table.add_class(SymbolicEntry::new(&name, &typed, &kind));

        while self.peek_val() == "," {
            self.eat(",");
            name = self.eat_identifier();
            self.table.add_class(SymbolicEntry::new(&name, &typed, &kind));
        }

        self.eat(";");
    }

    pub fn compile_subroutine(&mut self) {
        self.table.dump_subroutine(); // Reset subroutine table for new function

        let sub_type = self.peek_val(); // "constructor" | "function" | "method"
        self.advance();
        
        let _return_type = self.peek_val();
        self.advance();

        let sub_name = self.eat_identifier();

        // Methods inject implicit "this" as argument 0
        if sub_type == "method" {
            self.table.add_subroutine(SymbolicEntry::new("this", &self.class_name.clone(), "argument"));
        }

        self.eat("(");
        self.compile_parameter_list();
        self.eat(")");

        // Compile body
        self.eat("{");
        while self.peek_val() == "var" {
            self.compile_var_dec();
        }

        // Emit Function VM Header: function ClassName.SubroutineName nLocals
        let n_locals = self.table.count_kind_subroutine("local");
        self.result.push(format!("function {}.{} {}", self.class_name, sub_name, n_locals));

        // Emit Prologues based on type
        if sub_type == "constructor" {
            let n_fields = self.table.count_kind_class("field");
            self.result.push(format!("push constant {}", n_fields));
            self.result.push("call Memory.alloc 1".to_string());
            self.result.push("pop pointer 0".to_string());
        } else if sub_type == "method" {
            self.result.push("push argument 0".to_string());
            self.result.push("pop pointer 0".to_string());
        }

        self.compile_statements();
        self.eat("}");
    }

    pub fn compile_parameter_list(&mut self) {
        if self.peek_val() != ")" {
            let typed = self.peek_val();
            self.compile_type();
            let name = self.eat_identifier();
            self.table.add_subroutine(SymbolicEntry::new(&name, &typed, "argument"));

            while self.peek_val() == "," {
                self.eat(",");
                let typed = self.peek_val();
                self.compile_type();
                let name = self.eat_identifier();
                self.table.add_subroutine(SymbolicEntry::new(&name, &typed, "argument"));
            }
        }
    }

    pub fn compile_var_dec(&mut self) {
        self.eat("var");
        let typed = self.peek_val();
        self.compile_type();
        let mut name = self.eat_identifier();
        self.table.add_subroutine(SymbolicEntry::new(&name, &typed, "local"));

        while self.peek_val() == "," {
            self.eat(",");
            name = self.eat_identifier();
            self.table.add_subroutine(SymbolicEntry::new(&name, &typed, "local"));
        }

        self.eat(";");
    }

    pub fn compile_statements(&mut self) {
        while matches!(self.peek_val().as_str(), "let" | "if" | "while" | "do" | "return") {
            match self.peek_val().as_str() {
                "let" => self.compile_let(),
                "if" => self.compile_if(),
                "while" => self.compile_while(),
                "do" => self.compile_do(),
                "return" => self.compile_return(),
                _ => unreachable!(),
            }
        }
    }

    pub fn compile_let(&mut self) {
        self.eat("let");
        let var_name = self.eat_identifier();
        let is_array = self.peek_val() == "[";

        if is_array {
            self.eat("[");
            self.compile_expression();
            self.eat("]");
            
            // Push base address of array variable
            if let Some((_, entry)) = self.table.find(&var_name) {
                let segment = match entry.kind.as_str() {
                    "field" => "this",
                    s => s,
                };
                self.result.push(format!("push {} {}", segment, entry.index));
            }
            self.result.push("add".to_string());
        }

        self.eat("=");
        self.compile_expression();
        self.eat(";");

        if is_array {
            self.result.push("pop temp 0".to_string());
            self.result.push("pop pointer 1".to_string());
            self.result.push("push temp 0".to_string());
            self.result.push("pop that 0".to_string());
        } else if let Some((_, entry)) = self.table.find(&var_name) {
            let segment = match entry.kind.as_str() {
                "field" => "this",
                s => s,
            };
            self.result.push(format!("pop {} {}", segment, entry.index));
        }
    }

    pub fn compile_if(&mut self) {
        let label_num = self.current;
        self.eat("if");
        self.eat("(");
        self.compile_expression();
        self.eat(")");

        self.result.push("not".to_string());
        self.result.push(format!("if-goto IF_FALSE_{}", label_num));

        self.eat("{");
        self.compile_statements();
        self.eat("}");

        self.result.push(format!("goto IF_END_{}", label_num));
        self.result.push(format!("label IF_FALSE_{}", label_num));

        if self.peek_val() == "else" {
            self.eat("else");
            self.eat("{");
            self.compile_statements();
            self.eat("}");
        }

        self.result.push(format!("label IF_END_{}", label_num));
    }

    pub fn compile_while(&mut self) {
        let label_num = self.current;
        self.result.push(format!("label WHILE_EXP_{}", label_num));

        self.eat("while");
        self.eat("(");
        self.compile_expression();
        self.eat(")");

        self.result.push("not".to_string());
        self.result.push(format!("if-goto WHILE_END_{}", label_num));

        self.eat("{");
        self.compile_statements();
        self.eat("}");

        self.result.push(format!("goto WHILE_EXP_{}", label_num));
        self.result.push(format!("label WHILE_END_{}", label_num));
    }

    pub fn compile_do(&mut self) {
        self.eat("do");
        self.compile_subroutine_call();
        self.eat(";");
        self.result.push("pop temp 0".to_string()); // Void return cleanup
    }

    pub fn compile_return(&mut self) {
        self.eat("return");

        if self.peek_val() != ";" {
            self.compile_expression();
        } else {
            self.result.push("push constant 0".to_string()); // Return 0 for void
        }

        self.eat(";");
        self.result.push("return".to_string());
    }

    pub fn compile_expression(&mut self) {
        self.compile_term();
        
        while self.is_op() {
            let op = self.peek_val();
            self.advance();
            self.compile_term();

            match op.as_str() {
                "+" => self.result.push("add".to_string()),
                "-" => self.result.push("sub".to_string()),
                "*" => self.result.push("mut".to_string()),
                "/" => self.result.push("div".to_string()),
                "&" => self.result.push("and".to_string()),
                "|" => self.result.push("or".to_string()),
                "<" => self.result.push("lt".to_string()),
                ">" => self.result.push("gt".to_string()),
                "=" => self.result.push("eq".to_string()),
                _ => {},
            }
        }
    }

    pub fn compile_term(&mut self) {
        if self.is_integer_constant() {
            self.result.push(format!("push constant {}", self.peek_val()));
            self.advance();
        } else if self.is_string_constant() {
            let s = self.peek_val();
            self.advance();
            self.result.push(format!("push constant {}", s.len()));
            self.result.push("call String.new 1".to_string());
            for c in s.chars() {
                self.result.push(format!("push constant {}", c as u32));
                self.result.push("call String.appendChar 2".to_string());
            }
        } else if self.is_keyword_constant() {
            match self.peek_val().as_str() {
                "true" => {
                    self.result.push("push constant 0".to_string());
                    self.result.push("not".to_string());
                }
                "false" | "null" => self.result.push("push constant 0".to_string()),
                "this" => self.result.push("push pointer 0".to_string()),
                _ => {}
            }
            self.advance();
        } else if self.peek_val() == "(" {
            self.eat("(");
            self.compile_expression();
            self.eat(")");
        } else if self.is_unary_op() {
            let op = self.peek_val();
            self.advance();
            self.compile_term();
            if op == "-" { self.result.push("neg".to_string()); }
            else if op == "~" { self.result.push("not".to_string()); }
        } else if self.is_identifier() {
            match self.peek_ahead_val().as_str() {
                "[" => {
                    let var_name = self.eat_identifier();
                    self.eat("[");
                    self.compile_expression();
                    self.eat("]");

                    if let Some((_, entry)) = self.table.find(&var_name) {
                        let segment = match entry.kind.as_str() {
                            "field" => "this",
                            s => s,
                        };
                        self.result.push(format!("push {} {}", segment, entry.index));
                    }
                    self.result.push("add".to_string());
                    self.result.push("pop pointer 1".to_string());
                    self.result.push("push that 0".to_string());
                }
                "(" | "." => self.compile_subroutine_call(),
                _ => {
                    let var_name = self.eat_identifier();
                    if let Some((_, entry)) = self.table.find(&var_name) {
                        let segment = match entry.kind.as_str() {
                            "field" => "this",
                            s => s,
                        };
                        self.result.push(format!("push {} {}", segment, entry.index));
                    }
                }
            }
        }
    }

    fn compile_subroutine_call(&mut self) {
        let name = self.eat_identifier();
        let mut n_args = 0;
        let mut func_name = String::new();

        if self.peek_val() == "." {
            self.eat(".");
            let sub_name = self.eat_identifier();

            if let Some((_, entry)) = self.table.find(&name) {
                // Method call on object instance (e.g., square.draw())
                let segment = match entry.kind.as_str() {
                    "field" => "this",
                    s => s,
                };
                self.result.push(format!("push {} {}", segment, entry.index));
                func_name = format!("{}.{}", entry.typed, sub_name);
                n_args = 1; // Implicit target object passed as argument 0
            } else {
                // Static call (e.g., Math.multiply())
                func_name = format!("{}.{}", name, sub_name);
            }
        } else {
            // Implicit method call on current class (e.g., draw())
            self.result.push("push pointer 0".to_string());
            func_name = format!("{}.{}", self.class_name, name);
            n_args = 1;
        }

        self.eat("(");
        n_args += self.compile_expression_list();
        self.eat(")");

        self.result.push(format!("call {} {}", func_name, n_args));
    }

    pub fn compile_expression_list(&mut self) -> u16 {
        let mut count = 0;
        if self.peek_val() != ")" {
            self.compile_expression();
            count += 1;
            while self.peek_val() == "," {
                self.eat(",");
                self.compile_expression();
                count += 1;
            }
        }
        count
    }
}