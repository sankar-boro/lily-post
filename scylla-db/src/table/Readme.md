USER -> user
  uid
  fname
  lname
  email
  password
  createdat
  updatedat

BOOK -> book
  uid
  title
  body
  metadata
  image_url
  createdat
  updatedat

BLOG -> blog
  uid
  title
  body
  metadata
  image_url
  createdat
  updatedat

BOOKNODE_1 -> booknode_1
  uid
  title
  body
  identity
  book_id
  page_id
  top_unique_id
  metadata
  image_url
  created_at
  updated_at

BLOGNODE_1 -> blognode_1
  uid
  title
  body
  identity
  blog_id
  page_id
  top_unique_id
  metadata
  image_url
  created_at
  updated_at

