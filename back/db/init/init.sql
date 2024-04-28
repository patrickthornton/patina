create table posts (
    id integer not null primary key AUTOINCREMENT,
    author int not null references users(id),
    reply_to int references posts(id),
    text text not null,
    likes int not null default 0,
    timestamp int not null default 0
);

create table users (
    id integer not null primary key AUTOINCREMENT,
    name text not null unique
);

create table followers (
    follower int not null references users(id),
    followee int not null references users(id),
    primary key (follower, followee)
);

create unique index unique_follows on followers(follower, followee);

create table likes (
    post int not null references posts(id),
    user int not null references users(id),
    primary key (post, user)
);

create unique index unique_likes on likes(post, user);
