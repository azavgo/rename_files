This CLI utility allows to rename individual files as well as all the files one level down in a folder by adding a '-' with 8-character alpha-numeric string. 
For example to re-name an individual file, pass the file name as an argument: 
````console
    % ls my_file*
    % my_file.txt
    % rename_files my_file.txt
    % ls my_file*
    % my_file-tdrduqd7.txt
````
Rename all files in a folder without passing an argument to rename_files: 
````console
    % cd folder_2/
    % ls 
    % file_1.txt file_2.txt folder_2
    % rename_files 
    % ls
    % file_1-3gymos1r.txt file_2-aym6qtl3.txt folder_2
````
Rename all files in a folder with passing the folder as an argument: 
````console
    % ls
    % folder_1
    % cd folder_1
    % ls
    % file_1.txt file_2.txt folder_2
    % cd ..
    % rename_files folder_1
    % cd folder_1
    % ls 
    % file_1-xkbflimy.txt file_2-nha3lg64.txt folder_2
````

