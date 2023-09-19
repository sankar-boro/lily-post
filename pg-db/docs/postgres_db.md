### Postgres shortcuts
```psql
\dt;		// list database tables;
exit;		// exit
```

### Create Users table

```sql
CREATE TABLE users (
	uid serial PRIMARY KEY,
	fname VARCHAR ( 50 ) NOT NULL,
	lname VARCHAR ( 50 ) NOT NULL,
	email VARCHAR ( 50 ) UNIQUE NOT NULL,
	password TEXT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

### Create Books table

```sql
CREATE TABLE book (
	uid serial PRIMARY KEY,
  authorid INT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
  imageurl TEXT,
  metadata TEXT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE title (
	uid serial PRIMARY KEY,
	docid INT NOT NULL,
	parentid INT,
	title TEXT NOT NULL,
  identity SMALLINT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE booknode (
	uid serial PRIMARY KEY,
  authorid INT NOT NULL,
	docid INT NOT NULL,
	pageid INT NOT NULL,
	parentid INT,
  identity SMALLINT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
  imageurl TEXT,
  metadata TEXT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX booknodeidx ON booknode (docid, pageid);

DROP TABLE book;
DROP TABLE title;
DROP TABLE booknode;

```



### Create Blogs table

```sql
CREATE TABLE blog (
	uid serial PRIMARY KEY,
  	authorid INT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
  	imageurl TEXT,
  	metadata TEXT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  	updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE blognode (
	uid serial PRIMARY KEY,
  	authorid INT NOT NULL,
	docid INT NOT NULL,
	parentid INT,
  	identity SMALLINT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
  	imageurl TEXT,
  	metadata TEXT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  	updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

DROP TABLE blog;
DROP TABLE blognode;
```