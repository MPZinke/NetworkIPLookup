
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
SELECT "Temp"."address", "Temp"."label", "Temp"."is_reservation", "Temp"."is_static", "Temp"."mac", "Network"."id"
FROM
(
	VALUES
	('192.168.1.2', 'Resevered-2', FALSE, TRUE, NULL),
	('192.168.1.3', 'Resevered-3', FALSE, TRUE, NULL),
	('192.168.1.4', 'Resevered-4', FALSE, TRUE, NULL),
	('192.168.1.5', 'Resevered-5', FALSE, TRUE, NULL),
	('192.168.1.6', 'Resevered-6', FALSE, TRUE, NULL),
	('192.168.1.7', 'Resevered-7', FALSE, TRUE, NULL),
	('192.168.1.8', 'Resevered-8', FALSE, TRUE, NULL),
	('192.168.1.9', 'Resevered-9', FALSE, TRUE, NULL),
	('192.168.1.10', 'Resevered-10', FALSE, TRUE, NULL),
	-- Livingroom
	('192.168.1.11', 'Resevered-11', FALSE, TRUE, NULL),
	('192.168.1.12', 'Resevered-12', FALSE, TRUE, NULL),
	('192.168.1.13', 'Resevered-13', FALSE, TRUE, NULL),
	('192.168.1.14', 'Resevered-14', FALSE, TRUE, NULL),
	('192.168.1.15', 'Resevered-15', FALSE, TRUE, NULL),
	('192.168.1.16', 'Resevered-16', FALSE, TRUE, NULL),
	('192.168.1.17', 'Resevered-17', FALSE, TRUE, NULL),
	('192.168.1.18', 'Resevered-18', FALSE, TRUE, NULL),
	('192.168.1.19', 'Resevered-19', FALSE, TRUE, NULL),
	('192.168.1.20', 'Resevered-20', FALSE, TRUE, NULL),
	-- Bedroom
	('192.168.1.21', 'Resevered-21', FALSE, TRUE, NULL),
	('192.168.1.22', 'Resevered-22', FALSE, TRUE, NULL),
	('192.168.1.23', 'Resevered-23', FALSE, TRUE, NULL),
	('192.168.1.24', 'Resevered-24', FALSE, TRUE, NULL),
	('192.168.1.25', 'Resevered-25', FALSE, TRUE, NULL),
	('192.168.1.26', 'Resevered-26', FALSE, TRUE, NULL),
	('192.168.1.27', 'Resevered-27', FALSE, TRUE, NULL),
	('192.168.1.28', 'Resevered-28', FALSE, TRUE, NULL),
	('192.168.1.29', 'Resevered-29', FALSE, TRUE, NULL),
	('192.168.1.30', 'Resevered-30', FALSE, TRUE, NULL),
	-- Kitchen
	('192.168.1.31', 'Resevered-31', FALSE, TRUE, NULL),
	('192.168.1.32', 'Resevered-32', FALSE, TRUE, NULL),
	('192.168.1.33', 'Resevered-33', FALSE, TRUE, NULL),
	('192.168.1.34', 'Resevered-34', FALSE, TRUE, NULL),
	('192.168.1.35', 'Resevered-35', FALSE, TRUE, NULL),
	-- Computers
	('192.168.1.36', 'Resevered-36', FALSE, TRUE, NULL),
	('192.168.1.37', 'Resevered-37', FALSE, TRUE, NULL),
	('192.168.1.38', 'Resevered-38', FALSE, TRUE, NULL),
	('192.168.1.39', 'Resevered-39', FALSE, TRUE, NULL),
	('192.168.1.40', 'Resevered-40', FALSE, TRUE, NULL),
	-- Mobile
	('192.168.1.41', 'Resevered-41', FALSE, TRUE, NULL),
	('192.168.1.42', 'Resevered-42', FALSE, TRUE, NULL),
	('192.168.1.43', 'Resevered-43', FALSE, TRUE, NULL),
	('192.168.1.44', 'Resevered-44', FALSE, TRUE, NULL),
	('192.168.1.45', 'Resevered-45', FALSE, TRUE, NULL)
) AS "Temp"("address", "label", "is_reservation", "is_static", "mac")
JOIN "Network" ON "Network"."label" = 'Home';


-- ————————————————————————————————————————————————————— GROUPS ————————————————————————————————————————————————————— --
-- —————————————————————————————————————————————————————————————————————————————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('House'),  -- 2–10
-- Whole rooms
('Livingroom'),  -- 11–20
('Bedroom'),  -- 21–30
-- Half rooms
('Kitchen'),  -- 31–35
-- Other
('Computer'),  -- 36–40
('Mobile');  -- 41–45


-- ————————————————————————————————————————————————— GROUPS::ROOMS  ————————————————————————————————————————————————— --

-- House
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'House'
WHERE '192.168.1.2' <= "IP"."address"
AND "IP"."address" < '192.168.1.11';


-- Bedroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Livingroom'
WHERE '192.168.1.11' <= "IP"."address"
AND "IP"."address" < '192.168.1.21';


-- Livingroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Bedroom'
WHERE '192.168.1.21' <= "IP"."address"
AND "IP"."address" < '192.168.1.31';


-- Kitchen
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Kitchen'
WHERE '192.168.1.31' <= "IP"."address"
AND "IP"."address" < '192.168.1.36';


-- ———————————————————————————————————————————————— GROUPS::COMPUTER ———————————————————————————————————————————————— --

-- Computers
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Computer'
WHERE '192.168.1.36' <= "IP"."address"
AND "IP"."address" < '192.168.1.41';


-- ———————————————————————————————————————————————— GROUPS::DEVICES  ———————————————————————————————————————————————— --

-- Mobile
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Mobile'
WHERE '192.168.1.41' <= "IP"."address"
AND "IP"."address" < '192.168.1.46';

-- ———————————————————————————————————————————— GROUPS::ENTERTAINMENT ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Entertainment');


INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Entertainment'
JOIN "Network" ON "IP"."Network.id" = "Network"."id"
WHERE ("IP"."label", "Network"."label") IN
(
	('Bedroom-TV', 'Home'),
	('Livingroom-TV', 'Home')
);


-- ———————————————————————————————————————————— GROUPS::SMART ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Smart');


INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Smart'
JOIN "Network" ON "IP"."Network.id" = "Network"."id"
WHERE ("IP"."label", "Network"."label") IN
(
	('Bedroom-TV', 'Home'),
	('Livingroom-TV', 'Home')
);


-- ———————————————————————————————————————————— GROUPS::CURTAIN ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Curtain');


INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Curtain'
JOIN "Network" ON "IP"."Network.id" = "Network"."id"
WHERE ("IP"."label", "Network"."label") IN
(
	('Bedroom-Curtain', 'Home'),
	('Livingroom-Curtain', 'Home'),
);
