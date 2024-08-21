pub fn clean_file(file_name: String) {
    std::fs::OpenOptions::new()
	.create(true)
	.write(true)
	.truncate(true)
	.open(file_name)
	.unwrap();
}

pub fn append_to_file(file_name: String, data: String) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(file_name)
        .unwrap();
    write!(file, "{}\n", &data).expect("Unable to write data");
}

pub fn trim_until(text: String, tag: String) -> String {
    let mut iter = text.chars();
    loop {
	if iter.clone().take(tag.len()).collect::<String>() == tag.to_string() {
	    break;
	}

	iter.next();
    }

    iter.skip(tag.len()).collect::<String>()
}

pub fn include_until(text: String, tag: String) -> String {
    let mut iter = text.chars();
    let mut answer = "".to_string();
    loop {
	if iter.clone().take(tag.len()).collect::<String>() == tag {
	    break;
	}

	answer.push(iter.next().unwrap());
    }

    answer
}

pub fn decompose(text: String, tag: &str) -> String {
    let start = format!("<{}>", tag);
    let end = format!("</{}>", tag);
    if text.len() < start.len() + end.len() {
	println!("Error en el texto a evaluar");
	std::process::exit(1);
    }

    include_until(trim_until(text, start), end).to_string()
}

pub fn process_marca(url: &str, file_name: &str) {
    let mut body = reqwest::blocking::get(url).unwrap().text().unwrap();
    clean_file(file_name.to_string());    
    for i in 0..15 {
	let mut title = decompose(body.clone(), "title");
	title = trim_until(title, "<![CDATA[".to_string());
	title = title.replace("]", "");
	title = title.replace(">", "");
	let link = decompose(body.clone(), "link");
	let page_link =
	    match i {
		0 => {
		    title = include_until(title, "/".to_string());
		    format!("<h3><a style=\"color:orange;\" href=\"{}\">{}</a></h3>", link, title)
		},
		_ => format!("<p><a style=\"color:cornsilk;\" href=\"{}\">{}</a></p>", link, title),
	};
	body = trim_until(body, "</link>".to_string()).to_string();	
	if i == 1 { continue; }
	append_to_file(file_name.to_string(), page_link);
    }
    append_to_file(file_name.to_string(), "<br><br>".to_string());
}

pub fn url_to_string(url: &str) -> String {
    let mut url = std::process::Command::new("curl")
	.arg(url)
	.arg("-s")
	.arg("-o")
	.arg("url.txt")
	.spawn()
	.expect("Error al descargar url");

    url.wait().unwrap();

    std::fs::read_to_string("url.txt")
	.unwrap()
}

pub fn txt_to_html(file_name: &str) {
    for line in std::fs::read_to_string(file_name).unwrap().split('\n').collect::<Vec<&str>>() {
	append_to_file("index2.html".to_string(), line.to_string());
    }
}
