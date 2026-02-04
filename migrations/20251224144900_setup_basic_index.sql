-- Files
CREATE TABLE IF NOT EXISTS file (
    id   INTEGER PRIMARY KEY,
    path varchar(1000) NOT NULL,
    indexed_at STRING NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_path
ON file (path);

-- Symbols
CREATE TABLE IF NOT EXISTS symbol (
    id   INTEGER PRIMARY KEY,
    file_id INTEGER NOT NULL,
    kind varchar(255) NOT NULL,
    name varchar(255) NOT NULL,
    start_line INTEGER NOT NULL,
    start_column INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    end_column INTEGER NOT NULL,
    indexed_at STRING NOT NULL,
    FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_path_kind_name
ON symbol (file_id, name, start_line, start_column, end_line, end_column);

