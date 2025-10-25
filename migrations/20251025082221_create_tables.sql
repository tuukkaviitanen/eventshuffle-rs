CREATE TABLE "Event" (
    "id" CHAR(36) NOT NULL,
    "name" VARCHAR(255) NOT NULL,

    CONSTRAINT "Event_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "EventDate" (
    "id" CHAR(36) NOT NULL,
    "date" DATE NOT NULL,
    "eventId" CHAR(36) NOT NULL,

    CONSTRAINT "EventDate_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "EventDateVote" (
    "id" CHAR(36) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "eventDateId" CHAR(36) NOT NULL,

    CONSTRAINT "EventDateVote_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "EventDate" ADD CONSTRAINT "EventDate_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event"("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "EventDateVote" ADD CONSTRAINT "EventDateVote_eventDateId_fkey" FOREIGN KEY ("eventDateId") REFERENCES "EventDate"("id") ON DELETE CASCADE ON UPDATE CASCADE;
