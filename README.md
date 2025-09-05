## rename_files is a CLI utility that renames files in a directory by adding '-' and an alpha-numeric random 8-character string. 

## How to use rename_files utility
Let the directory with some files (file_1.txt file_2.txt) that need automatic renaming is "~/files". Pass the "~/files" argument to the program, for example: 
   ```console
      pwd 
      ~/
      ls 
      files rename_files
      cd files
      ls
      file_1.txt file_2.txt
      cd ..
      ./rename_files ~/files
      ls
      file_1-sqdexnmh.txt file_2-1hje1zvz.txt
   ```
As a result, files file_1.txt and file_2.txt were renamed to file_1-sqdexnmh.txt and file_2-1hje1zvz.txt. 

