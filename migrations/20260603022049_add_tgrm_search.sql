CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX idx_posts_trgm ON posts USING gin(title gin_trgm_ops, content gin_trgm_ops);