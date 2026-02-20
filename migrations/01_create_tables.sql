CREATE TABLE logo
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE technology_category
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE technology
(
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    name                   TEXT    NOT NULL UNIQUE,
    technology_category_id INTEGER NOT NULL,
    logo_id                INTEGER NOT NULL,
    FOREIGN KEY (technology_category_id) REFERENCES technology_category (id) ON DELETE RESTRICT,
    FOREIGN KEY (logo_id) REFERENCES logo (id) ON DELETE RESTRICT
);

CREATE INDEX idx_technology_category ON technology (technology_category_id);

CREATE TABLE project_status
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE project
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    status_id      INTEGER NOT NULL,
    title          TEXT    NOT NULL UNIQUE,
    description    TEXT,
    stacks         TEXT    NOT NULL DEFAULT '[]',
    url_to_project TEXT,
    FOREIGN KEY (status_id) REFERENCES project_status (id) ON DELETE RESTRICT
);

CREATE INDEX idx_project_status ON project (status_id);
CREATE INDEX idx_project_stacks ON project (stacks);

CREATE TABLE careers
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    title         TEXT,
    year          INTEGER,
    parent_id     INTEGER,
    technology_id INTEGER,
    logo_id       INTEGER,
    FOREIGN KEY (parent_id) REFERENCES careers (id) ON DELETE SET NULL,
    FOREIGN KEY (technology_id) REFERENCES technology (id) ON DELETE SET NULL,
    FOREIGN KEY (logo_id) REFERENCES logo (id) ON DELETE SET NULL
);

CREATE INDEX idx_careers_parent ON careers (parent_id);
CREATE INDEX idx_careers_technology ON careers (technology_id);
CREATE INDEX idx_careers_year ON careers (year);

CREATE TABLE recommendation
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    logo_id INTEGER NOT NULL,
    author  TEXT,
    texte   TEXT,
    FOREIGN KEY (logo_id) REFERENCES logo (id) ON DELETE CASCADE
);

CREATE INDEX idx_recommendation_author ON recommendation (author);
CREATE INDEX idx_recommendation_logo_id ON recommendation (logo_id);