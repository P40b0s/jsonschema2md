mod converter;
mod converter_error;
#[path="schema/lib.rs"]
mod schema;
#[path="traits/parser.rs"]
mod parser;
#[path="traits/markdown.rs"]
mod markdown;
#[path="pdf/pandoc.rs"]
mod pandoc;
mod object_type;
mod converter_type;
use std::env;

use colored::Colorize;
#[path="traits/value_type.rs"]
mod value_type;

fn main()
{
    //println!("{}", env::current_dir().unwrap().to_str().unwrap());
    let help = format!("Для обработки файла схемы необходимо запустить программу с параметрами: \n{} \n{} \n{}",
                                "Путь к файлу схемы, например: ../format/all.schema.json",
                                "Путь в файлу версий схемы, например: ../format/version.json",
                                "Путь для формируемого файла документации, например: ../format/documentation/format.schema.md").green();
    
    let schema_json = std::env::args().nth(1).expect(&help);
    let schema_version = std::env::args().nth(2).expect(&help);
    let output = std::env::args().nth(3).expect(&help);
    let converter = converter::JsonshchemaConverter::new(&schema_json, &schema_version, &output, converter_type::ConverterType::Markdown);
    if let Ok(mut schema) = converter
    {
        let md = schema.generate();
        if md.is_ok()
        {
            md.unwrap().write_data();
        }
        else 
        {
            println!("{} ({})", format!("{}", md.err().unwrap()).red(), schema_json);
        }
    }
    else 
    {
        println!("{} ({})", format!("{}", converter.err().unwrap()).red(), schema_json);
    }
}
