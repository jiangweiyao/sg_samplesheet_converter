use bio::alphabets::dna;

use std::collections::HashMap;
use std::error::Error;

type Record = HashMap<String, String>;

const IL_SAMPLE_ID: &str = "Sample_ID";
const IL_INDEX1_SEQUENCE: &str = "index";
const IL_INDEX2_SEQUENCE: &str = "index2";

const SG_SAMPLE_ID: &str = "Sample_ID";
const SG_INDEX1_SEQUENCE: &str = "Index1_Sequence";
const SG_INDEX2_SEQUENCE: &str = "Index2_Sequence";

pub fn converter(config: Config) -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::Reader::from_path(config.input_file)?;
    let mut wtr = csv::Writer::from_path(config.output_file)?;
    wtr.write_record(&[SG_SAMPLE_ID, SG_INDEX1_SEQUENCE, SG_INDEX2_SEQUENCE])?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        //println!("{:?}", record);
        //println!("{:?}", record.keys());
        //println!("{:?}", record.values());
        //let samplename = record.get(SAMPLE_ID).unwrap();
        //let samplename = record.get(SAMPLE_ID).unwrap();

        let mut samplename = String::new();
        let mut dna_rc1_string = String::new();
        let mut dna_rc2_string = String::new();
        let mut index2_seq = String::new();

        if record.get(IL_SAMPLE_ID).is_none() {
            samplename = String::from("");
            eprintln!("No {} column found", IL_SAMPLE_ID);
            break;
        }
        else {
            samplename = record.get(IL_SAMPLE_ID).unwrap().to_string();
        }
        //println!("{}", samplename);

        if record.get(IL_INDEX1_SEQUENCE).is_none() {
            dna_rc1_string = String::from("");
            //println!("{}", dna_rc1_string);
        }
        else {
            let index1_seq = record.get(IL_INDEX1_SEQUENCE).unwrap();
            let dna_rc1 = dna::revcomp(index1_seq.as_bytes());
            dna_rc1_string = String::from_utf8_lossy(&*dna_rc1).to_string();
            //println!("{}",  dna_rc1_string);
        }


        if record.get(IL_INDEX2_SEQUENCE).is_none() {
            dna_rc2_string = String::from("");
            //println!("{}", dna_rc2_string);
        }
        else {
            index2_seq = record.get(IL_INDEX2_SEQUENCE).unwrap().to_string();
            let dna_rc2 = dna::revcomp(index2_seq.as_bytes());
            dna_rc2_string = String::from_utf8_lossy(&*dna_rc2).to_string();
            //println!("{}",  dna_rc2_string);
        }
        /*let index1_seq = record.get(INDEX1_SEQUENCE).unwrap();
        /let index2_seq = record.get(INDEX2_SEQUENCE).unwrap();
        let dna_rc1 = dna::revcomp(index1_seq.as_bytes());
        let dna_rc1_string = String::from_utf8_lossy(&*dna_rc1).to_string();
        let dna_rc2 = dna::revcomp(index2_seq.as_bytes());
        let dna_rc2_string = String::from_utf8_lossy(&*dna_rc2).to_string();
        */
        if config.forstrand {
            wtr.write_record(&[&samplename, &index2_seq, &dna_rc1_string])?;
        }
        else {
            wtr.write_record(&[&samplename, &dna_rc2_string, &dna_rc1_string])?;
        }
    }

    wtr.flush()?;
    Ok(())
}

pub struct Config {
    pub forstrand: bool,
    pub input_file: String,
    pub output_file: String,
}



