use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// FDino Universal Transpiler 
/// Designed to cover all 18 core categories of Rust functionality.
fn main() {
    println!("--- FDino Compiler Engine: Full Capability Mode ---");

    let input_file = "main.fdino";
    let source_code = match fs::read_to_string(input_file) {
        Ok(code) => code,
        Err(_) => {
            println!("Error: Cannot find 'main.fdino'.");
            return;
        }
    };

    let mut dict: HashMap<&str, &str> = HashMap::new();

    // 1. DECLARATION & 9. CUSTOM TYPES & 11. TRAIT
    dict.insert("create variable", "let mut");
    dict.insert("create constant", "let");
    dict.insert("define task", "fn");
    dict.insert("blueprint", "struct");
    dict.insert("options", "enum");
    dict.insert("interface", "trait");

    // 2. ASSIGNMENT & 3. OPERATORS
    dict.insert(" is ", " = ");
    dict.insert(" plus is ", " += ");
    // Note: Standard operators like +, -, *, / stay the same in FDino for speed.

    // 4. CONDITION & 5. MATCHING
    dict.insert("if", "if");
    dict.insert("otherwise", "else");
    dict.insert("check match", "match");
    dict.insert("if let", "if let");

    // 6. LOOP & 7. CONTROL FLOW
    dict.insert("repeat forever", "loop");
    dict.insert("while true", "while");
    dict.insert("loop through", "for");
    dict.insert("in range", "in");
    dict.insert("stop loop", "break");
    dict.insert("skip to next", "continue");
    dict.insert("send back", "return");

    // 10. IMPLEMENTATION
    dict.insert("implement for", "impl");

    // 12. MODULE & IMPORT
    dict.insert("import", "use");
    dict.insert("as public", "pub");
    dict.insert("local", "super");

    // 13. OWNERSHIP & BORROWING
    dict.insert("borrow mutable", "&mut");
    dict.insert("borrow", "&");
    dict.insert("move ownership", "move");

    // 14. UNSAFE & 15. ASYNC
    dict.insert("unsafe zone", "unsafe");
    dict.insert("async task", "async fn");
    dict.insert("wait for", ".await");

    // 17. MACROS (Handling common ones)
    dict.insert("write line", "println!");
    dict.insert("write", "print!");
    dict.insert("panic now", "panic!");

    // PROCESSING ENGINE
    let mut rust_output = String::new();
    let mut keys: Vec<_> = dict.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));

    for line in source_code.lines() {
        let mut processed = line.to_string();
        if processed.trim().is_empty() { continue; }

        for key in &keys {
            if let Some(rust_val) = dict.get(*key) {
                processed = processed.replace(*key, rust_val);
            }
        }

        // 16. CLOSURE & 18. DESTRUCTURING 
        // Logic handles these naturally if the user writes them in FDino style
        // Example: "let (a, b) = (1, 2)" works because it's standard syntax.

        // Fix parenthesis for Write Macros
        if processed.contains("println!") || processed.contains("print!") {
            if !processed.contains("(") {
                processed = processed.replace("println! ", "println!(");
                processed = processed.replace("print! ", "print!(");
                processed.push(')');
            }
        }

        // Automatic Semicolon Injection
        let t = processed.trim();
        if !t.ends_with('{') && !t.ends_with('}') && !t.is_empty() && !t.ends_with(';') {
            processed.push(';');
        }
        
        rust_output.push_str(&processed);
        rust_output.push('\n');
    }

    let final_code = if !rust_output.contains("fn main") {
        format!("fn main() {{\n{}\n}}", rust_output)
    } else {
        rust_output
    };

    fs::write("target_code.rs", &final_code).expect("Write error");
    
    println!("System: Compiling all 18 categories to machine code...");
    let status = Command::new("rustc")
        .arg("target_code.rs")
        .arg("-O") 
        .status()
        .expect("Compiler failed.");

    if status.success() {
        println!("Final Build Successful: target_code.exe created.");
    } else {
        println!("Error: Compilation failed. Check your FDino logic.");
    }
}