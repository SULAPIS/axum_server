-- CREATE TABLE user_info (
--     user_id serial PRIMARY KEY,
--     number varchar(11) UNIQUE NOT NULL,
--     password varchar(11) NOT NULL,
--     token varchar UNIQUE NOT NULL,
--     register boolean DEFAULT FALSE
-- );
INSERT INTO user_info (number, password, token)
VALUES (
        '12345678901',
        '123456',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJudW1iZXIiOiIxMjM0NTY3ODkwMSIsInBhc3N3b3JkIjoiMTIzNDU2In0.p94YCkuaYCjyZn4f45k8Nxmj_iX-E_VDfQNa2gcDmom5z5UdjAwlzF04Gqmd8LH3GjzuxrteN2k57HIN6RP7jQ'
    );