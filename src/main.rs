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

const sample_ID: &str = "Sample_ID";
const Index1_Sequence: &str = "Index1_Sequence";
const Index2_Sequence: &str = "Index2_Sequence";

fn converter(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(input)?;
    let mut wtr = csv::Writer::from_path(output)?;
    wtr.write_record(&["Sample_ID", "Index1_Sequence", "Index2_Sequence"]);
    for result in rdr.deserialize() {
        let record: Record = result?;
        //println!("{:?}", record);
        //println!("{:?}", record.keys());
        //println!("{:?}", record.values());
        //let samplename = record.get(sample_ID).unwrap();
        let samplename = record.get(sample_ID).unwrap();
        //println!("{}", samplename);
        let index1_seq = record.get(Index1_Sequence).unwrap();
        let index2_seq = record.get(Index2_Sequence).unwrap();
        let dna_rc1 = dna::revcomp(index1_seq.as_bytes());
        let dna_rc1_string = String::from_utf8_lossy(&*dna_rc1).to_string();
        let dna_rc2 = dna::revcomp(index2_seq.as_bytes());
        let dna_rc2_string = String::from_utf8_lossy(&*dna_rc2).to_string();
        wtr.write_record(&[&samplename, &dna_rc2_string, &dna_rc1_string]);
        //wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    }


    Ok(())
}



fn main() {

    let mut verbose = false;
    let mut input_file = String::new();
    let mut output_file = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert an Illumina Samplesheet to a SG Samplesheet");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut input_file)
            .add_option(&["-i", "--input"], Store,
            "Input File").required();
        ap.refer(&mut output_file)
            .add_option(&["-o", "--output"], Store,
            "Output File").required();
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Input is {}", input_file);
        println!("Output is {}", output_file);
    }

    converter(&input_file, &output_file);

    /*let alphabet = alphabets::dna::alphabet();

    let dna_rc = dna::revcomp("GATTACA".as_bytes());
    let dna_rc_string = String::from_utf8_lossy(&*dna_rc).to_string();
    print_type_of(&dna_rc_string);
    println!("result: {}", dna_rc_string);

    let reversed = reverse(&dna_rc_string);
    print_type_of(&reversed);
    println!("result: {}", reversed);*/
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
