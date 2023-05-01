
window.onload = function(){
    const tickets = document.getElementById("tickets");

    fetch("http://127.0.0.1:8000/api/tickets")
        .then((response) => response.json())
        .then((ticket) => {
            for (let i = 0; i < ticket.length; i++) {
                const div = document.createElement("div")
                const header = document.createElement("h3")
                const body = document.createElement("p")
            
                div.id              = ticket[i][0]
                header.innerText    = ticket[i][1]
                body.innerText      = ticket[i][2]

                div.appendChild(header)
                div.appendChild(body)
                
                // <input type="button" value="Click me" onclick="msg()"> 
                const input = document.createElement("input")
                input.type = "button"
                input.value = "Complete"
                input.onclick = () => {
                    fetch("http://127.0.0.1:8000/api/complete_ticket/"+div.id, {
                            method: 'POST'
                        })
                        .then(_ => location.reload())
                }

                div.appendChild(input)
                // div.appendChild(form)
                tickets.appendChild(div)
            }
        })
}

