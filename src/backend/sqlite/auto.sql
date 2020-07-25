SELECT Id, Name, Created, Updated, (SELECT COUNT(1) FROM StoryCharacter WHERE CharacterId = Id) AS Count, 'character' AS Type FROM Character
    WHERE Name LIKE ? COLLATE NOCASE
UNION ALL
SELECT Id, Name, Created, Updated, (SELECT COUNT(1) FROM StoryOrigin WHERE OriginId = Id) AS Count, 'origin' AS Type FROM Origin
    WHERE Name LIKE ? COLLATE NOCASE
UNION ALL
SELECT P.Id, (SELECT GROUP_CONCAT(C.Name, CASE WHEN P.Platonic THEN '+' ELSE '/' END) FROM PairingCharacter PC LEFT JOIN Character C ON C.Id = PC.CharacterId WHERE PC.PairingId = P.Id) AS Name, P.Created, P.Updated, (SELECT COUNT(1) FROM StoryPairing WHERE PairingId = P.Id) AS Count, 'pairing' AS Type FROM Pairing P
    WHERE Name LIKE ? COLLATE NOCASE
UNION ALL
SELECT Id, Name, Created, Updated, (SELECT COUNT(1) FROM StoryTag WHERE TagId = Id) AS Count, 'tag' AS Type FROM Tag
    WHERE Name LIKE ? COLLATE NOCASE
ORDER BY Count DESC, Name ASC LIMIT 10;
