use crate::index;

pub fn add_tag(id: u64, tag: &str) -> anyhow::Result<()> {
    let index = index::Index::open()?;
    let mut note = index.get(id)?;

    note.add_tag(tag);

    Ok(())
}
