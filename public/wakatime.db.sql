--
-- SQLiteStudio v3.4.4 生成的文件，周四 5月 25 21:46:54 2023
--
-- 所用的文本编码：UTF-8
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- 表：categories
DROP TABLE IF EXISTS categories;

CREATE TABLE IF NOT EXISTS categories (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       INTEGER NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL
);


-- 表：dependencies
DROP TABLE IF EXISTS dependencies;

CREATE TABLE IF NOT EXISTS dependencies (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       REAL    NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：editors
DROP TABLE IF EXISTS editors;

CREATE TABLE IF NOT EXISTS editors (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       REAL    NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：grand_total
DROP TABLE IF EXISTS grand_total;

CREATE TABLE IF NOT EXISTS grand_total (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：languages
DROP TABLE IF EXISTS languages;

CREATE TABLE IF NOT EXISTS languages (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       REAL    NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：machines
DROP TABLE IF EXISTS machines;

CREATE TABLE IF NOT EXISTS machines (
    id              INTEGER PRIMARY KEY AUTOINCREMENT
                            NOT NULL,
    range_id        INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                  ON UPDATE NO ACTION
                            NOT NULL,
    decimal         TEXT    NOT NULL,
    digital         TEXT    NOT NULL,
    hours           INTEGER NOT NULL,
    machine_name_id TEXT    NOT NULL,
    minutes         INTEGER NOT NULL,
    name            TEXT    NOT NULL,
    percent         REAL    NOT NULL,
    seconds         INTEGER NOT NULL,
    text            TEXT    NOT NULL,
    total_seconds   REAL    NOT NULL
);


-- 表：operating_systems
DROP TABLE IF EXISTS operating_systems;

CREATE TABLE IF NOT EXISTS operating_systems (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       REAL    NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：projects
DROP TABLE IF EXISTS projects;

CREATE TABLE IF NOT EXISTS projects (
    id            INTEGER PRIMARY KEY AUTOINCREMENT
                          NOT NULL,
    range_id      INTEGER REFERENCES range (id) ON DELETE NO ACTION
                                                ON UPDATE NO ACTION
                          NOT NULL,
    decimal       TEXT    NOT NULL,
    digital       TEXT    NOT NULL,
    hours         INTEGER NOT NULL,
    minutes       INTEGER NOT NULL,
    name          TEXT    NOT NULL,
    percent       REAL    NOT NULL,
    seconds       INTEGER NOT NULL,
    text          TEXT    NOT NULL,
    total_seconds REAL    NOT NULL
);


-- 表：range
DROP TABLE IF EXISTS range;

CREATE TABLE IF NOT EXISTS range (
    id       INTEGER PRIMARY KEY AUTOINCREMENT
                     NOT NULL,
    date     TEXT    NOT NULL,
    end      TEXT    NOT NULL,
    start    TEXT    NOT NULL,
    text     TEXT    NOT NULL,
    timezone TEXT    NOT NULL
);


COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
