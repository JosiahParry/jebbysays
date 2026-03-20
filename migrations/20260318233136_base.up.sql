CREATE TABLE objectives (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    context TEXT,
    priority INTEGER NOT NULL DEFAULT 3
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    created INTEGER NOT NULL,
    completed INTEGER,
    deadline INTEGER,
    priority INTEGER NOT NULL DEFAULT 3,
    title TEXT NOT NULL,
    context TEXT,
    tags jsonb,
    objective TEXT NOT NULL REFERENCES objectives(id)
);

INSERT INTO objectives (id, title, context, priority)
VALUES ('other', 'other', 'Default objective as a catch all for all tasks.', 3);
