extern crate boardslam;

use std::env;
use std::collections::HashMap;
use boardslam::*;

fn main() {
    println!("Content-Type: text/html\n\n");
    let html = r#"
        <!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
        <html>
          <head>
            <title>Board Slam Calculator</title>
          </head>
          <body>
            <p><a href="/">seven1m.sdf.org</a></p>
            <h1>Board Slam Math Game Calculator</h1>
            <p><em>written April 2018 (updated Aug 2018)</em></p>
            <p>
              Classical Conversations (Homeschool) communities all over play a math game in the classroom called "Board Slam."
              Participants try to use 3 numbers determined by dice roll along with various mathematical operations to calculate
              the answers to fill a grid (the board) of one (1) through thirty-six (36) slots. This is great fun!
            </p>
            <p>
              But sometimes, students and parents alike get stuck, wondering if they've truly exhaused all possibilities.
              This page will show you possible ways to calculate the answers using the numbers given, and at the bottom of the
              page, it will tell you which answers seem to be impossible (if any).
            </p>
            <form>
              n1:
              <input name="n1" size="2" value="{n1}">

              n2:
              <input name="n2" size="2" value="{n2}">

              n3:
              <input name="n3" size="2" value="{n3}">

              <input type="submit" value="Go!">
            </form>
            <pre>{output}</pre>
            <hr>
            <p><i>Hosting for this site is provided by</i></p>
            <p><a href="http://sdf.org"><img src="http://sdf.org/sdfbanner.png"></a></p>
            <p><a href="http://sdf.org">The SDF Public Access UNIX System</a></p>
            <p><a href="http://validator.w3.org/check?uri=referer"><img src="http://www.w3.org/Icons/valid-html20"></a></p>
          </body>
        </html>
    "#;
    let query = env::var("QUERY_STRING").unwrap_or("".to_string());
    let args = args(query);
    let n1 = args.get("n1").unwrap_or(&"".to_string()).parse::<u8>().unwrap_or(0);
    let n2 = args.get("n2").unwrap_or(&"".to_string()).parse::<u8>().unwrap_or(0);
    let n3 = args.get("n3").unwrap_or(&"".to_string()).parse::<u8>().unwrap_or(0);
    let mut output;
    let final_html;
    if n1 == 0 || n2 == 0 || n3 == 0 {
        output = "Please specify 3 numbers between 1 and 6".to_string();
        final_html = html.replace("{n1}", "").replace("{n2}", "").replace("{n3}", "").replace("{output}", &output);
    } else {
        let results = fill_board(n1, n2, n3);
        let missing = get_missing(&results);
        let missing = missing
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        output = display(&results);
        output.push_str(&format!("\nmissing: {}", missing));
        final_html = html.replace("{n1}", &n1.to_string()).replace("{n2}", &n2.to_string()).replace("{n3}", &n3.to_string()).replace("{output}", &output);
    }
    println!("{}", final_html);
}

fn args(query: String) -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    let pairs = query.split("&");
    for pair in pairs {
        let parts: Vec<&str> = pair.split("=").collect();
        if parts.len() >= 2 {
            args.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    args
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_args() {
        let args = args("n1=1&n2=2&n3=3".to_string());
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert("n1".to_string(), "1".to_string());
        expected.insert("n2".to_string(), "2".to_string());
        expected.insert("n3".to_string(), "3".to_string());
        assert_eq!(args, expected);
    }
}
