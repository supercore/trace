use reqwest;
use std::env;

async fn get(symbols: &str) -> Result<String, reqwest::Error> {
    let mut url = String::from("http://sqt.gtimg.cn/utf8/q=");
    url.push_str(symbols);
    url.push_str("&offset=3,2,4,33,39,60,40,46,1");
    Ok(reqwest::get(url.as_str()).await?.text().await?)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(resp) = get(args[1].as_str()).await {
        let mut wants = Vec::new();
        let fld_name: Vec<&str> = vec![
            "ticker", "name", "price", "chg%", "tn%", "tn2%", "pe", "mv", "x",
        ];
        wants.push(fld_name);
        let rows: Vec<&str> = resp.split(';').collect();
        for v in rows.iter() {
            let r: Vec<&str> = v.split('"').collect();
            if r[0].len() > 4 {
                println!("{}", r[1]);
                let mut row: Vec<&str> = r[1].split('~').collect();
                match row[8] {
                    "100" => row[4] = row[5],
                    "1" | "51" | "200" => (),
                    _ => (),
                }
                wants.push(row);
            }
        }

        for w in wants.iter() {
            let s = format!(
                "{:<12}{:<12}{:<12}{:<12}{:<-12}{:<12}{:<12}",
                w[0], w[2], w[3], w[4], w[6], w[7], w[1]
            );
            println!("\n{}", s);
        }

    }
}
