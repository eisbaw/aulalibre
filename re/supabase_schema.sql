-- Schema for the hosted Aula MCP server's token store.
--
-- Run in the Supabase dashboard (SQL Editor) for project udmwfkrdwytntnqnhrmv.
-- Holds the same LoginData JSON the CLI writes to tokens.json.

create table if not exists public.aula_tokens (
  id         text primary key,           -- 'default'
  data       jsonb not null,
  updated_at timestamptz not null default now()
);

-- No policies on purpose: a publishable/anon key reads nothing. Only the
-- project secret key (service_role, BYPASSRLS) can touch this table, so it
-- must live exclusively in server-side env vars.
alter table public.aula_tokens enable row level security;

-- Keep updated_at honest on upserts (PostgREST sends only id + data).
create or replace function public.aula_tokens_touch()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists aula_tokens_touch on public.aula_tokens;
create trigger aula_tokens_touch
  before update on public.aula_tokens
  for each row execute function public.aula_tokens_touch();
