extern crate hyper;
extern crate select;

use std::io::Read;
use hyper::Client;
use std::fs::File;
//use std::fs::create_dir_all;

//use std::io::{BufWriter};

use select::document::Document;
use select::predicate::{ Class};

use std::error::Error;
use std::io::prelude::*;

use std::path::Path;

fn main() {
    let mut start = 0;
    while start < 900 {
        println!("start的值:{}",start);
       get_date(&mut start,);
    }
}

fn get_date(start: &mut i32){
    let client = Client::new();
    let url = "https://movie.douban.com/subject/1866479/reviews?start=";
    let num_str = format!("{0}{1}", url.to_string(), start);//拼接字符串

    //println!("url{}",num_str);
    let mut res = client.get(num_str.trim()).send().unwrap();
    /*assert_eq!(res.status, hyper::Ok);
    println!("headers:\n {}", res.headers);*/
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let   htmlbody= body.trim();
    /*  println!("body:\n {}", htmlbody);*/
    let document = Document::from(htmlbody);

    let mut content = String::from("");//单页数据
    for node in document.find(Class("main-bd")) {
        let question = node.find(Class("short-content")).next().unwrap();
        let qes_str = question.text();
        let  qes_str=  qes_str.replace("\n","").replace("\t","").replace("\r","").replace(" ","");
        let qes_str = format!("{0}{1}{2}", "start:", qes_str,"\r\n");//拼接字符串
       /* content.add(qes_str);*/
        &content.push_str(&qes_str);
    }
    let pathname = format!("{0}{1}{2}","./",start,"-page.txt" );//拼接字符串
    //println!("一页内容{}",&content);
    wtrie_file(&pathname,&mut content);
    *start+=20;
}
//写入文件
fn wtrie_file(path:&String ,conetent:&mut String){
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("创建文件失败{}: {}",display, why.description()),
        Ok(file) => file,
    };
    match file.write_all(conetent.as_bytes()) {
        Err(why) => {
            panic!("写入失败 {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("成功写入 {}", display),
    }
}

                    