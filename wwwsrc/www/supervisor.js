
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
                        form.id = i

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
                        submit.onclick = console.log(i)
                        form.appendChild(submit)
                    
                        header.innerText = ticket[i][0]
                        body.innerText = ticket[i][1]
                        div.appendChild(header)
                        div.appendChild(body)
                        div.appendChild(form)
                        tickets.appendChild(div)
                    }
                });
        }) 
}

