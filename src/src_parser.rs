use pom::DataInput;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use parsers::declaration::declaration_list;
use pom::parser::end;
use std::path::Path;

use std::result::Result;
use symbols::collector::*;

use std::io::Read;

use time::PreciseTime;

use symbols::zSymb_Table::zSymbol_Table;
use symbols::zCPar_Symbol::zSymbol;



extern crate regex;
use glob::*;

pub fn parse_src(path: &str) -> String {

    let file = File::open(path).expect("failed to read src file");
    let breader = BufReader::new(&file);

    let dir = Path::new(path).parent().unwrap().to_string_lossy();

    let lines =
        breader.lines()
            .map(|s|s.unwrap().clone())
            .filter(|s|!s.is_empty())
            .map(|line|format!("{}/{}", dir, line))
            .collect::<Vec<String>>();

    let symbols = lines.iter().flat_map(|line|parse_glob(line).into_iter()).collect::<Vec<_>>();

    let mut table = zSymbol_Table::new(symbols);

    table.fold_constants();
    "".to_string()
}

pub fn parse_glob(line: &str) -> Vec<zSymbol> {
    let result = glob(line).expect("Failed to read glob pattern")
        .flat_map(|path|
                parse_file(&path.unwrap()).unwrap().into_iter()
            )
        .collect::<Vec<_>>();

    result

}


pub fn parse_file(path: &Path) -> Result<Vec<zSymbol>, &'static str> {
    let time_entry = PreciseTime::now();

    if path.file_name().unwrap().to_str().unwrap().ends_with(".src") { return Ok(Vec::new())}
    let input_vec = prepare_file(path);

    let time_after_prepare = PreciseTime::now();
    if input_vec.len() == 0 { return Ok(Vec::new()); }
    let mut input = DataInput::new(&input_vec);

    let parsed = (declaration_list() - end()).parse(&mut input);

    let time_after_parse = PreciseTime::now();

    /*let result = parsed.map(|decl_vec|
        decl_vec.iter().map(|decl|
            match *decl {
                Symbol::Func(ref f) => walk_func_decl(&f),
                Symbol::Var(ref v) => walk_var_decl_list(&v),
                Symbol::Class(ref c) => walk_class_decl(&c),
                Symbol::Inst(ref i) => walk_inst_decl(&i),
                Symbol::Proto(ref p) => walk_proto_decl(&p),
                Symbol::Const(ref c) => walk_const_decl(&c),
                Symbol::ConstArray(ref con_arr) => walk_const_array_decl(&con_arr)
            }
        ).collect::<Vec<_>>()
    );*/

    let result = parsed.expect("failed to parse")
        .iter_mut()
        .flat_map(|decl|
            collect_declaration(decl).into_iter()
        )
        .collect::<Vec<_>>();




    let time_after_collector = PreciseTime::now();

    println!("{}\nIt took {} seconds to prepare, {} seconds to parse and {} seconds to collect",
             path.to_str().unwrap(),
             time_entry.to(time_after_prepare),
             time_after_prepare.to(time_after_parse),
             time_after_parse.to(time_after_collector));

    Ok(result)
}


fn prepare_file(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();

    let mut byte_vec = Vec::new();

    file.read_to_end(&mut byte_vec);

    let no_comments = remove_comments(
        byte_vec
            .split(|c|
                *c == b'\n'
            )
            .map(|vec|vec.to_owned())
            .collect::<Vec<Vec<u8>>>()
    );


    let result = turn_uppercase(no_comments);

    //println!("hehe {}", result.iter().map(|c|*c as char).collect::<String>());
    result
}


fn remove_comments(lines: Vec<Vec<u8>>) -> Vec<u8>{
    let no_single_comments = remove_singleline_comments(lines);


    // 0 -> not in comment
    // 1 -> exiting comment
    // 2 -> exiting comment
    // 3 -> exiting comment
    // 4 -> in comment
    let mut in_multi_comment = 0;
    let mut no_multi_comments = Vec::new();

    if no_single_comments.len() == 0 {
        no_multi_comments == no_single_comments;
    } else {
        for i in 0..no_single_comments.len() {
            if i != no_single_comments.len()-1 { // Don't check at the last symbol
                if no_single_comments[i] == b'/' && no_single_comments[i + 1] == b'*' {
                    in_multi_comment = 4;
                } else if no_single_comments[i] == b'*' && no_single_comments[i + 1] == b'/' {
                    in_multi_comment = 2;
                }
            }


            if in_multi_comment == 0 {
                no_multi_comments.push(no_single_comments[i]);
            } else if in_multi_comment != 4 {
                in_multi_comment -= 1;
            }
        }
    }
    no_multi_comments
}

fn remove_singleline_comments(lines: Vec<Vec<u8>>) -> Vec<u8> {
    lines.iter().flat_map(|line| {

        let mut pos = None;
        if line.len() < 2 {

        } else {
            for i in 0..line.len() - 1 {
                if line[i] == b'/' && line[i + 1] == b'/' {
                    pos = Some(i);
                    break;
                }
            }
        }
        match pos {
            None => line.to_owned(),
            Some(pos) => { let mut vec = line[0..pos].to_owned(); vec.push(b'\n'); vec }

        }
    }).collect::<Vec<u8>>()
}

fn turn_uppercase(input: Vec<u8>) -> Vec<u8> {
    let mut in_string = false;
    let mapped = input.iter().fold(Vec::new(), |mut acc: Vec<u8>, c| {
        match *c {
            b'\"' => { in_string = !in_string; acc.push(b'\"') },

            ch => if in_string {
                acc.push(ch);
            } else {
                if ch >= b'a' && ch <= b'z' {
                    acc.push((ch as i32 - 32) as u8)
                } else {
                    if ch == 228 { acc.push(0xC4); } // ä -> Ä
                    else if ch == 252 { acc.push(0xDC) } // ü -> Ü
                    else if ch == 246 { acc.push(0xD6) } // ö -> Ö
                    else {
                        acc.push(ch)
                    };
                }
            }
        }
        acc
    });
    mapped
}