-- Vague 3 : notifications in-app (compteur dans le header, page /notifications).

CREATE TABLE public.notifications (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    kind text NOT NULL,
    title text NOT NULL,
    message text NOT NULL,
    link text,
    read boolean NOT NULL DEFAULT false,
    created_at timestamp with time zone NOT NULL
);

CREATE INDEX notifications_user_idx ON public.notifications (user_id, read, created_at DESC);
