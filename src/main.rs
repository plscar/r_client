use std::{
    io::prelude::*,
    net::TcpStream
    };
///用于请求时的请求头
struct RequestHead
{
    method:String,
    url:String,
    BodyPara:String,
    Accept:String,
    AcceptEncoding:String,
    AcceptLanguage:String,
    // ContentLength:String,
    ContentType:String,
    Connection:String,
    Cookie:String,
    Host:String,
    UserAgent:String,
}

impl RequestHead
{
    ///将requesthead转换为字符串
    fn to_string(&self)->String{
        let mut mystring=String::new();
        mystring=mystring+&self.method+" "+&self.url+" HTTP/1.1"+"\r\n";
        mystring=mystring+"Accept:"+&self.Accept+"\r\n";
        mystring=mystring+"Accept-Encoding:"+&self.AcceptEncoding+"\r\n";
        mystring=mystring+"Accept-Language:"+&self.AcceptLanguage+"\r\n";
        mystring=mystring+"Content-Type:"+&self.ContentType+"\r\nContent-Length:{}\r\n";
        mystring=mystring+"Connection:"+&self.Connection+"\r\n";
        mystring=mystring+"Cookie:"+&self.Cookie+"\r\n";
        mystring=mystring+"Host:"+&self.Host+"\r\n";
        mystring=mystring+"User-Agent:"+&self.UserAgent+"\r\n\r\n";
        mystring=mystring+&self.BodyPara;
        let lent=15+mystring.len();
        let lent_s=lent+0;
        let lstr= (lent_s.to_string().len()+lent).to_string();
        println!("mystring=>{}",lstr);
        format!(mystring,&lstr)
    }
}

fn post_req(path:String,para:String,mycookie:String,tarhost:String)->String
{
    let myreques=RequestHead
    {
        method:"POST".to_string(),
        url:path,
        BodyPara:para,
        Accept:"text/plain, */*; q=0.01".to_string(),
        AcceptEncoding:"gzip, deflate".to_string(),
        AcceptLanguage:"en-US,en;q=0.5".to_string(),
        Connection:"keep-alive".to_string(),
        Cookie:mycookie,
        Host:tarhost,
        UserAgent:"Mozilla/5.0 Gecko/20100101 Firefox/59.0".to_string(),
        ContentType:"application/x-www-form-urlencoded; charset=UTF-8".to_string(),
    };
    myreques.to_string()
}

fn main() {
    let mut stream=TcpStream::connect("118.31.18.42:80").unwrap();
    let p=post_req("/Tools/handler/GetUser.ashx".to_string(), format!("action={}&code={}&password={}&remember={}&username={}","LoginUser","19362","adfas","false","aaaaa"), 
    "ASP.NET_SessionId=4ssozgzdqdjuzv45j0hyvn55".to_string(), "118.31.18.42:80".to_string());
    let s=p.as_bytes();
    stream.write(s);
    let mut res=String::new();
    stream.read_to_string(&mut res);
    println!("{}",res);

}
