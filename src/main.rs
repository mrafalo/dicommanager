use dicom::core::{DataElement};
use dicom::object::{open_file, Result, Error, OpenFileOptions};
use dicom_pixeldata::PixelDecoder;

use dicom_pixeldata::ndarray::NewAxis;
use walkdir::WalkDir;
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};
use std::path::Path;
use std::error::Error as StdError;
use std::collections::HashMap;
use xlsxwriter::*;
use std::time;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
struct Patient {
    patient_name: String, 
    source_file_name: String,
    patient_dicom_id: String,
    patient_internal_id: u16,
    image_num: u16,
    dest_file_name: String,
}

fn if_dcm_file(_file: &str) ->  bool {

    match open_file(_file) {
        Ok(_) =>  return true,
        Err(_) => return false
      };

}

fn get_patient_data(_file: &str) ->  (String, String) {

    let res = open_file(_file);

    let mut res_ok = true;

    match res  {
        Ok(_) =>  res_ok = true,
        Err(_) =>  res_ok = false 
      };

    
      if res_ok {

        let obj1 = &res.unwrap().clone();
        let obj2 = obj1.clone();
        
        return (obj1.element_by_name("PatientID").unwrap().to_str().unwrap().to_string(),
                obj2.element_by_name("PatientName").unwrap().to_str().unwrap().to_string());

      } else {

        return (String::from("no DICOM"), String::from("no DICOM"))
      
    }

}

fn anomize_dcm(_source_file_name: &str, _index: u16, _idx:u16, _dest_file_name: &str) -> Result<String, Error> {

   
    let mut obj = open_file(_source_file_name)?;

    let new_patient_address = DataElement::new(
        dicom::dictionary_std::tags::PATIENT_ADDRESS,
        dicom::core::VR::PN,
        dicom::dicom_value!(Str, String::from("unknown")),
    );

    let new_patient_name = DataElement::new(
        //Tag(0x0010, 0x0010),
        dicom::dictionary_std::tags::PATIENT_NAME,
        dicom::core::VR::PN,
        dicom::dicom_value!(Str, _index.to_string()),
    );

    let new_patient_id = DataElement::new(
        //Tag(0x0010, 0x0010),
        dicom::dictionary_std::tags::PATIENT_ID,
        dicom::core::VR::PN,
        dicom::dicom_value!(Str, _index.to_string()),
    );

    let new_study_description = DataElement::new(
        dicom::dictionary_std::tags::STUDY_DESCRIPTION,
        dicom::core::VR::PN,
        dicom::dicom_value!(Str, String::from("unknown")),
    );

    obj.put(new_patient_address);
    obj.put(new_patient_name);
    obj.put(new_study_description);
    obj.put(new_patient_id);


    obj.write_to_file(_dest_file_name)?;

    

    Ok(String::from("OK"))
}



fn create_metadata_csv(_source:&str, _dest: &str) -> (String, Vec<String>) {

    let in_folder = _source;
    let out_csv_file_metadata = format!("{}{}", _dest, String::from("/_metadata.csv"));

    let mut csv_writer_metadata = Writer::from_path(&out_csv_file_metadata).unwrap();

    let mut file_count = 0;
    let mut files = Vec::new();
    let total_start = time::Instant::now();


    csv_writer_metadata.write_record(&["lp", "path", "size"]).ok();

    for e in WalkDir::new(in_folder).into_iter().filter_map(|e| e.ok()) {
        //let file_name = e.file_name().to_string_lossy();
        let file_path = String::from(e.path().to_str().unwrap());
        let file_path_clone = file_path.clone();
        
        let start = time::Instant::now();

        if if_dcm_file(&file_path)
        {
            let file_size = std::fs::metadata(&file_path).unwrap().len();

            file_count = file_count + 1;
            files.push(file_path);

            // if (file_count % 100) == 0 {
                
            //     let elapsed = start.elapsed();

            //     println!("{:?}: processing {} files...", elapsed , file_count.to_string());
               
            // }
            csv_writer_metadata.write_record(&[file_count.to_string(), file_path_clone, file_size.to_string()]).ok();
        }
    };

    csv_writer_metadata.flush().ok();

    println!("All OK, total time: {:?} seconds. Total files: {}. Metadata saved in {}", total_start.elapsed().as_secs_f32(), file_count, &out_csv_file_metadata);

    return (out_csv_file_metadata, files)

}


fn xls_wirte_line(_row: u32, _line: Vec<&str>, _sheet: &mut xlsxwriter::Worksheet){

    let mut i = 0;

    for l in _line {
        &_sheet.write_string(_row, i, l,None);
        i = i + 1;


    }

}

fn create_metadata_xlsx(_source:&str, _dest: &str) -> Result<String, Error>{

    let in_folder = _source;
    let out_xlsx_file_metadata = format!("{}{}", _dest, String::from("/metadata.xlsx"));

    let xls_writer_metadata = Workbook::new(&out_xlsx_file_metadata).unwrap();
    let mut sheet1 = xls_writer_metadata.add_worksheet(None).unwrap();

    let mut file_count = 0;
    let mut files = Vec::new();
    let total_start = time::Instant::now();

    sheet1.write_string(0, 0, "lp",None);
    sheet1.write_string(0, 1, "path",None);
    sheet1.write_string(0, 2, "size",None);


    for e in WalkDir::new(in_folder).into_iter().filter_map(|e| e.ok()) {
        //let file_name = e.file_name().to_string_lossy();
        let file_path = String::from(e.path().to_str().unwrap());
        let file_path_clone = file_path.clone();
        
        let start = time::Instant::now();

        if if_dcm_file(&file_path)
        {
            let file_size = std::fs::metadata(&file_path).unwrap().len();

            file_count = file_count + 1;
            files.push(file_path);

            // if (file_count % 100) == 0 {
                
            //     let elapsed = start.elapsed();

            //     println!("{:?}: processing {} files...", elapsed , file_count.to_string());
               
            // }

            xls_wirte_line(file_count, vec![&file_count.to_string(), &file_path_clone, &file_size.to_string()], &mut sheet1);
           
        }


    };

    xls_writer_metadata.close().ok();

    println!("metadata_xls --> total time: {:?} seconds. Total files: {}. Metadata saved in {}", total_start.elapsed().as_secs_f32(), file_count, &out_xlsx_file_metadata);

    Ok(out_xlsx_file_metadata)

}


fn patient_exists(_patients: &Vec<Patient>, _patient_id: &str) -> bool{

    for p in _patients{

        if p.patient_dicom_id == _patient_id{
            return true
        }
    }
    return false;

}

fn get_patient_internal_id(_patients: &Vec<Patient>, _patient_id: &str) -> u16 {

    for p in _patients{

        if p.patient_dicom_id == _patient_id{
            let ret = p.patient_internal_id.clone();

            return ret;
        }
    }
    return 0;

}

fn get_last_image_id(_patients: &Vec<Patient>, _patient_internal_id: u16) -> u16 {

    let mut ret = 0;

    for p in _patients{

        if p.patient_internal_id == _patient_internal_id{
            let tmp_ret = p.image_num.clone();

            if tmp_ret > ret{
                ret = tmp_ret;
            }

        }
    }
    return ret;

}

fn anomizer_par(_source:&str, _dest: &str)  {

    let total_start = time::Instant::now();
    let files = Arc::new(Mutex::new(vec![]));

    let patient_internal_index = Arc::new(Mutex::new([5000]));
    let patient_internal_index = patient_internal_index.clone();

    let mut paths = Vec::new();

    for e in WalkDir::new(_source).into_iter().filter_map(|e| e.ok()) {
       
        let file_path = String::from(e.path().to_str().unwrap());

        let file_path_clone = file_path.clone();

        paths.push(file_path_clone);
        
    }


    paths.par_iter()
        .enumerate()
        .for_each(|(_,p)| {

            let file_path = p.clone();
        

            let files_clone = Arc::clone(&files);
            let mut v = files_clone.lock().unwrap();

            let (patient_id, patient_name) = get_patient_data(&file_path);
            let patient_id_clone = patient_id.clone();

    
            if patient_id != "no DICOM"
            {
                let mut idx = patient_internal_index.lock().unwrap();
                
                let patient_internal_id = get_patient_internal_id(&v, &patient_id_clone) ;

                if patient_internal_id == 0
                {
                    let out_path = format!("{}/out_{}_{}.dcm", _dest, idx[0], 1);
                   
                    let patient = Patient{
                        patient_name: patient_name,
                        source_file_name: file_path, 
                        patient_dicom_id: patient_id, 
                        patient_internal_id:idx[0], 
                        image_num: 1,
                        dest_file_name:out_path };

                   
                    v.push(patient);
                    idx[0] = idx[0] + 1;

                } else {

                    let last_image_id = get_last_image_id(&v, patient_internal_id) ;
                    let out_path = format!("{}/out_{}_{}.dcm", _dest, patient_internal_id, last_image_id+1);

                    let patient = Patient{
                        patient_name: patient_name,
                        source_file_name: file_path, 
                        patient_dicom_id: patient_id, 
                        patient_internal_id:patient_internal_id, 
                        image_num: last_image_id + 1,
                        dest_file_name: out_path };

                    v.push(patient);
                }
            }

    });

    let metadata = &*files.lock().unwrap();


    println!("Files read OK, total time: {:?} seconds", total_start.elapsed().as_secs_f32());


    // metadata.par_iter()
    //     .enumerate()
    //     .for_each(|(_,p)| {
           
    //         anomize_dcm(&p.source_file_name, p.patient_internal_id, p.image_num, &p.dest_file_name).unwrap();
        
    //     });


   
    // let out_xlsx_file = format!("{}{}", _dest, String::from("/_out.xlsx"));
    
    // let xls_writer_metadata = Workbook::new(&out_xlsx_file).unwrap();
    // let mut sheet1 = xls_writer_metadata.add_worksheet(None).unwrap();
    // xls_wirte_line(0, vec!["source_file", "patient_name", "patient_id_DICOM",  "patient_index", "image_num", "destinaton_file", "status"], &mut sheet1);

    // let mut i = 1;

    // for m in metadata{
    //     xls_wirte_line(i, vec![&m.source_file_name,  &m.patient_name.to_string(), &m.patient_dicom_id,  &m.patient_internal_id.to_string(), &m.image_num.to_string(), &m.dest_file_name, "OK"], &mut sheet1);
    //     i = i + 1;

    // }

    // xls_writer_metadata.close().ok();
    // println!("parallel run ---> total time: {:?} seconds.", total_start.elapsed().as_secs_f32());

}



fn anomizer_par1(_source:&str, _dest: &str)  {

    let total_start = time::Instant::now();
    let mut v = Vec::new();

    let mut paths = Vec::new();

    for e in WalkDir::new(_source).into_iter().filter_map(|e| e.ok()) {
       
        let file_path = String::from(e.path().to_str().unwrap());

        let file_path_clone = file_path.clone();

        if if_dcm_file(&file_path){
            paths.push(file_path_clone);
        }
    }


    for p in paths {

            let file_path = p.clone();

            let (patient_id, patient_name) = get_patient_data(&file_path);
            //let (patient_id, patient_name) = (String::from("111"), String::from("111"));

            if if_dcm_file(&p)
            //if patient_id != "no DICOM"
            {

                let patient = Patient{
                    patient_name: patient_name,
                    source_file_name: file_path, 
                    patient_dicom_id: patient_id, 
                    patient_internal_id:0, 
                    image_num: 1,
                    dest_file_name:String::from("none") };


                v.push(patient);

            }

    };

 
    // for m in metadata{
    //     println!("{:?}", m);

    // }


   println!("anomizer_par1 total time: {:?} seconds", total_start.elapsed().as_secs_f32());


    // metadata.par_iter()
    //     .enumerate()
    //     .for_each(|(_,p)| {
           
    //         anomize_dcm(&p.source_file_name, p.patient_internal_id, p.image_num, &p.dest_file_name).unwrap();
        
    //     });


   
    // let out_xlsx_file = format!("{}{}", _dest, String::from("/_out.xlsx"));
    
    // let xls_writer_metadata = Workbook::new(&out_xlsx_file).unwrap();
    // let mut sheet1 = xls_writer_metadata.add_worksheet(None).unwrap();
    // xls_wirte_line(0, vec!["source_file", "patient_name", "patient_id_DICOM",  "patient_index", "image_num", "destinaton_file", "status"], &mut sheet1);

    // let mut i = 1;

    // for m in metadata{
    //     xls_wirte_line(i, vec![&m.source_file_name,  &m.patient_name.to_string(), &m.patient_dicom_id,  &m.patient_internal_id.to_string(), &m.image_num.to_string(), &m.dest_file_name, "OK"], &mut sheet1);
    //     i = i + 1;

    // }

    // xls_writer_metadata.close().ok();
    // println!("parallel run ---> total time: {:?} seconds.", total_start.elapsed().as_secs_f32());

}



fn anomizer_par2(_source:&str, _dest: &str)  {

    let total_start = time::Instant::now();
    let mut v = Vec::new();

   // let mut paths = Vec::new();

    for e in WalkDir::new(_source).into_iter().filter_map(|e| e.ok()) {
       
        let file_path = String::from(e.path().to_str().unwrap());

        let file_path_clone = file_path.clone();

        if if_dcm_file(&file_path){
            paths.push(file_path_clone);
        }
    }


    paths.par_iter()
        .enumerate()
        .for_each(|(_,p)| {

            let file_path = p.clone();

            let (patient_id, patient_name) = get_patient_data(&file_path);
            //let (patient_id, patient_name) = (String::from("111"), String::from("111"));

            if if_dcm_file(&p)
            //if patient_id != "no DICOM"
            {

                let patient = Patient{
                    patient_name: patient_name,
                    source_file_name: file_path, 
                    patient_dicom_id: patient_id, 
                    patient_internal_id:0, 
                    image_num: 1,
                    dest_file_name:String::from("none") };


                //v.push(patient);

            }

    });

 
    // for m in metadata{
    //     println!("{:?}", m);

    // }


   println!("anomizer_par2 total time: {:?} seconds", total_start.elapsed().as_secs_f32());


    // metadata.par_iter()
    //     .enumerate()
    //     .for_each(|(_,p)| {
           
    //         anomize_dcm(&p.source_file_name, p.patient_internal_id, p.image_num, &p.dest_file_name).unwrap();
        
    //     });


   
    // let out_xlsx_file = format!("{}{}", _dest, String::from("/_out.xlsx"));
    
    // let xls_writer_metadata = Workbook::new(&out_xlsx_file).unwrap();
    // let mut sheet1 = xls_writer_metadata.add_worksheet(None).unwrap();
    // xls_wirte_line(0, vec!["source_file", "patient_name", "patient_id_DICOM",  "patient_index", "image_num", "destinaton_file", "status"], &mut sheet1);

    // let mut i = 1;

    // for m in metadata{
    //     xls_wirte_line(i, vec![&m.source_file_name,  &m.patient_name.to_string(), &m.patient_dicom_id,  &m.patient_internal_id.to_string(), &m.image_num.to_string(), &m.dest_file_name, "OK"], &mut sheet1);
    //     i = i + 1;

    // }

    // xls_writer_metadata.close().ok();
    // println!("parallel run ---> total time: {:?} seconds.", total_start.elapsed().as_secs_f32());

}



fn anomizer(_source:&str, _dest: &str) -> Result<(), Error>{
    let total_start = time::Instant::now();

    let out_folder = _dest;
    let out_xlsx_file = format!("{}{}", _dest, String::from("/_out.xlsx"));
    
    let xls_writer_metadata = Workbook::new(&out_xlsx_file).unwrap();
    let mut sheet1 = xls_writer_metadata.add_worksheet(None).unwrap();
    
    let mut index = 1000;
    
    let (metadata_file, files) = create_metadata_csv(_source, _dest);

    let mut patients_ids: HashMap<String, u16> = HashMap::new();
    let mut patients_cnt: HashMap<String, u16> = HashMap::new();
    let mut patients_names: HashMap<String, u16> = HashMap::new();
  
    //let mut idx = 0;
    let mut curr_idx = 0;
   
    let mut metadata_reader = csv::Reader::from_path(&metadata_file).unwrap();
    let file_count:u64 =metadata_reader.records().count().try_into().unwrap();


    let bar_style = ProgressStyle::with_template("[{prefix} {spinner}] {bar:60.green/blue} {pos:>7}/{len:7} {msg}").unwrap().progress_chars("##.");

    let bar = ProgressBar::new(file_count).with_style(bar_style);

    let mut metadata_reader = csv::Reader::from_path(&metadata_file).unwrap();

    xls_wirte_line(0, vec!["source_file", "patient_name", "patient_id_DICOM",  "patient_index", "file_size", "destinaton_file", "status"], &mut sheet1);


    for f in metadata_reader.records() {

        let row = f.unwrap().clone();


        let file_path = &row[1];
        let file_size = &row[2];

        curr_idx = curr_idx + 1;
        let obj = OpenFileOptions::new()
            .read_until( dicom::dictionary_std::tags::PIXEL_DATA)
            .open_file(&file_path)?;

        let patient_name = obj.element_by_name("PatientName")?.to_str().unwrap().to_string();
        let patient_id = obj.element_by_name("PatientID")?.to_str().unwrap().to_string();

        let mut patient_index = 0;
        let patient_cnt:u16;

        if !patient_id.trim().is_empty() {
        
            if patients_ids.get(&patient_id) == None {

                patients_names.insert(patient_name.clone(), index);
                patients_ids.insert(patient_id.clone(), index);
                patients_cnt.insert(patient_id.clone(), 1);
                patient_index = index;
                patient_cnt = 1;
                index = index + 1;


            } else {

                let cnt = patients_cnt.get(&patient_id).unwrap();
                patient_index =  *patients_ids.get(&patient_id).unwrap();
                patient_cnt = *cnt + 1;
                patients_cnt.insert(patient_id.clone(), cnt + 1);
            }


            let dest_file_name = format!("{}/out_{}_{}.dcm", out_folder, patient_index, patient_cnt);

            anomize_dcm(file_path, patient_index, patient_cnt, &dest_file_name).unwrap();

           // csv_writer.write_record(&[file_path, &patient_name, &patient_id,  &patient_index.to_string(), &file_size.to_string(), &out_file]).ok();
            xls_wirte_line(curr_idx, vec![file_path, &patient_name, &patient_id,  &patient_index.to_string(), &file_size.to_string(), &dest_file_name, "OK"], &mut sheet1);

        } else {

            //csv_writer.write_record(&[file_path, &patient_name, &patient_id,  &patient_index.to_string(), &file_size.to_string(), "ERROR"]).ok();
            xls_wirte_line(curr_idx, vec![file_path, &patient_name, &patient_id,  &patient_index.to_string(), &file_size.to_string(), "NONE", "ERROR"], &mut sheet1);

        }


        //idx = idx + 1;

        bar.set_message(format!("{}{}",String::from("processing patient: "), patient_name.clone()));

        bar.inc(1);
        
    }

    bar.set_message("Finished!");
    bar.finish();

    xls_writer_metadata.close().ok();

    println!("normal run ---> total time: {:?} seconds. Total files: {}.  Data saved in {}", total_start.elapsed().as_secs_f32(), file_count, &out_xlsx_file);

 
    Ok(())

}


fn get_file_metadata(_file_name: &str) -> Result<(), Error>{

    let obj = OpenFileOptions::new()
        .read_until( dicom::dictionary_std::tags::PIXEL_DATA)
        .open_file(_file_name)?;

    
    println!("PatientName: {}", obj.element_by_name("PatientName")?.to_str().unwrap().to_string());
    println!("PatientSex: {}", obj.element_by_name("PatientSex")?.to_str().unwrap().to_string());

    
    


    println!("PatientAge: {:?}", obj.element_by_name("PatientAge").ok());

    //let timezone: String = cookie.map(|c| c.value().to_string()).unwrap_or_default();

    //println!("PatientAge: {:?}", obj.element_by_name("PatientAge").ok().map(|c| c.value().to_str()).unwrap_or_default());
    println!("PatientBirthDate: {:?}", obj.element_by_name("PatientBirthDate").ok().unwrap().to_str());
    println!("PatientBirthDate: {}", obj.element_by_name("PatientBirthDate")?.to_str().unwrap().to_string());
    println!("StudyDescription: {}", obj.element_by_name("StudyDescription")?.to_str().unwrap().to_string());
    println!("NUmber of frames: {}", obj.element(dicom::dictionary_std::tags::NUMBER_OF_FRAMES)?.to_str().unwrap().to_string());


    Ok(())

}

fn export_image(_source: &str, _dest: &str) -> Result<(), Box<dyn StdError>> {


    let obj = open_file(_source)?;
    let image = obj.decode_pixel_data()?;
    
    println!("number of frames: {:?}", image.number_of_frames());

    let dynamic_image = image.to_dynamic_image(0)?;


    dynamic_image.save(_dest)?;


/*
    let mut avi = AVI::new(_dest).unwrap();

    let mut new_meta: Vec<Frame> = Vec::new();
    
    for frame in &mut avi.frames.meta {
        if frame.is_pframe() || frame.is_audioframe() {
            for _ in 0..3 {
                new_meta.push(*frame);
            }
        }
        else {
            new_meta.push(*frame);
        }
    }
    avi.frames.meta = new_meta;
    avi.output(_dest).unwrap();

*/

    println!("Done.. file saved to :{}", _dest);

    Ok(())

}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Export image to png file
    Export {
        /// Source DICOM filename
        #[arg(short, long, default_value_t = String::from(""))]
        source: String,

        /// Desination filename
        #[arg(short, long, default_value_t = String::from(""))]
        dest: String
    },

    /// Show DICOM metadata
    Metadata {
        /// Source DICOM filename
        #[arg(short, long, default_value_t = String::from(""))]
        filename: String,

    },

    /// Anomize DICOM files
    Anomize {
        /// Source folder with dicom files
        #[arg(short, long, default_value_t = String::from(""))]
        source: String,

        /// Destination folder, where dicom files are saved
        #[arg(short, long, default_value_t = String::from(""))]
        dest: String,
    },
}

#[derive(Parser)]
struct Arguments {
    #[command(subcommand)]
    command: SubCommand,
}




fn main() ->  Result<(), Error> {

    let args = Arguments::parse();


    match args.command {
        SubCommand::Export {source, dest} =>{
            assert_eq!(Path::new(&source).exists(), true, "File not found! {}", &source);
            export_image(&source, &dest).unwrap();

        },

        SubCommand::Metadata {filename} => {

            assert_eq!(Path::new(&filename).exists(), true, "Path not found! {}", filename);
            get_file_metadata(&filename)?;

        }

        SubCommand::Anomize {source, dest} => {
            assert_eq!(Path::new(&source).exists(), true, "Path not found! {}", &source);
            assert_eq!(Path::new(&dest).exists(), true, "Path not found! {}", &dest);
            create_metadata_xlsx(&source, &dest);
            anomizer_par1(&source, &dest);
            anomizer_par2(&source, &dest);
        }

    }

    Ok(())

}
