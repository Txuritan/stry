SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $3;
