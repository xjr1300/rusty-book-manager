INSERT INTO roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$B4fCusqV6Ke8Eip1C3/ZMOg3YwLBO.AQkd3zxU62N.iv43QEUbJ22',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
