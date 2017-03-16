CREATE TABLE bases (
    name VARCHAR(120) not null, 
    id UUID PRIMARY KEY default uuid_generate_v4(),

    updated_at TIMESTAMP default current_timestamp not null,
    created_at TIMESTAMP DEFAULT current_timestamp not null
);


CREATE TABLE pods (
    name VARCHAR(120) not null, 
    id UUID PRIMARY KEY default uuid_generate_v4(),

    user_id UUID references users(id) not null,
    base_id UUID references bases(id),

    updated_at TIMESTAMP default current_timestamp not null,
    created_at TIMESTAMP DEFAULT current_timestamp not null
);


-- CREATE TYPE resource_types AS ENUM ('steel', 'energy', 'water');

CREATE TABLE resources (
--    name resource_types not null, 
    name VARCHAR(120) not null, 
    amount bigint not null,
    max_amount bigint not null,
    UNIQUE (name, base_id, pod_id),
    CHECK (amount < max_amount),

    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null)
);


CREATE TABLE modules (
    name VARCHAR(120) not null, 
    UNIQUE (name, base_id, pod_id),

    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),

    updated_at TIMESTAMP default current_timestamp not null,
    created_at TIMESTAMP DEFAULT current_timestamp not null
);


CREATE TABLE researches (
    name VARCHAR(120) not null,
    UNIQUE (name, base_id, pod_id),

    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),

    updated_at TIMESTAMP default current_timestamp not null,
    created_at TIMESTAMP DEFAULT current_timestamp not null
);


CREATE TABLE queues (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    pod_id UUID references pods(id) on DELETE CASCADE,
    base_id UUID references bases(id) on DELETE CASCADE,
    CHECK (base_id is not null or pod_id is not null),

    slots integer not null default 2,
    CHECK (slots > 0),
    updated_at TIMESTAMP default current_timestamp not null,
    created_at TIMESTAMP default current_timestamp not null
);


CREATE TABLE queue_entries (
    id UUID PRIMARY KEY default uuid_generate_v4(),
    queue_id UUID references queues(id) on DELETE CASCADE not null,
    research_id UUID references researches(id),
    module_id UUID references modules(id),
    CHECK (module_id is not null or research_id is not null),

    duration INTERVAL not null,
    created_at TIMESTAMP default current_timestamp not null
);


SELECT diesel_manage_updated_at('bases');
SELECT diesel_manage_updated_at('pods');
SELECT diesel_manage_updated_at('modules');
SELECT diesel_manage_updated_at('researches');
SELECT diesel_manage_updated_at('queues');
SELECT diesel_manage_updated_at('queue_entries');
