-- Your SQL goes here
CREATE TABLE post_edge(
	id			INT UNSIGNED	AUTO_INCREMENT,
	from_post	INT	UNSIGNED	NOT NULL,
	to_post		INT	UNSIGNED	NOT NULL,
	PRIMARY KEY (`id`)
)
