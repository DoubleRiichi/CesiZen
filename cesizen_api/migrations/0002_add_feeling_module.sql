-- Add migration script here
-- ========================================
-- FEELING CATEGORY
-- ========================================

CREATE TABLE feeling_category (
                                  id          SERIAL PRIMARY KEY,
                                  name        TEXT NOT NULL UNIQUE,
                                  created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                  updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER trigger_feeling_category_updated_at
    BEFORE UPDATE ON feeling_category
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


-- ========================================
-- FEELING
-- ========================================

CREATE TABLE feeling (
                         id                  SERIAL PRIMARY KEY,
                         feeling_category_id INTEGER NOT NULL REFERENCES feeling_category(id) ON DELETE CASCADE,
                         name                TEXT NOT NULL,
                         created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

                         UNIQUE (feeling_category_id, name)
);

CREATE INDEX idx_feeling_category_id
    ON feeling(feeling_category_id);

CREATE TRIGGER trigger_feeling_updated_at
    BEFORE UPDATE ON feeling
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


-- ========================================
-- FEELING TRACKER
-- ========================================

CREATE TABLE feeling_tracker (
                                 id              SERIAL PRIMARY KEY,
                                 user_id         INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
                                 feeling_id      INTEGER NOT NULL REFERENCES feeling(id) ON DELETE CASCADE,
                                 timestamp_start TIMESTAMP NOT NULL,
                                 timestamp_end   TIMESTAMP NOT NULL,
                                 intensity       SMALLINT NOT NULL CHECK (intensity BETWEEN 1 AND 10),
                                 notes           TEXT DEFAULT '',
                                 location        TEXT DEFAULT '',
                                 created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_feeling_tracker_user
    ON feeling_tracker(user_id);

CREATE INDEX idx_feeling_tracker_feeling
    ON feeling_tracker(feeling_id);

CREATE TRIGGER trigger_feeling_tracker_updated_at
    BEFORE UPDATE ON feeling_tracker
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

INSERT INTO feeling_category (name)
VALUES
    ('Positive'),
    ('Negative'),
    ('Neutral');

INSERT INTO feeling (feeling_category_id, name)
VALUES
    (1, 'Happy'),
    (1, 'Excited'),
    (2, 'Sad'),
    (2, 'Angry'),
    (3, 'Calm');
-- (The deleted article has no tags — just for testing)
