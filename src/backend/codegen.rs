use crate::parser::ast::{Expr, Literal, Op};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct CodeGenerator {
    output: String,
    data_section: String,
    variables: HashMap<String, (String, String)>,
    scopes: Vec<HashMap<String, (String, String)>>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::from(""),
            data_section: String::from("section .data\n"),
            variables: HashMap::new(),
            scopes: vec![HashMap::new()],
        }
    }

    // スコープをプッシュするメソッド
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // スコープをポップするメソッド
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    // 現在のスコープを取得するメソッド
    // fn current_scope(&mut self) -> Option<&mut HashMap<String, (String, String)>> {
    //     self.scopes.last_mut()
    // }

    pub fn generate(&mut self, expr: &Expr) -> Result<String, String> {
        // データセクション呼び出し前に変数を初期化しておく必要がある
        self.initialize_variables(expr)?;
        println!("Before generation: {}", self.data_section);
        self.output.clear();
        self.output.push_str("extern printf\n");
        self.output.push_str("section .bss\nbuffer_0 resb 12\n");
        self.output.push_str(&self.data_section);
        self.output
            .push_str("section .text\nglobal _start, int_to_ascii\n");

        self.int_to_ascii();

        self.emit_function_definitions(expr)?;

        self.output.push_str("_start:\n");
        self.preprocessor(expr)?;
        self.output.push_str("mov rax, 60\nxor rdi, rdi\nsyscall\n");
        println!("After generation: {}", self.output);
        Ok(self.output.clone())
    }

    fn int_to_ascii(&mut self) {
        self.output.push_str("int_to_ascii:\n");
        self.output.push_str("    push rbx\n");
        self.output.push_str("    mov rbx, rdi\n");
        self.output.push_str("    lea rsi, [rel buffer_0 + 11]\n");
        self.output.push_str("    mov byte [rsi], 0\n");
        self.output.push_str("    sub rsi, 1\n");
        self.output.push_str("    mov rcx, 10\n");
        self.output.push_str("convert_loop:\n");
        self.output.push_str("    xor rdx, rdx\n");
        self.output.push_str("    div rcx\n");
        self.output.push_str("    add dl, '0'\n");
        self.output.push_str("    mov [rsi], dl\n");
        self.output.push_str("    test rax, rax\n");
        self.output.push_str("    jz convert_end\n");
        self.output.push_str("    sub rsi, 1\n");
        self.output.push_str("    mov rbx, rax\n");
        self.output.push_str("    jmp convert_loop\n");
        self.output.push_str("convert_end:\n");
        self.output.push_str("    pop rbx\n");
        self.output.push_str("    ret\n");
    }

    pub fn generate_to_file(&mut self, expr: &Expr, file_path: &str) -> Result<(), String> {
        self.generate(expr)?;
        let mut file =
            File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(self.output.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }

    // generate関数で一回しか呼ばれない
    fn initialize_variables(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Assignment {
                name,
                type_decl,
                value,
            } => {
                println!("Initializing variable '{}'", name);
                let var_address = self.allocate_variable(name, type_decl)?;
                self.emit_assignment(&var_address, type_decl, value)?;
            }
            Expr::Variable(name) => {
                println!("Referencing variable '{}' during initialization", name);
                self.emit_variable(name)?;
            }
            Expr::Block(expressions) => {
                println!("Initializing block");
                for expression in expressions {
                    self.initialize_variables(expression)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_function_definitions(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Block(expressions) = expr {
            for expression in expressions {
                if let Expr::FunctionDef { name, params, body } = expression {
                    self.emit_function_def(name, params, body)?;
                }
            }
        }
        Ok(())
    }

    pub fn preprocessor(&mut self, expr: &Expr) -> Result<String, String> {
        println!("Processing expression: {:?}", expr);
        match expr {
            Expr::FunctionCall { name, args } => {
                self.emit_function_call(name, args)?;
            }
            Expr::IfExpr {
                condition,
                consequence,
                alternative,
            } => {
                self.emit_if_expr(condition, consequence, alternative)?;
            }
            Expr::WhileLoop { condition, body } => {
                println!(
                    "Expr While Debug: Entering while loop with condition: {:?}",
                    condition
                );
                self.emit_while_loop(condition, body)?;
                println!("Expr While Debug: Exiting while loop");
            }
            Expr::BinaryOp { left, op, right } => {
                println!("Processing BinaryOp: {:?} {:?} {:?}", left, op, right);
                self.emit_binary_op(left, op, right)?;
            }
            Expr::Literal(lit) => {
                self.emit_literal(lit)?;
            }
            Expr::Block(expressions) => {
                println!(
                    "Processing block with {} expressions(preprocessor func)",
                    expressions.len()
                );
                for expression in expressions {
                    self.preprocessor(expression)?;
                }
            }
            Expr::Return(expr) => {
                self.emit_return(expr)?;
            }
            Expr::Print(expr) => {
                println!("Processing print statement.");
                self.emit_print(expr)?;
            }
            _ => println!("Processing other expression types"),
        }
        Ok(self.output.clone())
    }

    fn emit_function_def(
        &mut self,
        name: &str,
        params: &[(String, String)],
        body: &Expr,
    ) -> Result<(), String> {
        self.output.push_str(&format!("{}:\n", name));
        self.output.push_str("    push rbp\n");
        self.output.push_str("    mov rbp, rsp\n");

        // 新しいスコープをプッシュし、各パラメータをスタックフレームに割り当てる
        self.push_scope();
        let mut offset = 16;
        for (param_name, param_type) in params.iter() {
            //let data_type = if param_type == "i64" { 8 } else { 4 };
            let data_type = 8;
            let param_label = format!("{}_res", param_name);
            self.variables
                .insert(param_label, (format!("rbp+{}", offset), param_type.clone()));
            offset += data_type;
        }

        self.preprocessor(body)?;

        self.output.push_str("    pop rbp\n");
        self.output.push_str("    ret\n");

        self.pop_scope();
        Ok(())
    }

    fn emit_function_call(&mut self, name: &str, args: &[Expr]) -> Result<(), String> {
        println!("Emitting function call to '{}'", name); // 関数呼び出しのデバッグ情報
        for (i, arg) in args.iter().enumerate().rev() {
            println!("Evaluating argument {}: {:?}", args.len() - i, arg); // 引数評価の前に情報を出力
            self.preprocessor(arg)?;
            self.output.push_str("    push rax\n");
            println!("Pushed argument {} to stack", args.len() - i); // スタックにプッシュ後の情報を出力
        }
        self.output.push_str(&format!("    call {}\n", name));
        self.output
            .push_str(&format!("    add esp, {}\n", args.len() * 4));

        println!("Function '{}' called with {} arguments", name, args.len()); // 関数呼び出し後の情報を出力
        Ok(())
    }

    fn emit_if_expr(
        &mut self,
        condition: &Expr,
        consequence: &Expr,
        alternative: &Option<Box<Expr>>,
    ) -> Result<(), String> {
        let use_64bit = self.use_64bit_regs(condition);
        let cmp_reg = if use_64bit { "rax" } else { "eax" };

        let label_else = self.new_label("else");
        let label_end = self.new_label("endif");

        self.preprocessor(condition)?;

        // 結果が0かどうかを確認して適切なブロックへジャンプ
        self.output
            .push_str(&format!("    test {}, {}\n", cmp_reg, cmp_reg));
        self.output.push_str(&format!("    je {}\n", label_else));

        self.preprocessor(consequence)?;
        self.output.push_str(&format!("    jmp {}\n", label_end));

        self.output.push_str(&format!("{}:\n", label_else));
        if let Some(alt) = alternative {
            self.preprocessor(alt)?;
        }

        self.output.push_str(&format!("{}:\n", label_end));
        Ok(())
    }

    fn emit_while_loop(&mut self, condition: &Expr, body: &Expr) -> Result<(), String> {
        let label_start = self.new_label("start");
        let label_end = self.new_label("end");

        // ループ開始ラベル
        self.output.push_str(&format!("{}:\n", label_start));

        // 条件式の評価とループ終了のジャンプ
        self.preprocessor(condition)?;

        self.output.push_str(&format!("    jge {}\n", label_end));

        // ループ本体の処理
        self.process_loop_body(body)?;

        // ループの開始へ戻る
        self.output.push_str(&format!("    jmp {}\n", label_start));

        // ループ終了ラベル
        self.output.push_str(&format!("{}:\n", label_end));

        Ok(())
    }

    fn process_loop_body(&mut self, body: &Expr) -> Result<(), String> {
        if let Expr::Block(expressions) = body {
            for expression in expressions {
                match expression {
                    Expr::Assignment { name, value, .. } => {
                        self.process_assignment_in_loop(name, value)?;
                    }
                    _ => {
                        self.preprocessor(expression)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn process_assignment_in_loop(&mut self, name: &str, value: &Expr) -> Result<(), String> {
        match value {
            Expr::Literal(Literal::I32(i)) => {
                self.output
                    .push_str(&format!("    mov eax, [{}_res]\n", name));
                self.output.push_str(&format!("    add eax, {}\n", i));
                self.output
                    .push_str(&format!("    mov [{}_res], eax\n", name));
            }
            _ => {
                self.preprocessor(value)?;
                self.output
                    .push_str(&format!("    mov [{}_res], eax\n", name));
            }
        }
        Ok(())
    }

    fn allocate_variable(
        &mut self,
        name: &str,
        type_decl: &Option<String>,
    ) -> Result<String, String> {
        // 変数名に_resを追加する際に既に_resが含まれているかどうかを確認
        let var_label = if name.ends_with("_res") {
            name.to_string()
        } else {
            format!("{}_res", name)
        };

        // variablesにvar_name_resと登録される
        if !self.variables.contains_key(&var_label) {
            // 型宣言がない場合はエラーを返す
            let data_type = match type_decl {
                Some(t) => t.as_str(),
                None => {
                    return Err(format!(
                        "Error: No type declaration provided for variable '{}'",
                        name
                    ))
                }
            };

            println!("Allocating new variable '{}'", var_label);
            self.data_section.push_str(&format!(
                "{} {} 0\n",
                var_label,
                if data_type == "i64" { "dq" } else { "dd" }
            ));
            self.variables.insert(
                var_label.clone(),
                (var_label.clone(), data_type.to_string()),
            );
            println!(
                "Variable '{}' allocated with type '{}'",
                var_label, data_type
            );
        } else {
            println!("Variable '{}' already allocated", var_label);
        }

        // 正常に変数が確保された場合、変数のアドレスを返す
        Ok(self.variables.get(&var_label).unwrap().0.clone())
    }

    fn emit_assignment(
        &mut self,
        name: &str,
        type_decl: &Option<String>,
        value: &Expr,
    ) -> Result<(), String> {
        let var_address = self.allocate_variable(name, type_decl)?;
        match value {
            Expr::Literal(Literal::I32(i)) => {
                if let Some("i32") = type_decl.as_deref() {
                    let existing_def = format!("{} dd 0", var_address);
                    let new_def = format!("{} dd {}", var_address, i);
                    self.data_section = self.data_section.replace(&existing_def, &new_def);
                } else {
                    return Err("Type mismatch: expected i64, found i32".to_string());
                }
            }
            Expr::Literal(Literal::I64(i)) => {
                if let Some("i64") = type_decl.as_deref() {
                    let existing_def = format!("{} dq 0", var_address);
                    let new_def = format!("{} dq {}", var_address, i);
                    self.data_section = self.data_section.replace(&existing_def, &new_def);
                } else {
                    return Err("Type mismatch: expected i32, found i64".to_string());
                }
            }
            _ => return Err("Unsupported assignment type".to_string()),
        }
        println!(
            "Data section after assignment ({}): {}",
            type_decl.as_ref().unwrap_or(&"i32".to_string()),
            self.data_section
        );
        Ok(())
    }

    fn emit_variable(&mut self, name: &str) -> Result<(), String> {
        let var_label = format!("{}_res", name);
        // スコープチェーンを逆順で調べ、最初に見つかった変数のアドレスを使用
        for scope in self.scopes.iter().rev() {
            if let Some((address, _)) = scope.get(&var_label) {
                println!("Variable '{}' found at address '{}'", name, address);
                self.output.push_str(&format!("mov rax, [{}]\n", address));
                return Ok(());
            }
        }
        Err(format!("Variable '{}' not defined", name))
    }

    // 演算時に使用するレジスタのビット幅を決定するための関数
    // variablesに登録されている変数の名前は変数名_resとなっている
    fn use_64bit_regs(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Variable(name) => {
                let result = self.variables.get(&format!("{}_res", name)).map_or_else(
                    || {
                        println!("Variable '{}' not found in registry.", name);
                        false
                    },
                    |(_, type_decl)| {
                        println!(
                            "Checking type for variable '{}': type_decl={}",
                            name, type_decl
                        );
                        type_decl == "i64"
                    },
                );
                println!("Variable '{}' is using 64-bit registers: {}", name, result);
                result
            }
            Expr::Literal(Literal::I64(_)) => {
                println!("Literal is I64, using 64-bit registers.");
                true
            }
            _ => {
                println!("Expression is not using 64-bit registers.");
                false
            }
        }
    }

    fn emit_binary_op(&mut self, left: &Expr, op: &Op, right: &Expr) -> Result<(), String> {
        println!(
            "emit_binary_op called with left: {:?}, op: {:?}, right: {:?}",
            left, op, right
        );

        let (reg_left, reg_right, reg_result) =
            if self.use_64bit_regs(left) || self.use_64bit_regs(right) {
                println!("Using 64-bit registers for operation");
                ("rbx", "rcx", "rax")
            } else {
                println!("Using 32-bit registers for operation");
                ("ebx", "ecx", "eax")
            };

        println!(
            "Selected registers: reg_left={}, reg_right={}, reg_result={}",
            reg_left, reg_right, reg_result
        );

        // 左辺の評価とレジスタへのロード
        self.load_expr_to_register(left, reg_left)?;

        // 右辺の評価とレジスタへのロード
        self.load_expr_to_register(right, reg_right)?;

        // 演算の実行
        match op {
            Op::Add | Op::Subtract | Op::Multiply | Op::Divide => {
                if op == &Op::Add {
                    self.output
                        .push_str(&format!("    add {}, {}\n", reg_left, reg_right));
                } else if op == &Op::Subtract {
                    self.output
                        .push_str(&format!("    sub {}, {}\n", reg_left, reg_right));
                } else if op == &Op::Multiply {
                    self.output
                        .push_str(&format!("    imul {}, {}\n", reg_left, reg_right));
                } else if op == &Op::Divide {
                    self.output.push_str("    xor edx, edx\n");
                    self.output.push_str(&format!("    idiv {}\n", reg_right));
                }
                // 演算結果を結果用レジスタに格納
                self.output
                    .push_str(&format!("    mov {}, {}\n", reg_result, reg_left));
            }
            Op::LessThan | Op::GreaterThan => {
                self.output
                    .push_str(&format!("    cmp {}, {}\n", reg_left, reg_right));
                if op == &Op::LessThan {
                    self.output.push_str("    setl al\n");
                } else if op == &Op::GreaterThan {
                    self.output.push_str("    setg al\n");
                }
                self.output.push_str("    movzx eax, al\n");
            } // _ => {
              //     return Err(format!("Unsupported operation '{:?}'", op));
              // }
        }

        Ok(())
    }

    fn load_expr_to_register(&mut self, expr: &Expr, register: &str) -> Result<(), String> {
        match expr {
            Expr::Variable(name) => {
                let var_name = format!("{}_res", name);
                if let Some((address, _)) = self.variables.get(&var_name) {
                    self.output
                        .push_str(&format!("    mov {}, [{}]\n", register, address));
                } else {
                    return Err(format!("Variable '{}' not defined", name));
                }
            }
            Expr::Literal(Literal::I32(i)) => {
                self.output
                    .push_str(&format!("    mov {}, {}\n", register, i));
            }
            Expr::Literal(Literal::I64(i)) => {
                self.output
                    .push_str(&format!("    mov {}, {}\n", register, i));
            }
            Expr::BinaryOp { left, op, right } => {
                self.emit_binary_op(left, op, right)?;
                self.output
                    .push_str(&format!("push rax\nmov {}, rax\n", register));
            }
            _ => {
                return Err("Unsupported expression type for loading to register".to_string());
            }
        }
        Ok(())
    }

    fn emit_literal(&mut self, lit: &Literal) -> Result<(), String> {
        match lit {
            Literal::I32(i) => {
                self.output.push_str(&format!("    mov eax, {}\n", i));
            }
            Literal::I64(i) => {
                self.output.push_str(&format!("    mov rax, {}\n", i));
            }
            Literal::String(s) => {
                let label = self.new_label("str");
                self.output.push_str(&format!(
                    "    .data\n{}:\n    .string \"{}\"\n    .text\n    lea rax, [{}]\n",
                    label, s, label
                ));
            }
            Literal::Unit => {}
        }
        Ok(())
    }

    fn emit_return(&mut self, expr: &Expr) -> Result<(), String> {
        self.preprocessor(expr)?;
        self.output.push_str("    ret\n");
        Ok(())
    }

    fn new_label(&self, base: &str) -> String {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("{}_{}", base, count)
    }

    fn emit_print(&mut self, expr: &Expr) -> Result<(), String> {
        println!("Emitting print function for expression: {:?}", expr);

        let use_64bit = match expr {
            Expr::Variable(name) => self.use_64bit_regs(&Expr::Variable(name.clone())),
            _ => self.use_64bit_regs(expr),
        };

        let reg = if use_64bit { "rax" } else { "eax" };

        match expr {
            Expr::Literal(Literal::I32(i)) => {
                self.output.push_str(&format!("    mov {}, {}\n", reg, i));
            }
            Expr::Literal(Literal::I64(i)) => {
                self.output.push_str(&format!("    mov {}, {}\n", reg, i));
            }
            Expr::Literal(Literal::String(s)) => {
                let label = self.new_label("str");
                self.output.push_str(&format!(
                    "    .section .rodata\n{}:\n    .ascii \"{}\\0\"\n",
                    label, s
                ));
                self.output.push_str(&format!("    lea rsi, [rel {}]\n    mov edi, 1\n    mov eax, 1\n    mov edx, {}\n    syscall\n", label, s.len() + 1));
            }
            Expr::Variable(name) => {
                let var_name = format!("{}_res", name);
                if let Some((var_address, _)) = self.variables.get(&var_name) {
                    println!(
                        "Printing variable '{}', found at address '{}'",
                        name, var_address
                    );
                    self.output
                        .push_str(&format!("    mov {}, [{}]\n", reg, var_address));
                } else {
                    println!("Error: Variable '{}' not found during print", name);
                    return Err(format!("Variable '{}' not defined", name));
                }
            }
            Expr::BinaryOp { left, op, right } => {
                self.emit_binary_op(left, op, right)?;
            }
            _ => println!("Unsupported expression type in print"),
        }

        self.output.push_str(&format!("    mov {}, {}\n    lea rsi, [rel buffer_0]\n    call int_to_ascii\n    lea rsi, [rel buffer_0]\n    mov edi, 1\n    mov eax, 1\n    mov edx, 12\n    syscall\n", reg, reg));
        Ok(())
    }
}
