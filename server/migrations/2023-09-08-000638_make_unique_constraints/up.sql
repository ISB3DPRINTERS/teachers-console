ALTER TABLE users ADD CONSTRAINT users_email_unique UNIQUE (email);
ALTER TABLE users ADD CONSTRAINT users_username_unique UNIQUE (username);
ALTER TABLE users ADD CONSTRAINT users_token_unique UNIQUE (token);
