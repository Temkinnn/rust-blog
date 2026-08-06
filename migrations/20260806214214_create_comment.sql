CREATE TABLE IF NOT EXISTS comments (
  id Uuid Primary key default uuidv7(),
  content Text Not null,
  author_id Uuid references users (id) on delete set null,
  post_id Uuid references posts (id) on delete cascade,
  created_at Timestamp default Current_timestamp
);
