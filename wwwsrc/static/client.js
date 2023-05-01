
window.onload = function() {
    const value = readAndDelete("submit_success");
    const tickets = document.getElementById("tickets");
   
    if (value != "") {
        alert("Success!\nTicket ID: " + value);
    }

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

function readAndDelete(cname) {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');

    document.cookie = `${cname}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/; SameSite=Lax`;
    for(let i = 0; i <ca.length; i++) {
      let c = ca[i];
      while (c.charAt(0) == ' ') {
        c = c.substring(1);
      }
      if (c.indexOf(name) == 0) {
        return c.substring(name.length, c.length);
      }
    }
    return "";
  }
