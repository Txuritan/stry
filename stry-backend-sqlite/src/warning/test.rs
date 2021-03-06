use {
    crate::test_utils::setup,
    stry_models::{story::StoryBuilder, List, Rating, State, Story, Warning},
    tokio::runtime::Runtime,
};

#[test]
pub fn get() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<Warning>> {
        let backend = setup()?;

        let warning = backend.get_warning("brVRkN".into()).await?;

        Ok(warning)
    }

    assert_eq!(
        Some(Warning::new_test("brVRkN", "warning 1")),
        rt.block_on(run())?
    );

    Ok(())
}

#[test]
pub fn get_all() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Warning>>> {
        let backend = setup()?;

        let warnings = backend.all_warnings(0, 10).await?;

        Ok(warnings)
    }

    assert_eq!(
        Some(List {
            total: 3,
            items: vec![
                Warning::new_test("brVRkN", "warning 1"),
                Warning::new_test("3d72n5", "warning 2"),
                Warning::new_test("C7bcyL", "warning 3"),
            ]
        }),
        rt.block_on(run())?,
    );

    Ok(())
}

#[test]
pub fn get_stories() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Story>>> {
        let backend = setup()?;

        let stories = backend.warning_stories("brVRkN".into(), 0, 10).await?;

        Ok(stories)
    }

    assert_eq!(
        Some(List {
            total: 1,
            items: vec![StoryBuilder::new(
                "pS8LfM",
                "story 1",
                "example story",
                Rating::Teen,
                State::InProgress,
                2,
                6,
            )
            .with_author("ZqYCf8", "author 1")
            .with_origin("Nb4ynY", "origin 1")
            .with_warning("brVRkN", "warning 1")
            .with_tag("V3VEAR", "tag 1")
            .with_tag("fMNi7A", "tag 2")
            .with_tag("A38isy", "tag 3")
            .with_character("2crUDM", "character 1")
            .with_character("9Tb66w", "character 2")
            .with_character("iV5yY4", "character 3")
            .with_character("SqWCU9", "character 4")
            .with_pairing("FLR49G", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("9Tb66w", "character 2")
            })
            .with_pairing("SeUBQq", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("iV5yY4", "character 3")
            })
            .with_pairing("3TETzP", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("SqWCU9", "character 4")
            })
            .finish()],
        }),
        rt.block_on(run())?,
    );

    Ok(())
}
