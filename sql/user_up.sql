-- CREATE TABLE user_info (
--     user_id serial PRIMARY KEY,
--     number varchar UNIQUE NOT NULL,
--     password varchar NOT NULL,
--     token varchar UNIQUE NOT NULL
-- );
-- drop TABLE order_info;
-- CREATE TABLE order_info (
--     order_id serial PRIMARY KEY,
--     user_id integer NOT NULL,
--     user2_id integer DEFAULT NULL,
--     c_state integer default 0,
--     cartype integer NOT NULL,
--     c_type varchar,
--     rent real default 0,
--     ton real default 0,
--     address varchar,
--     c_time varchar
-- ) 
drop table order_detail;
CREATE TABLE order_detail (
    order_id serial PRIMARY KEY,
    user_id integer NOT NULL,
    user2_id integer DEFAULT NULL,
    c_state integer default 0 NOT NULL,
    detail TEXT
) -- INSERT INTO user_info (number, password, token)
-- VALUES (
--         '12345678901',
--         '123456',
--         'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJudW1iZXIiOiIxMjM0NTY3ODkwMSIsInBhc3N3b3JkIjoiMTIzNDU2In0.p94YCkuaYCjyZn4f45k8Nxmj_iX-E_VDfQNa2gcDmom5z5UdjAwlzF04Gqmd8LH3GjzuxrteN2k57HIN6RP7jQ'
--     );
-- INSERT INTO order_info (
--         user_id,
--         c_state,
--         cartype,
--         c_type,
--         rent,
--         ton,
--         address,
--         c_time
--     )
-- VALUES(
--         1,
--         0,
--         0,
--         '带破碎锤',
--         1.2,
--         35,
--         '饭都花园1楼一号',
--         '2022-01-02 09:00:00.'
--     )