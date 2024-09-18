use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::process;
use std::str::FromStr;

use isbn3::{Isbn10, Isbn13};

enum FileOrStdin
{
    File(fs::File),
    Stdin(io::Stdin),
}

impl Read for FileOrStdin
{
    fn read(&mut self, buf : &mut [u8]) -> io::Result<usize>
    {
        match *self
        {
            FileOrStdin::File(ref mut file) => file.read(buf),
            FileOrStdin::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
struct SBARecord
{
    #[serde(rename = "Systematik")]
    _systematik :  String,
    #[serde(rename = "Kurzanzeige")]
    _kurzanzeige : String,
    #[serde(rename = "JahrAufl.")]
    _jahr_aufl :   String,
    #[serde(rename = "VerlagOrt")]
    _verlag_ort :  String,
    #[serde(rename = "ISBN")]
    isbn :         Option<String>,
}

fn parse_csv() -> Result<(), Box<dyn Error>>
{
    let inp_reader = match env::args().nth(1)
    {
        None => FileOrStdin::Stdin(io::stdin()),
        Some(filename) => match filename.as_ref()
        {
            "-" => FileOrStdin::Stdin(io::stdin()),
            _ => FileOrStdin::File(fs::File::open(filename)?),
        },
    };
    let mut csvrdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(inp_reader);
    let headers = csvrdr.headers()?.to_owned();
    eprintln!("Read the headers: {:?}", headers);
    let mut ctr : usize = 0;
    for result in csvrdr.records()
    {
        ctr += 1;
        let record : SBARecord = match result
        {
            Ok(strrec) => strrec.deserialize::<SBARecord>(Some(&headers))?,
            Err(err) =>
            {
                eprintln!("{}: {}", ctr, err);
                continue;
            },
        };
        match record.isbn
        {
            Some(isbnx) =>
            {
                eprintln!("\t{}: {:?}", ctr, isbnx);
                let isbn = match Isbn10::from_str(&isbnx)
                {
                    Ok(isbn10) => Isbn13::from(isbn10),
                    Err(_) => match Isbn13::from_str(&isbnx)
                    {
                        Err(_) =>
                        {
                            eprintln!("{} No ISBN: {:?}", ctr, isbnx);
                            continue;
                        },
                        Ok(isbn) => isbn,
                    },
                };
                println!("{}: {} ({:?})", ctr, isbn, isbnx);
            },
            None =>
            {
                eprintln!("{} No ISBN: {:?}", ctr, record);
            },
        }
    }
    Ok(())
}

fn main()
{
    if let Err(err) = parse_csv()
    {
        println!("{}", err);
        process::exit(1);
    }
}
