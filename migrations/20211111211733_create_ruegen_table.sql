CREATE TABLE RUEGEN(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    identifier TEXT NOT NULL,
    title TEXT NOT NULL,
    ziffer TEXT NOT NULL,
    year SMALLINT NOT NULL
);