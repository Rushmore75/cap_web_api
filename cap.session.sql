
-- select all tickets where ticket.id = assignment.ticket where assignment.assigned_to = 5
-- @block
SELECT assignment.assigned_to, ticket.id
FROM assignment 
INNER JOIN ticket ON assignment.ticket = ticket.id AND assignment.assigned_to = 5;ed_to = 5

-- @block
SELECT ticket.id
FROM ticket
WHERE assignment.ticket != ticket.id;
