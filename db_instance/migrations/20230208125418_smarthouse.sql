-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    roomid          INTEGER PRIMARY KEY AUTOINCREMENT,
    roomname        VARCHAR(250)        NOT NULL UNIQUE,
    info            VARCHAR(250) DEFAULT 'NOTHING',
    attached_to_house VARCHAR(250),
    FOREIGN KEY(attached_to_house) REFERENCES smarthouse(housename) ON DELETE CASCADE ON UPDATE CASCADE
    
);
CREATE TABLE IF NOT EXISTS smarthouse
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    housename        VARCHAR(250)        NOT NULL UNIQUE,
    active      BOOLEAN             NOT NULL DEFAULT 0
);


