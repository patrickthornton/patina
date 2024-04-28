create table posts (
    id integer not null primary key AUTOINCREMENT,
    author int not null references users(id),
    text text not null,
    hue int not null,
    reply_to int references posts(id),
    likes int not null default 0,
    timestamp int not null default 1714268741
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
