CREATE TABLE event (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,

    CONSTRAINT "event_pkey" PRIMARY KEY ("id")
);

CREATE TABLE event_date (
    "id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "event_id" UUID NOT NULL,

    CONSTRAINT "event_date_pkey" PRIMARY KEY ("id")
);

CREATE TABLE event_date_vote (
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "event_date_id" UUID NOT NULL,

    CONSTRAINT "event_date_vote_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "event_date" ADD CONSTRAINT "event_date_event_id_fkey" FOREIGN KEY ("event_id") REFERENCES "event"("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "event_date_vote" ADD CONSTRAINT "event_date_vote_event_date_id_fkey" FOREIGN KEY ("event_date_id") REFERENCES "event_date"("id") ON DELETE CASCADE ON UPDATE CASCADE;
