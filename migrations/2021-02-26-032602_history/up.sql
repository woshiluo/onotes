-- Your SQL goes here
CREATE TABLE histories(
	id			INTEGER			PRIMARY KEY	AUTOINCREMENT,
	post_id		INT				NOT NULL,
	time		INT 			NOT NULL,
	markdown	TEXT
)
