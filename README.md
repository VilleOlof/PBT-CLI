# pbt-cli
A CLI for Party Bots Tournaments written in rust 🚀🚀.  
Takes in a tournament file and uploads it to [PB Tournaments](https://tournaments.partybots.net)  

## Usage
```bash
pbt-cli <tournament file>
```
It will then prompt you for extra tournament information such as title, date & media link.  

## Install
Download the pbt-cli binary from the Release page.  
Or build it yourself with `cargo build --release`  

And then add a `.release.env` file in the same directory as the binary  
with the following content:
```bash
DATABASE_URL=<mysql_database_url>
```
*Requires `libcrypto-3-x64.dll` & `libssl-3-x64.dll`*

## Development
Just run with `cargo run`  
And use a `.env` instead with the same content  
*Might need to set up diesel to initialize a new clean database*