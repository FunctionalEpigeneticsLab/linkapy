Notes
-----

Input files
===========

A couple of things need to be kept in mind with regards to methylation data.
At this point, five different file types for methylation data are supported:
 - Allcools files
 - MethylDackel bedgraph files
 - Bismark coverage files
 - Bismark CpG report files
 - BedMethyl files

Note that MethylDackel bedgraph files, Bismark CpG report files and BedMethyl files are assumed to be 0-based start encoded (and 1-based end, if applicable).
The Allcools files and Bismark coverage files are assumed to be 1-based encoded. 
For the Bismark coverage files, keep in mind that `bismark_methylation_extractor` has a flag to output 0-based files, so pay attention that this is correct.
For BedMethyl files, please note that _no_ checks are performed to ensure that only one modification type is present.
If you have multiple modification types (i.e. more then one 'name' in column 4), please split htem into separate files (you can include them in the output by specific multiple methylation_path/methylation_pattern combinations).

For the RNA-part, for now only featureCounts tables are supported.