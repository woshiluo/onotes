-- Your SQL goes here
CREATE TABLE users( 
	id			INTEGER				PRIMARY KEY	AUTOINCREMENT,
	nickname	TEXT	UNIQUE		NOT NULL,
	password	TEXT				NOT NULL,
	email		TEXT				NOT NULL,
	admin		INT		DEFAULT(0)	NOT NULL
);

CREATE TABLE tokens( 
	id			INTEGER				PRIMARY KEY	AUTOINCREMENT,
	user_id		INT					NOT NULL,
	token		TEXT				NOT NULL
);
