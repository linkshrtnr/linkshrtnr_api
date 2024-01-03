CREATE TABLE LinkClicks (
    ID SERIAL PRIMARY KEY,
    LinkID INTEGER REFERENCES "Links"(id) ON DELETE CASCADE,
    ClickTimestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    IPAddress VARCHAR(255),
    UserAgent TEXT,
    Referrer TEXT,
    DeviceInfo TEXT,
    GeographicLocation VARCHAR(255),
    UserID INTEGER REFERENCES "User"(id) ON DELETE SET NULL,
    ClickCount INTEGER
);
;
