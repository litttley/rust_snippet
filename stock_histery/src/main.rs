extern crate hyper;
extern crate serde_json;
use hyper::Client;
use std::io::Read;
use serde_json::{ Value};
fn main() {
    /* 请求地址：https://api.shenjian.io/?appid=944bb9ddadf7491420bd8dc43f10370746d9
    请求参数：code=601857&index=false&k_type=day&fq_type=qfq&start_date=2016-04-10&end_date=
    请求方式：GET*/

    loop {
        println!("输入股票代码：");
        let mut stock_num = String::new();
        std::io::stdin().read_line(&mut stock_num).expect("输入失败");

        let client = Client::new();
        let code = stock_num.as_str();
        let start_date = "2017-04-2";
        let end_date = "";
        let url = format!("https://api.shenjian.io/?appid=944badddb9d7491420bd8dc43f10370746d9&code={0}&index=false&k_type=day&fq_type=qfq&start_date={1}&end_date={2}", code, start_date, end_date);//拼接字符串
        let mut res = client.get(url.as_str()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let jsonobj: Value = serde_json::from_str(body.trim()).expect("字符串转json失败");
        // println!("{}", serde_json::to_string_pretty(&jsonobj).unwrap());
        let data_array = jsonobj["data"].as_array().expect("json转array ：21行");
        println!("|{0:width$}|{1:width$}|{2:width$}|{3:width$}|{4:width$}|{5:width$}|{6:width$}", "日期", "证券代码", "开盘价", "收盘价", "最高价", "最低价", "成交量",width=12);
        for value in data_array.iter() {
            let date_value = value["date"].as_str().unwrap();
            let code_value = value["code"].as_str().unwrap();
            let open_value = value["open"].as_str().unwrap();
            let close_value = value["close"].as_str().unwrap();
            let high_value = value["high"].as_str().unwrap();
            let low_value = value["low"].as_str().unwrap();
            let volume_value = value["volume"].as_str().unwrap();
            println!("|{0:width$}|{1:width$}|{2:width$}|{3:width$}|{4:width$}|{5:width$}|{6:width$}", date_value, code_value, open_value, close_value, high_value, low_value, volume_value,width=15);

        }
    }

}
