#!/bin/bash

count=0
filePath=$(pwd)
filePath="${filePath}/*.txt" 
for f in $filePath
do 
  echo "Processing $f file..."
  echo"$f"	
  ./rust_shell "$f" "$count"
  count=$((count + 1))
done  


#SHELL

