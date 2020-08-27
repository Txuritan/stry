-- ============================================================================
-- Authors
-- ============================================================================
INSERT INTO Author(Id, Name, Created, Updated) VALUES
    ('ZqYCf8', 'author 1', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('zsGEjQ', 'author 2', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('WbWWRz', 'author 3', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Chapters
-- ============================================================================
INSERT INTO Chapter(Id, Name, Pre, Main, Post, Words, Created, Updated) VALUES
    ('mg8sfV', 'chapter 1', '', 'some sample text', '', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('ypgKDW', 'chapter 2', '', 'some sample text', '', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('jC6K6M', 'chapter 1', '', 'some sample text', '', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('NA4Lgt', 'chapter 2', '', 'some sample text', '', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('6hnYRR', 'chapter 3', '', 'some sample text', '', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Characters
-- ============================================================================
INSERT INTO Character(Id, Name, Created, Updated) VALUES
    ('2crUDM', 'character 1', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('9Tb66w', 'character 2', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('iV5yY4', 'character 3', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SqWCU9', 'character 4', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Origins
-- ============================================================================
INSERT INTO Origin(Id, Name, Created, Updated) VALUES
    ('Nb4ynY', 'origin 1', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('J2Ej2P', 'origin 2', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Pairings
-- ============================================================================
INSERT INTO Pairing(Id, Hash, Platonic, Created, Updated) VALUES
    ('FLR49G', '2crUDM,9Tb66w', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SeUBQq', '2crUDM,iV5yY4', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('3TETzP', '2crUDM,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('qyaBa4', '9Tb66w,iV5yY4', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('CQi6Q9', '9Tb66w,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('mhneE3', 'iV5yY4,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('tBRhQm', '2crUDM,9Tb66w,iV5yY4', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SJGHc6', '2crUDM,9Tb66w,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('wwfEzC', '2crUDM,iV5yY4,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('Tk3Lq7', '9Tb66w,iV5yY4,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('rg3DeQ', '2crUDM,9Tb66w,iV5yY4,SqWCU9', false, '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 2 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('FLR49G', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('FLR49G', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 3 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('SeUBQq', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SeUBQq', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('3TETzP', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('3TETzP', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 2, character 3 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('qyaBa4', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('qyaBa4', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 2, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('CQi6Q9', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('CQi6Q9', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 3, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('mhneE3', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('mhneE3', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 2, character 3 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('tBRhQm', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('tBRhQm', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('tBRhQm', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 2, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('SJGHc6', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SJGHc6', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('SJGHc6', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 3, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('wwfEzC', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('wwfEzC', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('wwfEzC', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 2, character 3, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('Tk3Lq7', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('Tk3Lq7', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('Tk3Lq7', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

-- [ character 1, character 2, character 3, character 4 ]
INSERT INTO PairingCharacter(PairingId, CharacterId, Created, Updated) VALUES
    ('rg3DeQ', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('rg3DeQ', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('rg3DeQ', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('rg3DeQ', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Tags
-- ============================================================================
INSERT INTO Tag(Id, Name, Created, Updated) VALUES
    ('V3VEAR', 'tag 1', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('fMNi7A', 'tag 2', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('A38isy', 'tag 3', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('7TnYys', 'tag 4', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Warnings
-- ============================================================================
INSERT INTO Warning(Id, Name, Created, Updated) VALUES
    ('brVRkN', 'warning 1', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('3d72n5', 'warning 2', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('C7bcyL', 'warning 3', '2020-06-08 07:22:03', '2020-06-08 07:22:03');


-- ============================================================================
-- Stories
-- ============================================================================
INSERT INTO Story(Id, Name, Summary, Rating, State, Created, Updated) VALUES
    ('pS8LfM', 'story 1', 'example story', 'teen', 'in-progress', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'story 2', 'example story', 'mature', 'complete', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryAuthor(StoryId, AuthorId, Created, Updated) VALUES
    ('pS8LfM', 'ZqYCf8', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryAuthor(StoryId, AuthorId, Created, Updated) VALUES
    ('GQb4TP', 'zsGEjQ', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'WbWWRz', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryChapter(StoryId, ChapterId, Place, Created, Updated) VALUES
    ('pS8LfM', 'mg8sfV', 1, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'ypgKDW', 2, '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryChapter(StoryId, ChapterId, Place, Created, Updated) VALUES
    ('GQb4TP', 'jC6K6M', 1, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'NA4Lgt', 2, '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', '6hnYRR', 3, '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryCharacter(StoryId, CharacterId, Created, Updated) VALUES
    ('pS8LfM', '2crUDM', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', '9Tb66w', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'iV5yY4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'SqWCU9', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryOrigin(StoryId, OriginId, Created, Updated) VALUES
    ('pS8LfM', 'Nb4ynY', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryOrigin(StoryId, OriginId, Created, Updated) VALUES
    ('GQb4TP', 'J2Ej2P', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryPairing(StoryId, PairingId, Created, Updated) VALUES
    ('pS8LfM', 'FLR49G', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'SeUBQq', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', '3TETzP', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryPairing(StoryId, PairingId, Created, Updated) VALUES
    ('GQb4TP', 'qyaBa4', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'tBRhQm', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'Tk3Lq7', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'rg3DeQ', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryTag(StoryId, TagId, Created, Updated) VALUES
    ('pS8LfM', 'V3VEAR', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'fMNi7A', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('pS8LfM', 'A38isy', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryTag(StoryId, TagId, Created, Updated) VALUES
    ('GQb4TP', 'A38isy', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', '7TnYys', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryWarning(StoryId, WarningId, Created, Updated) VALUES
    ('pS8LfM', 'brVRkN', '2020-06-08 07:22:03', '2020-06-08 07:22:03');

INSERT INTO StoryWarning(StoryId, WarningId, Created, Updated) VALUES
    ('GQb4TP', '3d72n5', '2020-06-08 07:22:03', '2020-06-08 07:22:03'),
    ('GQb4TP', 'C7bcyL', '2020-06-08 07:22:03', '2020-06-08 07:22:03');
