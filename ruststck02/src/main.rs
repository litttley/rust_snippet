extern crate hyper;
extern crate serde_json;
use hyper::Client;
use std::io::Read;
use serde_json::{json, Value};
#[warn(unused_imports)]
use std::thread;
#[warn(unused_imports)]
use std::time::{Duration};

//Find-Package -Name "openssl" -Provider "Chocolatey" | Install-Package

//20190416 002017 东信和平  500 成本：16.724 成交  17.05 300 17.01 200  总亏赢 ：+155

fn main() {

    loop {

        let client = Client::new();
        let code="002017,002024,002175,000735,002157";
        let url = "https://api.shenjian.io/?appid=a92aa4aaddf00bf1b7637dbb73cf82677e15f&codes=";
        let cost_price=json!({"002017":"16.724","002024":"14.419","002175":"6.983","000735":"12.526","002157":"0"});
        let stock_count=json!({"002017":"0","002024":"300","002175":"300","000735":"200","002157":"0"});
        let url = format!("{0}{1}", url.to_string(), code);//拼接字符串

        //println!("url{}",num_str);
        let mut res = client.get(url.trim()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let jsonobj: Value  =  serde_json::from_str(body.trim()).expect("字符串转json失败");
        //   println!("{}", serde_json::to_string_pretty(&jsonobj).unwrap());
        let data_array = jsonobj["data"].as_array().expect("字符串转数据");
        println!("###################################start###################################################\n");
        for x in data_array.iter() {
            let price_now =  x["price"].as_str().unwrap();
            let price_name =  x["name"].as_str().unwrap();
            /*let mut  price_name="";
               if let  Some(t) = x["name"].as_str(){
                     price_name=t;
                }else {
                    println!("字符串转str:39行")
               }
            println!("price_name2{}",price_name2);*/
            let stock_code =  x["code"].as_str().expect("字符串转str");
            let data =  x["date"].as_str().unwrap();
            let time = x["time"].as_str().unwrap();
            let date_time=   format!("{0}{1}{2}",data," ",time);

            //println!("dd{}",price_now);

            let stock_code = &stock_code[2..8];
            let buy_price=   cost_price[stock_code.to_uppercase()].as_str().unwrap();
            let buy_count =   stock_count[stock_code.to_uppercase()].as_str().unwrap();
            //  println!("股票({}):{},当前价格：{};我的买入价:{},买入数量:{}",stock_code,price_name,price_now,buy_price,buy_count);

            let num_price_now =  price_now.parse::<f32>().unwrap();
            let num_buy_price = buy_price.parse::<f32>().unwrap();
            let num_count:f32 = buy_count.parse::<f32>().unwrap();
            let  mut cross_profit=0.0;
            let mut  avg_cross_profit:f32=0.0;
            if num_count != 0.0{
                cross_profit = (num_price_now- num_buy_price)*num_count;
                avg_cross_profit =cross_profit/(num_buy_price*num_count)*100 as f32;
            }

            println!("股票({}):{},当前价格：{};我的买入价:{},买入数量:{};总亏盈:{},百分比：{};时间:{}\n",stock_code,price_name,price_now,buy_price,buy_count,cross_profit,avg_cross_profit,date_time);


        }
        println!("#######################################end###############################################\n");
        let five_seconds = Duration::new(2, 0);
        thread::sleep(five_seconds);
    }
}
