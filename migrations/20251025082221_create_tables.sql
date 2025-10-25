CREATE TABLE "Event" (
    "id" CHAR(36) NOT NULL,
    "name" VARCHAR(255) NOT NULL,

    CONSTRAINT "Event_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "EventDate" (
    "id" CHAR(36) NOT NULL,
    "date" DATE NOT NULL,
    "event_id" CHAR(36) NOT NULL,

    CONSTRAINT "EventDate_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "EventDateVote" (
    "id" CHAR(36) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "event_date_id" CHAR(36) NOT NULL,

    CONSTRAINT "EventDateVote_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "EventDate" ADD CONSTRAINT "EventDate_event_id_fkey" FOREIGN KEY ("event_id") REFERENCES "Event"("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "EventDateVote" ADD CONSTRAINT "EventDateVote_event_date_id_fkey" FOREIGN KEY ("event_date_id") REFERENCES "EventDate"("id") ON DELETE CASCADE ON UPDATE CASCADE;
