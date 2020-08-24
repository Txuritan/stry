SELECT
    Id, Name, Created, Updated,
    Summary, Rating, State,
    'story' AS Type
FROM Story
WHERE Id = ?

UNION

SELECT
    A.Id, A.Name, A.Created, A.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State,
    'author' AS Type
FROM Author A
LEFT JOIN StoryAuthor SA ON SA.AuthorId = A.Id
WHERE SA.StoryId = ?

UNION

SELECT
    O.Id, O.Name, O.Created, O.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State,
    'origin' AS Type
FROM Origin O
LEFT JOIN StoryOrigin SO ON SO.OriginId = O.Id
WHERE SO.StoryId = ?

UNION

SELECT
    W.Id, W.Name, W.Created, W.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State,
    'warning' AS Type
FROM Warning W
LEFT JOIN StoryWarning SW ON SW.WarningId = W.Id
WHERE SW.StoryId = ?

UNION

SELECT
    C.Id, C.Name, C.Created, C.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State,
    'character' AS Type
FROM Character C
LEFT JOIN StoryCharacter SC ON SC.CharacterId = C.Id
WHERE SC.StoryId = ?

UNION

SELECT
    T.Id, T.Name, T.Created, T.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State,
    'tag' AS Type
FROM Tag T
LEFT JOIN StoryTag ST ON ST.TagId = T.Id
WHERE ST.StoryId = ?;
