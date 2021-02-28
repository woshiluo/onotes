-- Your SQL goes here
CREATE TABLE users( 
	id			INT		PRIMARY KEY NOT NULL,
	nickname	TEXT	UNIQUE		NOT NULL,
	password	TEXT				NOT NULL,
	email		TEXT				NOT NULL,
	admin		INT		DEFAULT(0)	NOT NULL
);

CREATE TABLE tokens( 
	id			INT		PRIMARY KEY NOT NULL,
	user_id		INT					NOT NULL,
	token		TEXT				NOT NULL
);
