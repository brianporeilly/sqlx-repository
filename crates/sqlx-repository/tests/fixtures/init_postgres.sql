-- Initialize PostgreSQL test database schema

-- Users table with soft delete support
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    status VARCHAR DEFAULT 'active',
    department VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Posts table without soft delete
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    user_id INTEGER REFERENCES users(id),
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert test data
INSERT INTO users (name, email, status, department) VALUES
    ('John Doe', 'john@example.com', 'active', 'engineering'),
    ('Jane Smith', 'jane@example.com', 'active', 'marketing'),
    ('Bob Johnson', 'bob@example.com', 'inactive', 'engineering');

INSERT INTO posts (title, content, user_id, published) VALUES
    ('First Post', 'This is the first post content', 1, true),
    ('Second Post', 'This is the second post content', 1, false),
    ('Third Post', 'This is the third post content', 2, true);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_department ON users(department);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_published ON posts(published);