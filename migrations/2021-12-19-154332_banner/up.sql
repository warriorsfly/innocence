
CREATE TABLE banners (
    id SERIAL PRIMARY KEY,
    -- 0->system banner 1->book 2->post 3->ads
    url TEXT NOT NULL DEFAULT '',
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- CREATE INDEX books_authors_idx ON banners (authors);
-- indices are already created for slugs, as slugs are unique as per the spec

SELECT diesel_manage_updated_at('banners');