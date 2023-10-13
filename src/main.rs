#![allow(dead_code)] 
use bio::alphabets::dna;
use argparse::{ArgumentParser, StoreTrue, Store};

use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
    io,
    any::type_name,
};

use std::collections::HashMap;

type Record = HashMap<String, String>;

const SAMPLE_ID: &str = "Sample_ID";
const INDEX1_SEQUENCE: &str = "Index1_Sequence";
const INDEX2_SEQUENCE: &str = "Index2_Sequence";

fn converter(config: Config) -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::Reader::from_path(config.input_file)?;
    let mut wtr = csv::Writer::from_path(config.output_file)?;
    wtr.write_record(&["Sample_ID", "INDEX1_SEQUENCE", "INDEX2_SEQUENCE"]);

    for result in rdr.deserialize() {
        let record: Record = result?;
        //println!("{:?}", record);
        //println!("{:?}", record.keys());
        //println!("{:?}", record.values());
        //let samplename = record.get(SAMPLE_ID).unwrap();
        let samplename = record.get(SAMPLE_ID).unwrap();
        println!("{}", samplename);
        let index1_seq = record.get(INDEX1_SEQUENCE).unwrap();
        let index2_seq = record.get(INDEX2_SEQUENCE).unwrap();
        let dna_rc1 = dna::revcomp(index1_seq.as_bytes());
        let dna_rc1_string = String::from_utf8_lossy(&*dna_rc1).to_string();
        let dna_rc2 = dna::revcomp(index2_seq.as_bytes());
        let dna_rc2_string = String::from_utf8_lossy(&*dna_rc2).to_string();
        if config.forstrand {
            wtr.write_record(&[&samplename, &index2_seq, &dna_rc1_string]);
        }
        else {
            wtr.write_record(&[&samplename, &dna_rc2_string, &dna_rc1_string]);
        }
        //wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    }


    Ok(())
}



fn main() {

    let mut verbose = false;
    let mut input_file = String::new();
    let mut output_file = String::new();
    let mut forstrand = false;
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert the index sequence on an Illumina Samplesheet to a SG Samplesheet");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut forstrand)
            .add_option(&["-f", "--forwardstrand"], StoreTrue, "Input samplesheet is Forward Strand Orientation. Default (not including this flag) is for when the input sample is Reverse Complement Orientation");
        ap.refer(&mut input_file)
            .add_option(&["-i", "--input"], Store, "Input File").required();
        ap.refer(&mut output_file)
            .add_option(&["-o", "--output"], Store, "Output File").required();
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Verbose option is {}", verbose);
        println!("Forward Strand option is {}", forstrand);
        println!("Input file is {}", input_file);
        println!("Output file is {}", output_file);
    }

    let config = Config { forstrand, input_file, output_file };
    //println!("Config is {} {} {}", config.forstrand, config.input_file, config.output_file);
    converter(config);
}

pub struct Config {
    pub forstrand: bool,
    pub input_file: String,
    pub output_file: String,
}


fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
