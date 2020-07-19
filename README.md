# Black Horse

## What is?

Black Horse is a very simple malware, just like Ransomware, that selects a .txt file from the target, clones it, and encrypts the original with sha512 method

## How was made?

It was made using mainly Rust, which will breach into the file and virus it, and a Shell script, that select the target, and runs the rust file, with the target as parameter, and finally a server-client structure created with python websockets, to send files to a determined dir or pc

## How I use it?

It's actualy pretty simple

### <b> First: </b>

```bash
# clone the repo
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

To send the copied files to another machine or directory you need to move the server.py file to the desired dir, establish the server, by running:

```bash
python3 server.py
# Or just: 
python server.py
# both work, depending on you or your pc
```

### <b>Fourth: </b>

#### Now you can:

Test the virus in this enviroment

##### Or:

You can test it in another directory with at least one .txt 

### <b>Fifth: </b>

#### If you've chosen the second choice, you just need to move the the files to the directory, by using: 

```bash
mv script.sh virus_file client.py <the name of the directory>
```

#### Else is just to follow the next step

## And <b> Sixth </b> TADA :tada: :tada: ENJOY :tada: :tada:, you've virused your target!


## Future features: 

### The malware is still in development and I intend to continuously updating it until reaches the expected point and features. Some features I want to in the future implement, are:

- Windows OS suport 
- Robustness
- Versability

## So stay tuned :sparkles:
<p align="center"> Made with ♥ by <a href="https://github.com/iuri-pdista"> Iuri Corrêa </a></p>