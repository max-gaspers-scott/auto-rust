
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
            

            fetch("http://localhost:3002/api/get_one_usersdisplay_name").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_userscreated_at").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/add_conversations", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    [ add object of key values based on the struct ]
                })
            }).then(response => response.json()).then(data => console.log(data)); 
            

            fetch("http://localhost:3002/api/get_conversations").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversationsconversation_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversationsname").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversationsis_group_chat").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversationscreated_at").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/add_conversation_participants", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    [ add object of key values based on the struct ]
                })
            }).then(response => response.json()).then(data => console.log(data)); 
            

            fetch("http://localhost:3002/api/get_conversation_participants").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversation_participantsparticipant_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversation_participantsconversation_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversation_participantsuser_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_conversation_participantsjoined_at").then(response => response.json()).then(data => console.log(data));
            

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
            

            fetch("http://localhost:3002/api/get_one_messagesconversation_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagessender_id").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagescontent").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagessent_at").then(response => response.json()).then(data => console.log(data));
            

            fetch("http://localhost:3002/api/get_one_messagesedited_at").then(response => response.json()).then(data => console.log(data));
            
