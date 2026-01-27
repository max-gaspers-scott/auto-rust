
            fetch("http://localhost:3002/api/add_users", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    [ add object of key values based on the struct ]
                })
            }).then(response => response.json()).then(data => console.log(data)); 
            

            fetch("http://localhost:3002/api/get_users").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_usersuser_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_usersusername").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_usersemail").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/add_messages", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    [ add object of key values based on the struct ]
                })
            }).then(response => response.json()).then(data => console.log(data)); 
            

            fetch("http://localhost:3002/api/get_messages").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagesmessage_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagessender_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagesrecipiant_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagescontent").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagessent_at").then(response => response.json()).then(data => console.log(data));
            
