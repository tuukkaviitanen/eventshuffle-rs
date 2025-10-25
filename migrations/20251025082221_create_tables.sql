CREATE TABLE event (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL
);

CREATE TABLE event_date (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    "date" DATE NOT NULL,
    "event_id" UUID NOT NULL
);

CREATE TABLE event_date_vote (
    "id" UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "event_date_id" UUID NOT NULL
);

ALTER TABLE "event_date" ADD CONSTRAINT "event_date_event_id_fkey" FOREIGN KEY ("event_id") REFERENCES "event"("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "event_date_vote" ADD CONSTRAINT "event_date_vote_event_date_id_fkey" FOREIGN KEY ("event_date_id") REFERENCES "event_date"("id") ON DELETE CASCADE ON UPDATE CASCADE;
