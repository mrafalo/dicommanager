# DICOM Anomizer

## Overview

Tool for mass anonymization of DICOM files. It works very fast, even on thousands of files.


Dependencies:

* csv 
* dicom 
* dicom-pixeldata 
* indicatif 
* rayon 
* walkdir 
* clap 
* xlsxwriter 


## Parameters

The system currently supports three options:

##### Export image file, extracted from DICOM file

Parameters:
* `source` - source DICOM filename
* `dest` - destination filename (png file)

Usage:

    dicommanager export -s in/files/dicom/001 -d out/123.png 

##### Perform mass anonimization for selected folder

Parameters:
* `source` - source folder with DICOM files (the application works recursively in all subfolders)
* `dest` - destination folder, where anomized DICOM files are saved

Usage:

    dicommanager anomize -s in/files -d /out/files

##### Extract and print DICOM file metadata (selected tags)

Parameters:

* `filename` 

Usage:

    dicommanager metadata -f in/files/dicom/001

## Performance results

TODO