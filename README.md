# blogback
A Blogging Backend powered by Rust!

## Setup
### Prerequisites
- A running instance of PostgreSQL

### Setting Up the Database
1. Log into the cli with `psql -U postgres`
2. Run the following commands to set up the user, database and table
```sql
CREATE DATABASE my_database;
CREATE USER my_user WITH PASSWORD 'my_password';
GRANT ALL PRIVILEDGES ON DATABASE my_database TO my_user;

-- Connect to our database
\c my_database

-- Create the table
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL
    path TEXT NOT NULL,
    image_path TEXT NOT NULL,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Allow our user to access the table
GRANT USAGE, SELECT ON SEQUENCE posts_id_seq TO my_user;
GRANT ALL PRIVILEGES ON TABLE posts TO my_user;
```

### Installing blogback
```bash
# Clone the repo
git clone https://github.com/CommanderT3706/blogback
cd blogback
# Install the crate
cargo install --path .
```

### Creating the Config
blogback requires a configuration to be present at `/etc/blogback/config.toml` in order to work
```toml
# For hosting blogback
[server]
# The port for blogback to run on
port = 8080
# The root of your website
server_root = "/var/www/"

# For configuring the PostgreSQL database
[db]
# The address of your database
address = "localhost"
# The port of your database
port = 5432
# The name of your database
db_name = "my_database"
# The name of your user
db_user = "my_user"
# The password of your user
db_passwd = "my_password"

# For generating the sitemap
[site]
# The homepage of your site
homepage = "https://example.com/"
# The posts page of your site
posts_page = "https://example.com/posts"
# Any other static pages on your site
static_pages = [ "https://example.com/apple", "https://example/com/banana" ]
# Default lastmod for static pages
static_lastmod = "2024-11-09"
# Default changefreq for all pages
default_changefreq = "monthly"
```

### Using blogback
Here are some examples on how to use blogback
```bash
# Display the help menu
blogback --help

# Test the connection to the database
blogback test

# Run blogback
blogback serve

# Create a post (may need to be run as sudo depending on your server root)
blogback post "My Title" "My Description" "https://example.com/path/to/website" "https://example.com/path/to/an/image"

# Force update the sitemap (may need to be run as sudo depending on your server root)
blogback sitemap
```