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
