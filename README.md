# This is a cli tool to create APIs that interact with postgress SQL databases. 

I think that much of the manual work of setting this up can be automated, and this project will test my hypothosis. 


This project is structured such that it is made each subcomand increase the functionality of your api by programticly generating code based on the databas scheme you provide. 


a tipical flow might be
* setup
* sql_crate
* post tabelName
* select tabelName

whh not a tui?
this is meant to be runrible from other programs and the output is meant to be piped into other programs 

why docker?
the generated program should be cloud agnostic and able to run on a single VPS 

road map 
* minio for media storage

