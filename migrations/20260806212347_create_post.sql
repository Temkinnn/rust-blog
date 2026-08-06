CREATE TABLE IF NOT EXISTS posts (
  id Uuid Primary key default uuidv7(),
  title Varchar(100) Not null Unique,
  slug Text Not Null unique,
  content Text not null,
  published Bool not null default false,
  tags Varchar(20) [], -- In future it will be separate table
  author_id Uuid references users (id) On Delete Cascade,
  created_at Timestamp Default Current_timestamp,
  updated_at TimeStamp Default Current_timestamp
);
