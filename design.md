inside the goose folder there is Cloudwolf. it is a fork of goose ai agent. It is not general perpus like goose and spesifacly to create projects writen in rust and hosted with docker-compse. and does this with specialised tools

the llm api is mgs-proxy witch proxys to openai compatible endpoings (gemini right now) and requers users to be loged in and have a jwt toekn. 

there are several fucnionts defined that aid in createing rust apis and hosting them with dockercompose. there funcionts can be called derectly or used thought a cli tool in main.
* the login tool gets a jwt token from mgs-proxy
* the tools operat on the asumpoting that they are being run withing the users project derectery, aka `setup` dosent create the project dir, but it will create the backend bolder withing it

cloudwolf (should) call these tools directly the same way it calles tools to edit or read files. it should NOT requier the user to install a speret cli tool and explain to cloudwolf how to use it. 

cloudwolf should compile to a singal zero-dependency binary with all nesisary tools that the user can install




