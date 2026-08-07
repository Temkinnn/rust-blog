-- Add migration script here
Alter table posts
Alter column tags
SET NOT NULL;
