# scorpion2
A better client-server klogger, purpose of learning rust

### Instalation

`git clone https://github.com/drakegawain/scorpion2 ~/.scorpion2`

#### Use

On client side

`cargo run -- client -u <URL to send keys> -p <PORT> -i <ID of the machine> -s`

On server side

`cargo run -- server -u <URL to receive keys> -p <PORT>  -d <DATABASE url>`

#### Database

Set a schema like

`CREATE DATABASE text(
id VARCHAR(30),
date DATE,
text varchar(1000));`

to ensure db can catch the keys


### Install to path

Add this lines to bottom of .bashrc

`export SCORPION2_INSTALL = "$HOME/.scorpion2/target/release"
export PATH = $SCORPION2_INSTALL:$PATH`

### Build

Run 

`cd ~/.scorpion2
cargo build --release`

And thats it!

### Commands

##### Client

`scorpion2 client -u "127.0.0.1" -p 8000 -i "ths"`

##### Server

`scorpion2 server -u "127.0.0.1" -p 8000 -d "mysql://user:user@127.0.0.1/scorpion2"`

