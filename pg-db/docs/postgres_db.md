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
	uid serial,
	docid INT NOT NULL,
	parentid INT,
	title TEXT NOT NULL,
  identity SMALLINT NOT NULL,
	createdat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updatedat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (docid)
);
CREATE TABLE booknode (
	uid serial,
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

CREATE INDEX docid ON title(docid);
CLUSTER title USING docid;
CREATE INDEX bookNodeIndex ON booknode(docid, pageid);
CLUSTER booknode USING bookNodeIndex;

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
	uid serial,
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

CREATE INDEX blogNodeIndex ON blognode(docid);
CLUSTER blognode USING blogNodeIndex;

DROP TABLE book;
DROP TABLE title;
DROP TABLE booknode;
DROP TABLE blog;
DROP TABLE blognode;
```
