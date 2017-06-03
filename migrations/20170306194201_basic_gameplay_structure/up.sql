CREATE TABLE bases (
    name VARCHAR(120) not null, 
    id UUID PRIMARY KEY default uuid_generate_v4(),

    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


CREATE TABLE pods (
    name VARCHAR(120) not null, 
    id UUID PRIMARY KEY default uuid_generate_v4(),

    user_id UUID references users(id) not null,
    base_id UUID references bases(id),

    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


CREATE TABLE resources (
    name VARCHAR(120) not null, 
    amount bigint not null,
    production bigint default 0 not null,
    max_amount bigint not null,
    UNIQUE (name, base_id, pod_id),
    CHECK (amount <= max_amount),

    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null)
);


CREATE TABLE modules (
    name VARCHAR(120) not null, 
    level integer not null default 0,
    stationary BOOLEAN not null default FALSE ,
    x_pos integer,
    y_pos integer,
    UNIQUE (name, base_id, pod_id),
    CHECK (
        (stationary is true and x_pos is null and y_pos is null) or
        (stationary is false and x_pos is not null and y_pos is not null)
    ),
    CHECK (
        (not(
                (x_pos is null and y_pos is not null) or
                (y_pos is null and x_pos is not null)
        ))
    ),


    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),

    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


CREATE TABLE researches (
    name VARCHAR(120) not null,
    level integer not null default 0,
    UNIQUE (name, base_id, pod_id),

    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),


    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


CREATE TABLE queues (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),

    slots integer not null default 2,
    CHECK (slots > 0),
    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


CREATE TABLE queue_entries (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    queue_id UUID references queues(id) on DELETE CASCADE not null,
    module_id UUID references modules(id),
    module_name VARCHAR(120),
    research_id UUID references researches(id),
    research_name VARCHAR(120),
    level integer not null,
    duration bigint not null,
    CHECK (module_name is not null or research_name is not null),

--    duration INTERVAL not null,
    finishes_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE default current_timestamp not null,
    created_at TIMESTAMP WITH TIME ZONE default current_timestamp not null
);


SELECT diesel_manage_updated_at('bases');
SELECT diesel_manage_updated_at('pods');
SELECT diesel_manage_updated_at('modules');
SELECT diesel_manage_updated_at('researches');
SELECT diesel_manage_updated_at('queues');
SELECT diesel_manage_updated_at('queue_entries');
