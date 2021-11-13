CREATE TABLE RUEGEN_DOMAINS(
    ruegen_id INT REFERENCES ruegen(id) NOT NULL,
    domain_id INT REFERENCES domains(id) NOT NULL,
    CONSTRAINT dr_pk PRIMARY KEY (ruegen_id, domain_id)
);