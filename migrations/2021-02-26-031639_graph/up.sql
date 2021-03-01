-- Your SQL goes here
CREATE TABLE post_edge(
	id			INTEGER			PRIMARY KEY	AUTOINCREMENT,
	from_post	INT				NOT NULL,
	to_post		INT				NOT NULL
)
