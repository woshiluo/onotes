-- Your SQL goes here
CREATE TABLE posts (
	id	INT PRIMARY KEY		NOT NULL,
	title			TEXT	NOT NULL,
	markdown		TEXT
);

-- INSERT INDEX
INSERT INTO posts (id, title, markdown)
VALUES ( 1, 'Index', '`Hello, World!`' );
