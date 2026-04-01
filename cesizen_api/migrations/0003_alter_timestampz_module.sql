-- ========================================
-- USER
-- ========================================

ALTER TABLE "user"
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';


-- ========================================
-- FEELING CATEGORY
-- ========================================

ALTER TABLE feeling_category
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';


-- ========================================
-- FEELING
-- ========================================

ALTER TABLE feeling
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';


-- ========================================
-- FEELING TRACKER
-- ========================================

ALTER TABLE feeling_tracker
ALTER COLUMN timestamp_start TYPE TIMESTAMPTZ USING timestamp_start AT TIME ZONE 'UTC',
    ALTER COLUMN timestamp_end   TYPE TIMESTAMPTZ USING timestamp_end   AT TIME ZONE 'UTC',
    ALTER COLUMN created_at      TYPE TIMESTAMPTZ USING created_at      AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at      TYPE TIMESTAMPTZ USING updated_at      AT TIME ZONE 'UTC';


-- ========================================
-- ARTICLE
-- ========================================

ALTER TABLE article
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';