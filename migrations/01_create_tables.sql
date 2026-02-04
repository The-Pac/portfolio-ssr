
CREATE TABLE logo (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      path TEXT NOT NULL,
                      name TEXT NOT NULL
);

CREATE TABLE technology_category (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     title TEXT NOT NULL,
                                     description TEXT
);

CREATE TABLE technology (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            name TEXT NOT NULL,
                            technology_category_id INTEGER NOT NULL,
                            logo_id INTEGER NOT NULL,
                            FOREIGN KEY (technology_category_id) REFERENCES technology_category(id) ON DELETE CASCADE,
                            FOREIGN KEY (logo_id) REFERENCES logo(id) ON DELETE SET NULL
);

CREATE TABLE project_status (
                                id INTEGER PRIMARY KEY AUTOINCREMENT,
                                name TEXT NOT NULL UNIQUE
);

CREATE TABLE project (
                         id INTEGER PRIMARY KEY AUTOINCREMENT,
                         status_id INTEGER NOT NULL,
                         description TEXT,
                         title TEXT NOT NULL,
                         stacks TEXT NOT NULL,
                         url_to_project TEXT,
                         FOREIGN KEY (status_id) REFERENCES project_status(id) ON DELETE RESTRICT
);

CREATE INDEX project_stacks_idx ON project(stacks);

CREATE TABLE careers (
                         id INTEGER PRIMARY KEY AUTOINCREMENT,
                         title TEXT,
                         year INTEGER,
                         parent_id INTEGER,
                         technology_id INTEGER,
                         logo_id INTEGER,
                         FOREIGN KEY (parent_id) REFERENCES careers(id) ON DELETE SET NULL,
                         FOREIGN KEY (technology_id) REFERENCES technology(id) ON DELETE SET NULL,
                         FOREIGN KEY (logo_id) REFERENCES logo(id) ON DELETE SET NULL
);

CREATE TABLE project_technology (
                                    project_id INTEGER NOT NULL,
                                    technology_id INTEGER NOT NULL,
                                    PRIMARY KEY (project_id, technology_id),
                                    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE,
                                    FOREIGN KEY (technology_id) REFERENCES technology(id) ON DELETE CASCADE
);

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;