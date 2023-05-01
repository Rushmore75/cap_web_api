# What?
This is the capstone project from my time in college. It's a help desk submission website. You have client accounts who can submit tickets, supervisor accounts who can assign tickets to employees, and employees who can complete tickets assigned to them.

Backend is all built in Rust, with Rocket for website handling and Diesel ORM for interacting with the PostgreSQL database.
Frontend is JS/HTML/CSS

# Next
It could use some CSS help, it's just very time consuming.

# Developing:
You will need [diesel](https://diesel.rs/) installed to work with the ORM.

Use `diesel migration run` to set up the databases the first time. If you need to reset the database you can use `diesel migration redo`.

Included is a docker compose file that contains a postgres database for easy setup.


## Notes
You need access to the database to (de)escalate user privileges.
