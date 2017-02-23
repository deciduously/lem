-- Your SQL goes here
CREATE TABLE sysinfo (
  id SERIAL PRIMARY KEY,
  datetime TIMESTAMP NOT NULL,
  uname TEXT NOT NULL,
  uptime TEXT NOT NULL
)