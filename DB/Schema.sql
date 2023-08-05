
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


-- SUMMARY:  List of Networks that are tracked.
DROP TABLE IF EXISTS "Network" CASCADE;
CREATE TABLE "Network"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"auth_value" VARCHAR(128) DEFAULT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '' UNIQUE,
	"gateway" VARCHAR(15) NOT NULL,
	"netmask" VARCHAR(15) NOT NULL
);


CREATE TYPE BAND AS ENUM ('2.4GHz', '5GHz', 'Ethernet');


-- SUMMARY:  Device Addresses for a Network.
-- RELATION: <Device>:<Network> N:1.
DROP TABLE IF EXISTS "Device" CASCADE;
CREATE TABLE "Device"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"address" VARCHAR(15) DEFAULT NULL,
	"band" BAND DEFAULT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"is_reservation" BOOL NOT NULL DEFAULT FALSE,
	"is_static" BOOL NOT NULL DEFAULT FALSE,
	"mac" CHAR(17) DEFAULT NULL,
	"Network.id" INT NOT NULL,
	FOREIGN KEY ("Network.id") REFERENCES "Network"("id"),
	UNIQUE("label", "Network.id")
);


CREATE UNIQUE INDEX ON "Device"("address", "Network.id")
  WHERE "address" IS NOT NULL;


CREATE UNIQUE INDEX ON "Device"("mac", "Network.id")
  WHERE "mac" IS NOT NULL;


-- SUMMARY:  Services that runs on the device for a device.
-- RELATION: <Service>:<Device> N:1.
DROP TABLE IF EXISTS "Service" CASCADE;
CREATE TABLE "Service"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"port" SMALLINT NOT NULL DEFAULT 80,
	"auth_value" TEXT DEFAULT NULL,
	"Device.id" INT NOT NULL,
	FOREIGN KEY ("Device.id") REFERENCES "Device"("id"),
	UNIQUE("label", "Device.id")
);


-- SUMMARY:  Types of devices.
-- RELATION: <Group>:<Device> N:M.
-- REQUIRED VALUES: ['Other', 'Mixed']
DROP TABLE IF EXISTS "Group" CASCADE;
CREATE TABLE "Group"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL UNIQUE
);


-- SUMMARY:  Associates Groups with Devices.
-- RELATION: <Group>:<Device> N:M.
DROP TABLE IF EXISTS "Group-Device" CASCADE;
CREATE TABLE "Group-Device"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"Group.id" INT NOT NULL DEFAULT 1,
	FOREIGN KEY ("Group.id") REFERENCES "Group"("id"),
	"Device.id" INT NOT NULL DEFAULT 1,
	FOREIGN KEY ("Device.id") REFERENCES "Device"("id"),
	UNIQUE("Group.id", "Device.id")
);


-- SUMMARY:  Device Addresses for a Network.
-- RELATION: <Device>:<Network> N:1.
DROP TABLE IF EXISTS "OtherDevice" CASCADE;
CREATE TABLE "OtherDevice"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"address" VARCHAR(15) DEFAULT NULL,
	"band" BAND DEFAULT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"mac" CHAR(17) DEFAULT NULL,
);
