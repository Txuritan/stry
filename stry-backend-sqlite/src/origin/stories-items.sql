SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;
