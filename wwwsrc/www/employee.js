
window.onload = function(){
    const tickets = document.getElementById("tickets");

    fetch("http://127.0.0.1:8000/api/tickets")
        .then((response) => response.json())
        .then((ticket) => {
            for (let i = 0; i < ticket.length; i++) {
                const div = document.createElement("div")
                const header = document.createElement("h3")
                const body = document.createElement("p")
            
                fetch("http://127.0.0.1:8000/api/message/"+ticket[i].title)
                    .then((response) => response.json())
                    .then((title) => {
                        header.innerText = title 
                    })                   
                    
                fetch("http://127.0.0.1:8000/api/message/"+ticket[i].description)
                    .then((response) => response.json())
                    .then((content) => {
                        body.innerText = content 
                    })                 
                div.appendChild(header)
                div.appendChild(body)
                div.appendChild(form)
                tickets.appendChild(div)
            }
        })
}

