CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password varchar not null,
    bio varchar null,
    image varchar null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table posts (
    id uuid default uuid_generate_v4() primary key,
    author_id uuid not null,
    slug varchar not null unique,
    title varchar not null,
    description varchar not null,
    body text not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp, 
    foreign key (author_id) references users(id)
);

create table comments (
    id uuid default uuid_generate_v4() primary key,
    author_id uuid not null,
    post_id uuid not null,
    body text not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp, 
    foreign key (author_id) references users(id),
    foreign key (post_id) references posts(id)
);

create table bands (
    id uuid default uuid_generate_v4() primary key,
    name varchar not null unique,
    owner_id uuid not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,

    foreign key (owner_id) references users(id)
);

create table answers (
    id uuid default uuid_generate_v4() primary key,
    content varchar not null unique,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table questions (
    id uuid default uuid_generate_v4() primary key,
    content varchar not null,
    correct_answer_id uuid not null,
    band_id uuid not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,

    foreign key (correct_answer_id) references answers(id),
    foreign key (band_id) references bands(id)
);
