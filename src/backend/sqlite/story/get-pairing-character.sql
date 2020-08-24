SELECT C.Id, C.Name, C.Created, C.Updated FROM Pairing P LEFT JOIN PairingCharacter PC ON PC.PairingId = P.Id LEFT JOIN Character C ON PC.CharacterId = C.Id WHERE P.Id = ? ORDER BY C.Name ASC;
