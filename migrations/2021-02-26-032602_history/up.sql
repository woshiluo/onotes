-- Your SQL goes here
CREATE TABLE histories(
	id			INT 	UNSIGNED	AUTO_INCREMENT,
	post_id		INT		UNSIGNED	NOT NULL,
	time		INT 	UNSIGNED	NOT NULL,
	markdown	TEXT,
	PRIMARY KEY (`id`)
)
