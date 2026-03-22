ALTER TABLE tasks ADD COLUMN depends_on TEXT REFERENCES tasks(id);
