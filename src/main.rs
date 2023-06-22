use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::Parser as ClapParser;
use std::collections::HashMap;
use ulid::Ulid;

use csv::WriterBuilder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut term_to_id: HashMap<String, String> = HashMap::new();
    let mut id_to_term: HashMap<String, String> = HashMap::new();
    let cli = Cli::parse();
    let config_path = if let Some(path) = cli.path.as_deref() {
        println!("Parsing md from path: {}", path.display());
        Path::new(path)
    } else {
        Path::new("/Users/alexandermikhalev/NewDigitalMind")
    };
    println!("{:?}", config_path);
    let binding = config_path.join("**/*.md");
    let glob_path = binding.to_str().unwrap();
    println!("{}", glob_path);
    for entry in glob(glob_path)? {
        let path = entry?;
        let concept = path
            .file_stem()
            .ok_or("Failed to get file stem")?
            .to_str()
            .ok_or("Failed to convert file stem to string")?;
        println!("Concept: {}", concept);
        let concept = concept.to_string().trim().to_lowercase();
        let ulid = Ulid::new().to_string();
        println!("Ulid for concept {}", ulid.clone());

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            // println!("{}", line);
            let (key, value) = match line.split_once("::") {
                Some((k, v)) => (k, v),
                None => continue,
            };
            // TODO: below are stubs to be used for filtering concepts, they require populated knowledge graph.
            if key == "type" {
                println!("Type {:?}", key);
                if value.contains("[[Concept]]") {
                    println!("Found concept {:?}", value);
                    term_to_id.insert(concept.clone(), ulid.clone());
                    id_to_term.insert(ulid.clone(), concept.clone());
                }
            }
            if key == "public" {
                println!("Public, we are good {:?}", value);
            }
            if key == "synonyms" {
                for term in value.split(',').collect::<Vec<&str>>() {
                    let synonym = term.trim().trim().to_lowercase();
                    term_to_id.insert(synonym.clone(), ulid.clone());
                }
            }
            println!("{}: {}", key, value);
        }
    }
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(GzEncoder::new(
            File::create("./data/term_to_id.csv.gz")?,
            Compression::default(),
        ));

    for (term, id) in term_to_id {
        writer.write_record(&[term, id])?;
    }
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(GzEncoder::new(
            File::create("./data/id_to_term.csv.gz")?,
            Compression::default(),
        ));

    for (id, term) in id_to_term {
        writer.write_record(&[id, term])?;
    }
    Ok(())
}