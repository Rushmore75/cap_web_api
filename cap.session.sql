-- @block
SELECT assignment.assigned_to, ticket.id
FROM assignment 
INNER JOIN ticket ON assignment.ticket = ticket.id AND assignment.assigned_to = 5;

-- select all the tickets that aren't assigned
-- @block
SELECT ticket.id
FROM ticket
LEFT OUTER JOIN assignment
ON ticket.id = assignment.ticket
WHERE ASSIGNMENT.ticket IS NULL
;

-- @block
SELECT account.id
FROM dept 
INNER JOIN account ON dept.id = account.dept AND dept.dept_name = 'client' ;


-- @block
INSERT INTO dept (dept_name) VALUES('client');
INSERT INTO dept (dept_name) VALUES('flunky');
INSERT INTO dept (dept_name) VALUES('supervisor');

-- @block
UPDATE account SET dept = 4 WHERE email LIKE '%@email.com';

-- @block
UPDATE account SET dept = 4 WHERE email = 'ike@mail.com';