ALTER TABLE "image"
    ADD COLUMN "owner" text REFERENCES "user"("user_name") ON DELETE CASCADE NOT NULL;