SELECT COUNT(SO.StoryId) as Id FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ?;
