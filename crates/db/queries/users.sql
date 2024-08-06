--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM users;

--! create_user
INSERT INTO 
    users (email)
VALUES
    (:email);