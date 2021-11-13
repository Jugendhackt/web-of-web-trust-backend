CREATE TABLE DOMAIN_LINK(
    source_id INT REFERENCES domains(id) NOT NULL,
    target_id INT REFERENCES domains(id) NOT NULL,
    CONSTRAINT dl_pk PRIMARY KEY (source_id, target_id)
);
