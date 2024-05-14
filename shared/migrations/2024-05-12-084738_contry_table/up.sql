-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE countries (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    alpha2_code CHAR(2) NOT NULL ,
    "name" VARCHAR(100) NOT NULL,
    CONSTRAINT PK_countries PRIMARY KEY (id),
    CONSTRAINT UQ_countries_alpha2_code UNIQUE (alpha2_code),
    CONSTRAINT UQ_ccountries_name UNIQUE ("name")
 )