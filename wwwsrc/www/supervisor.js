
window.onload = function(){
    const tickets = document.getElementById("tickets");

    fetch("http://127.0.0.1:8000/api/get_employees")
        .then((response) => response.json())
        .then((employees) => {
            fetch("http://127.0.0.1:8000/api/unassigned")
                .then((response) => response.json())
                .then((ticket) => {
                    for (let i = 0; i < ticket.length; i++) {
                        const div = document.createElement("div")
                        const header = document.createElement("h3")
                        const body = document.createElement("p")
                    
                        const form = document.createElement("form")
                        form.id = ticket[i].id

                        employees.forEach(emp => {
                            const input = document.createElement("input")
                            const label = document.createElement("label")
                            input.type = "checkbox"
                            input.id = emp
                            input.name = "assignees"
                            label.for = emp
                            label.innerHTML = emp

                            form.appendChild(label)
                            form.appendChild(input)
                        });
                        const submit = document.createElement("input")
                        submit.type = "submit"
                        submit.value = "Assign"
                        submit.onclick = () => {
                            var sending = {
                                ticket: ticket[i].id,
                                assigned_to: []
                            };
                            const f = document.getElementById(ticket[i].id)
                            console.log(f)
                            f.childNodes.forEach(child => {
                                if (child.type == "checkbox" ) {
                                    if (child.checked) {
                                        sending.assigned_to.push(child.id)
                                    }
                                }
                            });

                            fetch("http://127.0.0.1:8000/api/assign_ticket/", {
                                method: 'POST',
                                body: JSON.stringify(sending)
                            })
                            .then(response => response.json())
                            return false
                        }

                        form.appendChild(submit)
                    
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
                });
        }) 
}

