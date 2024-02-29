use regex::Regex;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use zip::ZipArchive;

#[derive(Debug, Deserialize, Clone)]
struct Article {
    id: String,
    title: String,
    text: String,
    #[serde(skip_deserializing)] 
    path: String,
}

fn main() {
    let file = File::open("cod.zip").unwrap();
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader).unwrap();
    let mut articles: Vec<Article> = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_name = file.name().to_string();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut articles_in_file: Vec<Article> = match serde_json::from_str(&contents) {
            Ok(articles) => articles,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                continue;
            }
        };
        for article in &mut articles_in_file {
            article.path = file_name.clone();
        }

        articles.extend(articles_in_file);
        println!("Fisier procesat: {}", file_name);
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("output.txt")
        .unwrap();
    let file = Arc::new(Mutex::new(BufWriter::new(file)));

    let articles = Arc::new(articles);

    let results = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for i in 0..4 {
        let articles_clone = Arc::clone(&articles);
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let mut result = String::new();

            if i == 0 {
                result = fq_words_written(&articles_clone);
            } else if i == 1 {
                result = fq_words_lowercase(&articles_clone);
            } else if i == 2 {
                result = longest_article(&articles_clone);
            } else if i == 3 {
                result = longest_title_article(&articles_clone);
            }

            results_clone.lock().unwrap().insert(i, result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let results = results.lock().unwrap();
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by_key(|&(k, _)| k);

    let mut file = file.lock().unwrap();
    for (_, data) in sorted_results {
        writeln!(file, "{}", data).unwrap();
    }

    println!("Toate thread-urile s-au terminat si datele au fost scrise in ordine.");
}

fn fq_words_written(articles: &[Article]) -> String {
    let mut fq_words: HashMap<String, u32> = HashMap::new();
    let word_regex = Regex::new(r"\b[\w'-]+\b").unwrap();

    for article in articles {
        for word in word_regex.find_iter(&article.text) {
            let word = word.as_str();
            *fq_words.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut fq_words: Vec<(&String, &u32)> = fq_words.iter().collect();
    fq_words.sort_by(|a, b| b.1.cmp(a.1));

    let mut result = String::new();
    result.push_str("Frecventa cuvintelor scrise:\n");
    for (word, count) in fq_words {
        let line = format!("{}: {}\n", word, count);
        result.push_str(&line);
    }

    result
}

fn fq_words_lowercase(articles: &[Article]) -> String {
    let mut fq_words: HashMap<String, u32> = HashMap::new();
    let word_regex = Regex::new(r"\b[\w'-]+\b").unwrap();

    for article in articles {
        for word_match in word_regex.find_iter(&article.text) {
            let word = word_match.as_str().to_lowercase();
            *fq_words.entry(word).or_insert(0) += 1;
        }
    }

    let mut fq_words: Vec<(&String, &u32)> = fq_words.iter().collect();
    fq_words.sort_by(|a, b| b.1.cmp(a.1));

    let mut result = String::new();
    result.push_str("Frecventa cuvintelor scrise:\n");
    for (word, count) in fq_words {
        let line = format!("{}: {}\n", word, count);
        result.push_str(&line);
    }

    result
}

fn longest_article(articles: &Vec<Article>) -> String {
    let mut longest_article = &articles[0];
    for article in articles {
        if article.text.len() > longest_article.text.len() {
            longest_article = article;
        }
    }

    let result = format!(
        "Cel mai lung articol:\nID: {}\nLungime: {}\nTitlu: {}\nCale: {}\n",
        longest_article.id,
        longest_article.text.len(),
        longest_article.title,
        longest_article.path
    );
    
    result
    
}

fn longest_title_article(articles: &Vec<Article>) -> String {
    let mut longest_title_article = &articles[0];
    for article in articles {
        if article.title.len() > longest_title_article.title.len() {
            longest_title_article = article;
        }
    }

    let result = format!(
        "Cel mai lung titlu:\nID: {}\nLungime: {}\nTitlu: {}\nCale: {}\n",
        longest_title_article.id,
        longest_title_article.title.len(),
        longest_title_article.title,
        longest_title_article.path
    );
    
    result
    
}