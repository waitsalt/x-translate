create table interfaces (
    interface_id integer primary key autoincrement,
    interface_name text not null unique,
    interface_desc text not null default '',
    provider text not null,
    model_name text not null,
    api_key text not null,
    base_url text not null,
    enable BOOLEAN not null default false
);
