fn main() {
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
