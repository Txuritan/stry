SELECT
    S.Id, S.Name, S.Created, S.Updated,
    S.Summary, S.Rating, S.State, (SELECT COUNT(SC.StoryId) as Count FROM StoryChapter SC WHERE SC.StoryId = S.Id) AS Chapters, (SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = S.Id) AS Words,
    'story' AS Type
FROM Story S
WHERE S.Id = ?

UNION

SELECT
    A.Id, A.Name, A.Created, A.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State, NULL AS Chapters, NULL AS Words,
    'author' AS Type
FROM Author A
LEFT JOIN StoryAuthor SA ON SA.AuthorId = A.Id
WHERE SA.StoryId = ?

UNION

SELECT
    O.Id, O.Name, O.Created, O.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State, NULL AS Chapters, NULL AS Words,
    'origin' AS Type
FROM Origin O
LEFT JOIN StoryOrigin SO ON SO.OriginId = O.Id
WHERE SO.StoryId = ?

UNION

SELECT
    W.Id, W.Name, W.Created, W.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State, NULL AS Chapters, NULL AS Words,
    'warning' AS Type
FROM Warning W
LEFT JOIN StoryWarning SW ON SW.WarningId = W.Id
WHERE SW.StoryId = ?

UNION

SELECT
    C.Id, C.Name, C.Created, C.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State, NULL AS Chapters, NULL AS Words,
    'character' AS Type
FROM Character C
LEFT JOIN StoryCharacter SC ON SC.CharacterId = C.Id
WHERE SC.StoryId = ?

UNION

SELECT
    T.Id, T.Name, T.Created, T.Updated,
    NULL AS Summary, NULL AS Rating, NULL AS State, NULL AS Chapters, NULL AS Words,
    'tag' AS Type
FROM Tag T
LEFT JOIN StoryTag ST ON ST.TagId = T.Id
WHERE ST.StoryId = ?;
