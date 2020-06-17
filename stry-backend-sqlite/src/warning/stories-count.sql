SELECT COUNT(SW.StoryId) as Count FROM StoryWarning SW LEFT JOIN Story S ON S.Id = SW.StoryId WHERE SW.WarningId = ?;
