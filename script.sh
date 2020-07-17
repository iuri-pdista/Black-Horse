#!/bin/bash

count=0
VAR=$(pwd)
VAR="${VAR}/*.txt" 
for f in $VAR
do 
  echo "Processing $f file..."
  # count number of lines and output that for file $f
  echo"$f"	
  ./rust_shell "$f" "$count"
  count=$((count + 1))
done  


#SHELL

