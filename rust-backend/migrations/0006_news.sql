-- Vague 1 : actualités (news) publiées par la fédération / les commissions.

CREATE TABLE public.news (
    id uuid PRIMARY KEY,
    title text NOT NULL,
    slug text UNIQUE NOT NULL,
    excerpt text,
    content text NOT NULL,
    image text,
    category text NOT NULL DEFAULT 'general',
    author_id uuid REFERENCES public.users(id) ON DELETE SET NULL,
    published boolean NOT NULL DEFAULT false,
    published_at timestamp with time zone,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX news_published_idx ON public.news (published, published_at DESC);
CREATE INDEX news_category_idx ON public.news (category);
