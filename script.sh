#!/bin/bash

VAR=$(pwd)
VAR="${VAR}/*.txt" 
echo ""
for f in $VAR
do
  echo "Processing $f file..."
  # count number of lines and output that for file $f
  echo"$f"	
  ./rust_shell "$f"
done  


#SHELL

