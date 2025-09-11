Notes
-----

Input files
===========

A couple of things need to be kept in mind with regards to methylation data.
At this point, four different file types for methylation data are supported:
 - Allcools files
 - MethylDackel bedgraph files
 - Bismark coverage files
 - Bismark CpG report files

Note that both MethylDackel bedgraph files and Bismark CpG report files are assumed to be 0-based encoded.
The Allcools files and Bismark coverage files are assumed to be 1-based encoded. 
For the Bismark coverage files, this can be changed when generating the files, so pay attention that this is correct.

For the RNA-part, for now only featureCounts tables are supported.