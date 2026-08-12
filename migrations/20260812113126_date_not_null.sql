Alter table comments
Alter column created_at
set NOT NULL,
Alter column updated_at
set NOT NULL;

Alter table posts
Alter column created_at
set NOT NULL,
Alter column updated_at
set NOT NULL;

Alter table users
Alter column created_at
set NOT NULL,
add column updated_at TimeStamp NOT NULL Default Current_timestamp;
