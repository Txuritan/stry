SELECT SC.StoryId FROM StoryCharacter SC LEFT JOIN Story S ON S.Id = SC.StoryId WHERE SC.CharacterId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;