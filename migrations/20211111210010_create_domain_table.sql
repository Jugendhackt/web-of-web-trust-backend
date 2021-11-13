CREATE TABLE DOMAINS (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    fqdn_hash TEXT UNIQUE NOT NULL,
    fqdn TEXT UNIQUE NOT NULL,
    last_updated BIGINT NOT NULL
);
CREATE INDEX domains_fqdn_idx ON domains (fqdn_hash);
