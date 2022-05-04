
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
CREATE TABLE "Network"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '' UNIQUE,
	"gateway" VARCHAR(15) NOT NULL,
	"netmask" VARCHAR(15) NOT NULL
);


-- SUMMARY:  IP Addresses for a Network.
-- RELATION: <IP>:<Network> N:1.
CREATE TABLE "IP"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"address" VARCHAR(15) NOT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"is_reservation" BOOL NOT NULL DEFAULT FALSE,
	"is_static" BOOL NOT NULL DEFAULT TRUE,
	"mac" CHAR(16) DEFAULT NULL,
	"Network.id" INT NOT NULL,
	FOREIGN KEY ("Network.id") REFERENCES "Network"("id"),
	UNIQUE("address", "Network.id"),
	UNIQUE("label", "Network.id")
);


-- SUMMARY:  Services that runs on the device for an IP.
-- RELATION: <Service>:<IP> N:1.
CREATE TABLE "Service"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"port" SMALLINT NOT NULL DEFAULT 80,
	"IP.id" INT NOT NULL,
	FOREIGN KEY ("IP.id") REFERENCES "IP"("id"),
	UNIQUE("label", "IP.id")
);


-- SUMMARY:  Types of devices.
-- RELATION: <Group>:<IP> N:M.
-- REQUIRED VALUES: ['Other', 'Mixed']
CREATE TABLE "Group"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL UNIQUE
);


-- SUMMARY:  Associates Groups with IPs.
-- RELATION: <Group>:<IP> N:M.
CREATE TABLE "Group-IP"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"Group.id" INT NOT NULL DEFAULT 1,
	FOREIGN KEY ("Group.id") REFERENCES "Group"("id"),
	"IP.id" INT NOT NULL DEFAULT 1,
	FOREIGN KEY ("IP.id") REFERENCES "IP"("id"),
	UNIQUE("Group.id", "IP.id")
);

