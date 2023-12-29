create table links (
    id serial primary key,
    original_url text not null,
    short_url text not null,
    created_at date default current_date
)