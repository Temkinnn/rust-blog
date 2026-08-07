-- Add migration script here
alter table posts
alter column author_id
set Not null;
