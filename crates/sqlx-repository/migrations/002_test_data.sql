-- Insert test data for integration tests
-- This migration contains sample data that some tests may depend on

INSERT INTO users (name, email, status, department) VALUES
    ('John Doe', 'john@example.com', 'active', 'engineering'),
    ('Jane Smith', 'jane@example.com', 'active', 'marketing'),
    ('Bob Johnson', 'bob@example.com', 'inactive', 'engineering');

INSERT INTO posts (title, content, user_id, published) VALUES
    ('First Post', 'This is the first post content', 1, true),
    ('Second Post', 'This is the second post content', 1, false),
    ('Third Post', 'This is the third post content', 2, true);