use bio::alphabets::dna;
use argparse::{ArgumentParser, StoreTrue, Store};

use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
    any::type_name,
};



fn converter(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
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

    converter(&input_file);

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
