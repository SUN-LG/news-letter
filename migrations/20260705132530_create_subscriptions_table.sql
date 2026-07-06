-- Add migration script here
-- create subscriptions table
create table subscriptions(
  id bigint generated always as identity PRIMARY KEY,
  email text not null unique,
  name text not null,
  subscribe_at timestamptz not null
);
