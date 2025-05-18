# Article Analyzer

A Rust tool for processing a ZIP of JSON-encoded articles and extracting key statistics in parallel.

## What It Does

1. **Unpacks ZIP Archive**
   Reads `cod.zip`, loads each JSON file inside as a list of articles.

2. **Annotates Articles**
   Records the source filename for each article.

3. **Runs 4 Analyses Concurrently**

   * **Word Frequency (Exact Case):** Counts every distinct word as-is.
   * **Word Frequency (Lowercase):** Counts words in lowercase to merge variations.
   * **Longest Article:** Finds the article with the most characters in its text.
   * **Longest Title:** Finds the article with the longest title.

4. **Writes Results in Order**
   The four reports are collected from worker threads and appended in sequence to `output.txt`.


## Usage

* The program prints each processed filename to the console.
* When all threads finish, it writes four sections into `output.txt`:

  1. Word frequencies by exact case
  2. Word frequencies normalized to lowercase
  3. Details of the longest article (ID, length, title, path)
  4. Details of the article with the longest title

## How It Works

* **Thread Pool:** Spawns four threads, each calling one of the analysis functions.
* **Shared State:** Uses `Arc` + `Mutex` to collect results and write output safely.
* **Parsing:** A simple `serde_json` deserialization into `Article` structs.
* **Regex:** Extracts words with `\b[\w'-]+\b` pattern for frequency counts.
