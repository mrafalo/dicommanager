# DICOM Manager

## Overview

Application for mass anonymization of DICOM files. It works very fast, even on thousands of files.


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

##### Export image file, axtracted from DICOM file

Parameters:
* `source`
* `dest`

Usage:

    dicommanager export -s -d

##### Extract and print DICOM file metadata (selected tags)

Parameters:
* `source` 
* `dest` 

Usage:

    dicommanager anomize -s -d

##### Extract and print DICOM file metadata (selected tags)

Parameters:

* `filename` 

Usage:

    dicommanager metadata -s -d

## Performance results

TODO