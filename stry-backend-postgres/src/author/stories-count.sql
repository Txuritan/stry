SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1;
