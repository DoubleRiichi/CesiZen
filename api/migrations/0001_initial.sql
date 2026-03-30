-- ========================================
-- FULL MIGRATION WITH SINGULAR TABLE NAMES
-- (as you specifically requested)
-- Run this to recreate the schema cleanly.
-- Matches your current Rust code (author_id column + ArticleWithAuthorRow)
-- ========================================

CREATE TYPE user_role AS ENUM ('Admin', 'Mod', 'User');
CREATE TYPE article_visibility AS ENUM ('Public', 'Unlisted', 'Private');

CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- USER table (quoted because "user" is a reserved word)
CREATE TABLE "user" (
                        id            SERIAL PRIMARY KEY,
                        username      TEXT NOT NULL UNIQUE,
                        email         TEXT NOT NULL UNIQUE,
                        password      TEXT NOT NULL,
                        role          user_role NOT NULL,
                        age           SMALLINT NOT NULL,
                        avatar        TEXT NOT NULL DEFAULT '',
                        is_active     BOOLEAN NOT NULL DEFAULT TRUE,
                        created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER trigger_user_updated_at
    BEFORE UPDATE ON "user"
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- TAG table
CREATE TABLE tag (
                     id   SERIAL PRIMARY KEY,
                     name TEXT NOT NULL UNIQUE
);

-- ARTICLE table (author_id to match your repository code)
CREATE TABLE article (
                         id          SERIAL PRIMARY KEY,
                         author_id   INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
                         title       TEXT NOT NULL,
                         content     TEXT NOT NULL,
                         is_deleted  BOOLEAN NOT NULL DEFAULT FALSE,
                         visibility  article_visibility NOT NULL DEFAULT 'Public',
                         created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER trigger_article_updated_at
    BEFORE UPDATE ON article
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE INDEX idx_article_author_id ON article(author_id);

-- ARTICLE_TAG junction table (singular, as you requested)
CREATE TABLE article_tag (
                             id          SERIAL PRIMARY KEY,
                             article_id  INTEGER NOT NULL REFERENCES article(id) ON DELETE CASCADE,
                             tag_id      INTEGER NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
                             UNIQUE (article_id, tag_id)
);

CREATE INDEX idx_article_tag_article_id ON article_tag(article_id);
CREATE INDEX idx_article_tag_tag_id     ON article_tag(tag_id);




-- ========================================
-- Done! Now re-run your sample data script
-- (just change the article INSERT column from user_id → author_id)
-- ========================================
-- ========================================
-- SAMPLE DATA SEEDING SCRIPT
-- Singular table names as requested: "user", tag, article, article_tag
-- Run this AFTER your migration (make sure tables are created with singular names)
-- Note: "user" is a reserved word in PostgreSQL, so we quote it as "user"
-- ========================================

-- 1. Sample users (passwords are plain-text for demo only — hash them in production!)
INSERT INTO "user" (username, email, password, role, age, avatar, is_active)
VALUES
    ('alice',   'alice@example.com',   'alice123',   'Admin', 32, 'https://i.pravatar.cc/150?img=1',  TRUE),
    ('bob',     'bob@example.com',     'bob123',     'User',  27, 'https://i.pravatar.cc/150?img=2',  TRUE),
    ('charlie', 'charlie@example.com', 'charlie123', 'Mod',   35, 'https://i.pravatar.cc/150?img=3', FALSE);

-- 2. Sample tags
INSERT INTO tag (name)
VALUES
    ('Rust'),
    ('SQL'),
    ('Web Development'),
    ('AI'),
    ('Backend');

-- 3. Sample articles
INSERT INTO article (author_id, title, content, is_deleted, visibility)
VALUES
    -- Alice (Admin) - 2 public articles
    (1, 'Why Rust is Taking Over in 2026',
     'Rust''s memory safety and performance make it the perfect choice for systems programming and web backends. Here''s why I switched...',
     FALSE, 'Public'),

    (1, 'Mastering SQL Window Functions',
     'Window functions like ROW_NUMBER(), RANK(), and LAG() are game-changers. Let me show you real examples.',
     FALSE, 'Public'),

    -- Bob (User) - one private article
    (2, 'My Secret Project Ideas',
     'Just some personal notes about my next big side project. Not ready for the world yet.',
     FALSE, 'Private'),

    -- Charlie (Mod) - one unlisted article + one deleted for testing
    (3, 'How AI is Changing Content Creation',
     'Quick thoughts on Grok, GPT, and the future of writing tools.',
     FALSE, 'Unlisted'),

    (3, 'Old Draft - Delete Me',
     'This article is just a test for the is_deleted flag.',
     TRUE, 'Public');

-- 4. Sample article <-> tag relationships (many-to-many)
INSERT INTO article_tag (article_id, tag_id)
VALUES
    -- Alice's first article: Rust + Backend
    (1, 1),  -- Rust
    (1, 5),  -- Backend

    -- Alice's second article: SQL
    (2, 2),  -- SQL

    -- Bob's private article: Web Development + AI
    (3, 3),  -- Web Development
    (3, 4),  -- AI

    -- Charlie's unlisted article: AI + Rust
    (4, 4),  -- AI
    (4, 1);  -- Rust



-- ========================================
-- Verify everything worked:
-- ========================================
-- SELECT * FROM "user";
-- SELECT * FROM tag;
-- SELECT * FROM article;
-- SELECT * FROM article_tag;

-- Optional: Quick check for ArticleWithAuthorRow style query
-- SELECT
--     a.id, a.title, a.content, a.is_deleted, a.created_at, a.updated_at,
--     u.id as user_id, u.username, u.role as user_role, u.avatar,
--     a.visibility,
--     ARRAY_AGG(t.name) as tags
-- FROM article a
-- JOIN "user" u ON a.user_id = u.id
-- LEFT JOIN article_tag at ON a.id = at.article_id
-- LEFT JOIN tag t ON at.tag_id = t.id
-- WHERE a.is_deleted = FALSE
-- GROUP BY a.id, u.id;