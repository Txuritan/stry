SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;
