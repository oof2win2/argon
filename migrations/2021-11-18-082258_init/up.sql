CREATE TABLE main.services (
	id  INTEGER PRIMARY KEY UNIQUE NOT NULL,
	secret TEXT NOT NULL
);

INSERT INTO main.services VALUES (
	1,
	"1234"
);