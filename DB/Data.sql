
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
	('192.168.1.2'::INET, 'Resevered-2', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.3'::INET, 'Resevered-3', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.4'::INET, 'Resevered-4', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.5'::INET, 'Resevered-5', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.6'::INET, 'Resevered-6', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.7'::INET, 'Resevered-7', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.8'::INET, 'Resevered-8', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.9'::INET, 'Resevered-9', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.10'::INET, 'Resevered-10', FALSE, TRUE, NULL::MACADDR),
	-- Livingroom
	('192.168.1.11'::INET, 'Resevered-11', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.12'::INET, 'Resevered-12', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.13'::INET, 'Resevered-13', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.14'::INET, 'Resevered-14', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.15'::INET, 'Resevered-15', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.16'::INET, 'Resevered-16', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.17'::INET, 'Resevered-17', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.18'::INET, 'Resevered-18', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.19'::INET, 'Resevered-19', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.20'::INET, 'Resevered-20', FALSE, TRUE, NULL::MACADDR),
	-- Bedroom
	('192.168.1.21'::INET, 'Resevered-21', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.22'::INET, 'Resevered-22', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.23'::INET, 'Resevered-23', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.24'::INET, 'Resevered-24', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.25'::INET, 'Resevered-25', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.26'::INET, 'Resevered-26', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.27'::INET, 'Resevered-27', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.28'::INET, 'Resevered-28', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.29'::INET, 'Resevered-29', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.30'::INET, 'Resevered-30', FALSE, TRUE, NULL::MACADDR),
	-- Kitchen
	('192.168.1.31'::INET, 'Resevered-31', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.32'::INET, 'Resevered-32', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.33'::INET, 'Resevered-33', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.34'::INET, 'Resevered-34', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.35'::INET, 'Resevered-35', FALSE, TRUE, NULL::MACADDR),
	-- Computers
	('192.168.1.36'::INET, 'Resevered-36', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.37'::INET, 'Resevered-37', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.38'::INET, 'Resevered-38', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.39'::INET, 'Resevered-39', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.40'::INET, 'Resevered-40', FALSE, TRUE, NULL::MACADDR),
	-- Mobile
	('192.168.1.41'::INET, 'Resevered-41', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.42'::INET, 'Resevered-42', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.43'::INET, 'Resevered-43', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.44'::INET, 'Resevered-44', FALSE, TRUE, NULL::MACADDR),
	('192.168.1.45'::INET, 'Resevered-45', FALSE, TRUE, NULL::MACADDR)
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
WHERE '192.168.1.2'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.11'::INET;


-- Bedroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Livingroom'
WHERE '192.168.1.11'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.21'::INET;


-- Livingroom
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Bedroom'
WHERE '192.168.1.21'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.31'::INET;


-- Kitchen
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Kitchen'
WHERE '192.168.1.31'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.36'::INET;


-- ———————————————————————————————————————————————— GROUPS::COMPUTER ———————————————————————————————————————————————— --

-- Computers
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Computer'
WHERE '192.168.1.36'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.41'::INET;


-- ———————————————————————————————————————————————— GROUPS::DEVICES  ———————————————————————————————————————————————— --

-- Mobile
INSERT INTO "Group-IP" ("Group.id", "IP.id")
SELECT "Group"."id", "IP"."id" FROM "IP"
JOIN "Group" ON "Group"."label" = 'Mobile'
WHERE '192.168.1.41'::INET <= "IP"."address"
AND "IP"."address" < '192.168.1.46'::INET;

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
