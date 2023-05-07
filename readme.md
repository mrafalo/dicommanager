# DICOM Anomizer

## Overview

Tool for mass anonymization of DICOM files. The tool was written in Rust.
The main functionality is anonymization of multiple DICOM files from the selected folder (and all subfolders). Source DICOM files are not modified. Anonymized files are copied to the folder indicated in the parameter. The program works quickly, even for thousands of DICOM documents. An important functionality is to maintain the consistency of patient data. Anonymization assigns patients a unique identifier (index), so if there is a different DICOM file for the same patient among the source files, the tool will assign it the same index. As a result, it is possible to recreate the image database, while preserving the assignment of the image to the patient.
Extend the program freely or let me know if you need any function.

Rust dependencies (for specific versions, check `Cargo.toml'):

* `csv` 
* `dicom-pixeldata `
* `indicatif` 
* `rayon` 
* `walkdir` 
* `clap` 
* `xlsxwriter` 


## Run options and parametres

The system supports following options:

### Perform mass anonimization for selected folder

Option: `anomize`. Parameters:
* `source` - source folder with DICOM files (the application works recursively in all subfolders)
* `dest` - destination folder, where anomized DICOM files are saved


The following tags of DICOM document are anomized:
* PATIENT_ADDRESS (0010,1040) - LO 1 DICOM
* PATIENT_NAME (0010,0010) - PN 1 DICOM
* PATIENT_ID (0010,0020) - LO 1 DICOM
* STUDY_DESCRIPTION (0008,1030) - LO 1 DICOM


Anonymization takes place in the following steps:
1. All DICOM files in the folder specified in the `source` parameter are listed and saved in the *_metadata.csv* file in the folder specified in the `dest` parameter. Note: all folders and subfolders in the `source` directory are processed. There can be any types of files in the `source` directory, the program verifies each file with the DICOM format. Only DICOM files end up in the metadata file. The *_metadata.csv* file contains three columns: file sequence number, full path to DICOM file, and DICOM file size.
1. For each file registered in *_metadata.csv* the following steps are performed:
	1. Extract patient data (PATIENT_ID and PATIENT_NAME) from the specified DICOM file
	1. Verify that this patient has already been processed (based on PATIENT_ID tag)
	1. If it has not been processed: add a new unique patient index, add a DICOM image and give it number 1
	1. If it was processed: find its index, for the found index (*patient_index*), add a DICOM image with a incremended number (increment successive images for a given *patient_index*)
	1. Anonymise the DICOM document
	1. Copy the anonymized DICOM file to the folder specified in the `dest` parameter. Source DICOM is not modified.
	1. Add an entry to the excel file *_out.xlsx* containing the following attributes (columns): 
		1. source_file - full path to source DICOM file
		2. patient_name - patient name extracted from source DICOM file (PATIENT_NAME)
		3. patient_id_DICOM  - patient ID extracted from source DICOM file (PATIENT_ID)
		4. patient_index - unique patient number assigned by the system (patients with same PATIENT ID will have the same PATIENT_INDEX)
		5. file_size - size of source DICOM file
		6. destinaton_file - full path to target (anomized) DICOM file
		7. status - anonimization status: OK or ERROR


Usage:

    dicommanager anomize -s in/files -d /out/files

### Extract and print DICOM file metadata (selected tags)

Option: `metadata`. Parameters:

* `filename` - full path to source DICOM file

Metadata information is printed on screen.

Usage:

    dicommanager metadata -f in/files/dicom/001

### Split images

Option: `splitby`. Parameters:
* `source` - source folder, containing images
* `mask` - files to be procesed within source folder (extension, e.g.: .png)
* `delimiter` - single character by which to split the filename (e.g.: _). The first item before the separator will be retrieved as a grouping object.

Usage scenario: assume that we have many image files with and there are multiple image files for one patient. All images are in single folder and we want to move them into subfolders (each subfolder for each patient). 

For example: for patient number 17 we have files: `17_001.png`, `17_002.png`, `17_003.png`, for patient number 21: `21_001.png`, `21_002`.png, etc. The program results in folders with names corresponding to patient numbers (17, 21, etc.). In these folders we have images corresponding to patient numbers.


Usage:

    dicommanager splitby -s in/files/dicom/files/ -m .png -d _ 


### Export image file, extracted from DICOM file

Option: `export`. Parameters:
* `source` - source DICOM filename
* `dest` - destination filename (png file)

This option retrives single image from source DICOM file and saves it to destination file. Only PNG file type is supported.

Usage:

    dicommanager export -s in/files/dicom/001 -d out/123.png 


## Current known limitations

1. The program only exports data to PNG format
2. Only selected tags from the DICOM document are anonymized
3. Identification of repeat patients is based on the PATIENT_ID field
4. Exported metadata files: *_out.xlsx* and *metadata.csv* have fixed names and hardcoded structure

## License

Copyright (c) 2023 Mariusz Rafało

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.