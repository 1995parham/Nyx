CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE encrypted_content (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    encrypted_data TEXT NOT NULL,
    private_key TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);