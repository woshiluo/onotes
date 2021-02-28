-- Your SQL goes here
CREATE TABLE histories(
	id			INT PRIMARY KEY NOT NULL,
	post_id		INT				NOT NULL,
	time		INT 			NOT NULL,
	markdown	TEXT
)
