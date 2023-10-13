use argparse::{ArgumentParser, StoreTrue, Store};
use sg_samplesheet_converter::Config;

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
    sg_samplesheet_converter::converter(config);
}

