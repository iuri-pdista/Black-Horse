# Black Horse

## What is?

Black Horse is a very simple malware, just like Ransomware, that selects a .txt file from the target, clones it, and encrypts the original with sha512 method

## How was made?

It was made using mainly Rust, which will breach into the file and virus it, and a Shell script, that select the target, and runs the rust file, with the target as parameter

## How I use it?

It's actualy pretty simple

### <b> First: </b>

```bash
# clone the repository
git clone https://github.com/iuri-pdista/Virus-Rust
```

#### <b> Or: </b>

You can just download its zip

### <b> Second: </b>

```bash
# enter the directory virus_test
cd ./virus_test
```

### <b> Third: </b>

#### Now you can:

Test the virus in this enviroment

##### Or:

You can test it in another directory with at least one .txt <b><u>(The script also works with more than one, but its support it is not the best)</u></b>

### <b>Four: </b>

#### If you choose the second choice, you just need to move the script.sh and virus_file to the directory, by using: 

```bash
mv script.sh virus_file <the name of the directory>
```

#### Else is just to follow the next step

## And <b> Five </b> TADA :tada: :tada: ENJOY, you've virused your target!


## Future features

### The malware is still in development and I intend to continuously updating it until reaches the expected point and features. Some features I want to futurely implement, are:

- Windows OS suport
- Suport to the more than one file mode
- Robustness
- Versability

## So stay tuned :sparkles:
<p align="center"> Made with ♥ by [Iuri Corrêa](https://github.com/iuri-pdista) </p>