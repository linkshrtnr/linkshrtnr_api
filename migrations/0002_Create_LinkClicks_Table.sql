CREATE TABLE linkclicks (
    ID SERIAL PRIMARY KEY,
    LinkID INTEGER REFERENCES links(id) ON DELETE CASCADE,
    ClickTimestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    IPAddress VARCHAR(255),
    UserAgent TEXT,
    Referrer TEXT,
    DeviceInfo TEXT,
    GeographicLocation VARCHAR(255),
    UserID INTEGER REFERENCES users (id) ON DELETE SET NULL,
    ClickCount INTEGER
);
;
