SELECT P.Id, P.Platonic, P.Created, P.Updated FROM Pairing P ORDER BY (SELECT GROUP_CONCAT(C.Name, '/') FROM PairingCharacter PC LEFT JOIN Character C ON C.Id = PC.CharacterId WHERE PC.PairingId = P.Id) LIMIT ? OFFSET ?;
