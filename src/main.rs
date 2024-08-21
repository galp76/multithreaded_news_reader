use std::net::{ TcpListener, TcpStream };
use std::io::{ prelude::*, BufReader };
use std::fs;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:10000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
            handle_connection(stream);
        }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
	.lines()
	.map(|result| result.unwrap())
	.take_while(|line| !line.is_empty())
	.collect();

    // Imprimiendo HTTP_REQUEST para debugging
    println!("Contenido de HTTP_REQUEST: {:?}", http_request);
    let (status_line, filename) =
        if http_request.len() == 0 {
            run_multithreads();
	        ("HTTP/1.1 200 OK", "index.html")
        } else {
            let binding = http_request[0].clone();
            let parts: Vec<&str> = binding
                .split(" ")
                .collect();

            if parts[1] == "/" || parts[1] == "/?" {
                run_multithreads();
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.xhtml")
            }
        };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{}\r\nContent length: {}\r\n\r\n{}",
		       status_line,
		       length,
		       contents);
    
    stream.write_all(response.as_bytes()).unwrap();
}

fn run_multithreads() {
    multithreaded_rust_news_reader::clean_file("index2.html".to_string());
    multithreaded_rust_news_reader::append_to_file(
        "index2.html".to_string(),
        "<html><body style=\"background-color:midnightblue\">".to_string(),
    );

    // BBC MUNDO
    let t5 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::clean_file("bbc_mundo.txt".to_string());	
        let mut body = reqwest::blocking::get("https://www.bbc.co.uk/mundo/index.xml")
            .unwrap()
            .text()
            .unwrap();
        for i in 0..15 {
            let mut title = multithreaded_rust_news_reader::decompose(body.clone(), "title");
            title = multithreaded_rust_news_reader::trim_until(title, "<![CDATA[".to_string());
            title = title.replace("]", "");
            title = title.replace(">", "");
            let link = multithreaded_rust_news_reader::decompose(body.clone(), "link");
            let page_link = match i {
                0 => format!(
                    "<h3><a style=\"color:orange;\" href=\"{}\">{}</a></h3>",
                    link, title
                ),
                _ => format!(
                    "<p><a style=\"color:cornsilk\" href=\"{}\">{}</a></p>",
                    link, title
                ),
            };
            multithreaded_rust_news_reader::append_to_file("bbc_mundo.txt".to_string(), page_link);
            body = multithreaded_rust_news_reader::trim_until(body, "<item>".to_string()).to_string();
        }
        multithreaded_rust_news_reader::append_to_file("bbc_mundo.txt".to_string(), "<br><br>".to_string());
    });
    
    // REAL MADRID
    let t2 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::process_marca("https://e00-marca.uecdn.es/rss/futbol/real-madrid.xml", "real_madrid.txt");
    });

    // LA LIGA
    let t3 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::process_marca("https://e00-marca.uecdn.es/rss/futbol/primera-division.xml", "la_liga.txt");
    });

    // PREMIER LEAGUE
    let t4 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::process_marca("https://e00-marca.uecdn.es/rss/futbol/premier-league.xml", "premier_league.txt");
    });

    // BUNDESLIGA
    let t6 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::process_marca("https://e00-marca.uecdn.es/rss/futbol/bundesliga.xml", "bundesliga.txt");
    });

    // HACKER NEWS
    let t7 = std::thread::spawn(|| {
        multithreaded_rust_news_reader::clean_file("hackernews.txt".to_string());	
        let mut body = reqwest::blocking::get("https://news.ycombinator.com/rss")
            .unwrap()
            .text()
            .unwrap();
        for i in 0..30 {
            let title = multithreaded_rust_news_reader::decompose(body.clone(), "title");
            let link = multithreaded_rust_news_reader::decompose(body.clone(), "comments");
            let page_link = match i {
            0 => format!(
                "<h3><a style=\"color:orange;\" href=\"{}\">{}</a></h3>",
                link, title
            ),
            _ => format!(
                "<p><a style=\"color:cornsilk\" href=\"{}\">{}</a></p>",
                link, title
            ),
            };
            multithreaded_rust_news_reader::append_to_file("hackernews.txt".to_string(), page_link);
            body = multithreaded_rust_news_reader::trim_until(body, "</comments>".to_string()).to_string();
        }

        multithreaded_rust_news_reader::append_to_file("hackernews.txt".to_string(), "<br><br>".to_string());
    });
     
    t2.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("real_madrid.txt");
    t3.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("la_liga.txt");
    t4.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("premier_league.txt");
    t6.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("bundesliga.txt");
    t5.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("bbc_mundo.txt");
    t7.join().unwrap();
    multithreaded_rust_news_reader::txt_to_html("hackernews.txt");
    
    multithreaded_rust_news_reader::append_to_file("index2.html".to_string(), "<br><br>".to_string());

    multithreaded_rust_news_reader::append_to_file("index2.html".to_string(), "</body></html>".to_string());

    std::fs::rename("index2.html", "index.html").unwrap();
}
