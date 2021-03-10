-- Your SQL goes here
CREATE TABLE posts (
	id				INT UNSIGNED	AUTO_INCREMENT,
	title			TEXT			NOT NULL,
	markdown		TEXT,
	PRIMARY KEY (`id`)
);

-- INSERT INDEX
INSERT INTO posts (title, markdown)
VALUES ( 'Index', '`Hello, World!`' );
