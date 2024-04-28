use std::io::{self, Result};

use crate::db::create_db;
use crate::scripting::lexer;

pub fn handle_command_arguments() -> Result<()> 
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 
    {
        match args[1].as_str() 
        {
            "create" => create_db_with_console()?,
            "lexer" => analyze_lexically()?,
            _ => todo!()
        }
    }
    else
    {
        print_help_section()?;
    }
    Ok(())
}

fn print_help_section() -> Result<()>
{
    let help_list = 
    r#"Blaze Db 0.0.1a
Available commands:
    lexer   - try the first version of Blaze Language Lexer
    create  - create a new datablaze"#;
    println!("{}", help_list);
    Ok(())
}

pub fn create_db_with_console() -> Result<()> {
    let mut path = String::new();
    println!("Specify a path to a datablaze");
    _ = io::stdin().read_line(&mut path);
    let path = path.trim();
    create_db::create_db_structure(path, true)?;

    Ok(())
}

fn analyze_lexically () -> Result<()> 
{
    let mut code_to_parse = String::new();
    std::io::stdin().read_line(&mut code_to_parse)?;
    code_to_parse = code_to_parse.trim().to_string();
    lexer::Lexer::new(code_to_parse, "Buffer".to_string()).analyze()?;
    Ok(())
}