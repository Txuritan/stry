SELECT COUNT(SC.StoryId) as Count FROM StoryCharacter SC LEFT JOIN Story S ON S.Id = SC.StoryId WHERE SC.CharacterId = ?;
