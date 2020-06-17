SELECT SW.StoryId FROM StoryWarning SW LEFT JOIN Story S ON S.Id = SW.StoryId WHERE SW.WarningId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;
