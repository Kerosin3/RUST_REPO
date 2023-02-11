-- Add migration script here
CREATE TABLE IF NOT EXISTS devices
(
    devid          INTEGER PRIMARY KEY AUTOINCREMENT,
    devname        VARCHAR(250)        NOT NULL UNIQUE,
    info           VARCHAR(250) DEFAULT 'NOTHING',
    active         BOOLEAN NOT NULL DEFAULT 0,
    attached_to_room VARCHAR(250),
    attached_to_house VARCHAR(250),
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(attached_to_room) REFERENCES rooms(roomname) ON DELETE CASCADE ON UPDATE CASCADE
    
);
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

