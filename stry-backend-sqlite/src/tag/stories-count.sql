SELECT COUNT(ST.StoryId) as Count FROM StoryTag ST LEFT JOIN Story S ON S.Id = ST.StoryId WHERE ST.TagId = ?;
