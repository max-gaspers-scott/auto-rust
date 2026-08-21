# This is a CLI tool to create APIs that interact with PostgreSQL databases. 

I think that much of the manual work of setting this up can be automated, and this project will test my hypothesis. 


This project is structured such that it is made for each subcommand to increase the functionality of your api by programmatically generating code based on the database schema you provide. 


A typical flow might be
* setup
* sql_crate
* post table_name
* select tableName

whh not a tui?
This is meant to be runnable from other programs, and the output is meant to be piped into other programs. This means that "text in text out" is better than lines and half blacks. 

Why Docker?
The generated program should be cloud-agnostic and able to run on a single VPS 

road map 
* minio for media storage
* integration with goose fork through mgs-proxy
