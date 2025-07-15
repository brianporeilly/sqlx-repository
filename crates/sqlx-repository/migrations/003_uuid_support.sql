-- Add UUID support to the test database
-- This migration demonstrates UUID table structures for testing

-- Enable UUID extension (required for UUID functions)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create UUID-based users table for testing
CREATE TABLE uuid_users (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    status VARCHAR DEFAULT 'active',
    department VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create UUID-based posts table for testing
CREATE TABLE uuid_posts (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES uuid_users(id),
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_uuid_users_email ON uuid_users(email);
CREATE INDEX idx_uuid_users_deleted_at ON uuid_users(deleted_at);
CREATE INDEX idx_uuid_posts_user_id ON uuid_posts(user_id);

-- Insert sample UUID data for testing
INSERT INTO uuid_users (id, name, email, status, department) VALUES
    (uuid_generate_v4(), 'UUID Alice', 'uuid_alice@example.com', 'active', 'engineering'),
    (uuid_generate_v4(), 'UUID Bob', 'uuid_bob@example.com', 'active', 'marketing'),
    (uuid_generate_v4(), 'UUID Carol', 'uuid_carol@example.com', 'inactive', 'engineering');