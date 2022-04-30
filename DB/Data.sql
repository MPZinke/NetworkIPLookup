
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


INSERT INTO "Network" ("label", "gateway", "netmask") VALUES ('Home', '192.168.1.1', '255.255.255.0');



INSERT INTO "IP" ("address", "label", "is_reservation", "is_static", "mac", "Network.id")
SELECT "address", "label", "is_reservation", "is_static", "mac", "Network"."id"
FROM
(
	('192.168.1.2', '', FALSE, TRUE, NULL),
	('192.168.1.3', '', FALSE, TRUE, NULL),
	('192.168.1.4', '', FALSE, TRUE, NULL),
	('192.168.1.5', '', FALSE, TRUE, NULL),
	('192.168.1.6', '', FALSE, TRUE, NULL),
	('192.168.1.7', '', FALSE, TRUE, NULL),
	('192.168.1.8', '', FALSE, TRUE, NULL),
	('192.168.1.9', '', FALSE, TRUE, NULL),
	('192.168.1.10', '', FALSE, TRUE, NULL),
	('192.168.1.11', '', FALSE, TRUE, NULL),
	('192.168.1.12', '', FALSE, TRUE, NULL),
	('192.168.1.13', '', FALSE, TRUE, NULL),
	('192.168.1.14', '', FALSE, TRUE, NULL),
	('192.168.1.15', '', FALSE, TRUE, NULL),
	('192.168.1.16', '', FALSE, TRUE, NULL),
	('192.168.1.17', '', FALSE, TRUE, NULL),
	('192.168.1.18', '', FALSE, TRUE, NULL),
	('192.168.1.19', '', FALSE, TRUE, NULL),
	('192.168.1.20', '', FALSE, TRUE, NULL),
	('192.168.1.21', '', FALSE, TRUE, NULL),
	('192.168.1.22', '', FALSE, TRUE, NULL),
	('192.168.1.23', '', FALSE, TRUE, NULL),
	('192.168.1.24', '', FALSE, TRUE, NULL),
	('192.168.1.25', '', FALSE, TRUE, NULL),
	('192.168.1.26', '', FALSE, TRUE, NULL),
	('192.168.1.27', '', FALSE, TRUE, NULL),
	('192.168.1.28', '', FALSE, TRUE, NULL),
	('192.168.1.29', '', FALSE, TRUE, NULL),
	('192.168.1.30', '', FALSE, TRUE, NULL),
	('192.168.1.31', '', FALSE, TRUE, NULL),
	('192.168.1.32', '', FALSE, TRUE, NULL),
	('192.168.1.33', '', FALSE, TRUE, NULL),
	('192.168.1.34', '', FALSE, TRUE, NULL),
	('192.168.1.35', '', FALSE, TRUE, NULL),
	('192.168.1.36', '', FALSE, TRUE, NULL),
	('192.168.1.37', '', FALSE, TRUE, NULL),
	('192.168.1.38', '', FALSE, TRUE, NULL),
	('192.168.1.39', '', FALSE, TRUE, NULL),
	('192.168.1.40', '', FALSE, TRUE, NULL),
	('192.168.1.41', '', FALSE, TRUE, NULL),
	('192.168.1.42', '', FALSE, TRUE, NULL),
	('192.168.1.43', '', FALSE, TRUE, NULL),
	('192.168.1.44', '', FALSE, TRUE, NULL),
	('192.168.1.45', '', FALSE, TRUE, NULL),
	('192.168.1.46', '', FALSE, TRUE, NULL),
	('192.168.1.47', '', FALSE, TRUE, NULL),
	('192.168.1.48', '', FALSE, TRUE, NULL),
	('192.168.1.49', '', FALSE, TRUE, NULL),
	('192.168.1.50', '', FALSE, TRUE, NULL),
	('192.168.1.51', '', FALSE, TRUE, NULL),
	('192.168.1.52', '', FALSE, TRUE, NULL),
	('192.168.1.53', '', FALSE, TRUE, NULL),
	('192.168.1.54', '', FALSE, TRUE, NULL),
	('192.168.1.55', '', FALSE, TRUE, NULL),
	('192.168.1.56', '', FALSE, TRUE, NULL),
	('192.168.1.57', '', FALSE, TRUE, NULL),
	('192.168.1.58', '', FALSE, TRUE, NULL),
	('192.168.1.59', '', FALSE, TRUE, NULL),
	('192.168.1.60', '', FALSE, TRUE, NULL)
)
JOIN "Network" ON "Network"."label" = 'Home';


INSERT INTO "Group" ("label") VALUES
('House'),
('Bedroom'),
('Livingroom'),
('Kitchen'),
('Computers'),
('Mobile');


-- House
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'House'
WHERE "IP"."address" < '192.168.1.21';


-- Bedroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Bedroom'
WHERE "IP"."address" < '192.168.1.31'
AND '192.168.1.21' <= "IP"."address";


-- Livingroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Livingroom'
WHERE "IP"."address" < '192.168.1.41'
AND '192.168.1.31' <= "IP"."address";


-- Kitchen
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Kitchen'
WHERE "IP"."address" < '192.168.1.51'
AND '192.168.1.41' <= "IP"."address";


-- Computers
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Computers'
WHERE "IP"."address" < '192.168.1.61'
AND '192.168.1.51' <= "IP"."address";


-- Phones
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Mobile'
WHERE "IP"."address" < '192.168.1.71'
AND '192.168.1.61' <= "IP"."address";
