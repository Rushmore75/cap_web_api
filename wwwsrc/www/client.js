
window.onload = function(){
    const tickets = document.getElementById("tickets");

    fetch("http://127.0.0.1:8000/api/owned_tickets")
        .then((response) => response.json())
        .then((json) => {
            for (let i = 0; i < json.length; i++) {
                const div = document.createElement("div")
                const header = document.createElement("h3"); 
                const body = document.createElement("p")
                header.innerText = json[i][0]
                body.innerText = json[i][1]
                div.appendChild(header)
                div.appendChild(body)
                tickets.appendChild(div)
            }
        });

}

