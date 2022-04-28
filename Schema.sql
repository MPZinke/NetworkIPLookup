


-- SUMMARY:  Types of devices.
CREATE TABLE "DeviceType"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL
);


-- SUMMARY:  List of Networks that are tracked.
CREATE TABLE "Network"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"gateway" INET NOT NULL,
	"netmask" INET NOT NULL
);


-- SUMMARY:  IP Addresses for a Network.
-- RELATION: <IP>:<Network> N:1.
CREATE TABLE "IP"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"address" INET NOT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"is_reservation" BOOL NOT NULL DEFAULT FALSE,
	"is_static" BOOL NOT NULL DEFAULT TRUE,
	"DeviceType.id" INT NOT NULL DEFAULT 1,
	FOREIGN KEY ("DeviceType.id") REFERENCES "DeviceType"("id"),
	"Network.id" INT NOT NULL,
	FOREIGN KEY ("Network.id") REFERENCES "Network"("id")
);


-- SUMMARY:  IP Address Ranges for a Network.
-- RELATION: <IPRange>:<Network> N:1.
CREATE TABLE "IPRange"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"start_address" INET NOT NULL,
	"end_address" INET NOT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"is_reservation" BOOL NOT NULL DEFAULT FALSE,
	"Network.id" INT NOT NULL,
	FOREIGN KEY ("Network.id") REFERENCES "Network"("id")
);


-- SUMMARY:  Services that runs on the device for an IP.
-- RELATION: <Service>:<IP> N:1.
CREATE TABLE "Service"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"port" SMALLINT NOT NULL DEFAULT 80,
	"IP.id" INT NOT NULL,
	FOREIGN KEY ("IP.id") REFERENCES "IP"("id")
);


