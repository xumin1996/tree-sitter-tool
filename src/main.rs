use std::{
    fs,
    io::{self, Read},
};

use clap::{Arg, Command};
use serde_json::{Value, json};
use tree_sitter::{Node, Parser};

fn main() {
    // 环境变量
    let matches = Command::new("tree-sitter-tools")
        .about("解析代码为string")
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_parser(clap::value_parser!(String))
                .help("编程语言(java,python,rust,sql,bash,js,ts,tsx,go,c,lua,html)"),
        )
        .arg(Arg::new("filename").value_parser(clap::value_parser!(String)))
        .get_matches();

    let content = matches
        .get_one::<String>("filename")
        .map(|file_name| {
            if "-" != file_name {
                return fs::read_to_string(file_name);
            } else {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf);
                return Ok(buf);
            }
        })
        .expect("file not exists")
        .expect("read fail");

    // 决定文件的编程语言
    let mut parser = Parser::new();
    if let Some(language) = matches.get_one::<String>("language") {
        if language == "java" {
            parser
                .set_language(&tree_sitter_java::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "rust" {
            parser
                .set_language(&tree_sitter_rust::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "python" {
            parser
                .set_language(&tree_sitter_python::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "sql" {
            parser
                .set_language(&tree_sitter_sequel::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "bash" {
            parser
                .set_language(&tree_sitter_bash::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "js" {
            parser
                .set_language(&tree_sitter_javascript::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "ts" {
            parser
                .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "tsx" {
            parser
                .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "go" {
            parser
                .set_language(&tree_sitter_go::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "c" {
            parser
                .set_language(&tree_sitter_c::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "lua" {
            parser
                .set_language(&tree_sitter_lua::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else if language == "html" {
            parser
                .set_language(&tree_sitter_html::LANGUAGE.into())
                .expect("tree_sitter_sequel init fail");
        } else {
            panic!("选择语言类型");
        }
    } else {
        panic!("选择语言类型");
    }

    // 解析
    let sql_tree = parser.parse(content.as_str(), None).unwrap();

    // 转换成json
    let json_value = node_to_json(&sql_tree.root_node(), content.as_str());
    println!("{}", json_value);
}

// 将Tree-sitter节点转换为可序列化的JSON结构
fn node_to_json(node: &Node, source_code: &str) -> Value {
    let mut map = serde_json::Map::new();

    // 基础节点信息
    map.insert(
        "type".to_string(),
        serde_json::to_value(node.kind()).unwrap(),
    );
    map.insert(
        "start_position".to_string(),
        serde_json::to_value({
            let pos = node.start_position();
            json!([pos.row, pos.column])
        })
        .unwrap(),
    );
    map.insert(
        "end_position".to_string(),
        serde_json::to_value({
            let pos = node.end_position();
            json!([pos.row, pos.column])
        })
        .unwrap(),
    );

    // 节点包含的源码片段
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let text = &source_code[start_byte..end_byte];
    map.insert("text".to_string(), serde_json::to_value(text).unwrap());

    // 处理子节点
    if node.child_count() > 0 {
        let mut children = Vec::new();
        let mut cursor = node.walk();

        if cursor.goto_first_child() {
            loop {
                let child_node = cursor.node();
                children.push(node_to_json(&child_node, source_code));

                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            // 返回父节点
            cursor.goto_parent();
        }

        map.insert(
            "children".to_string(),
            serde_json::to_value(children).unwrap(),
        );
    }

    serde_json::Value::Object(map)
}
