-- Your SQL goes here
CREATE TABLE post_edge(
	id		INT PRIMARY KEY	NOT NULL,
	from_post	INT				NOT NULL,
	to_post		INT				NOT NULL
)
