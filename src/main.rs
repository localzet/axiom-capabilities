use anyhow::{bail, Result};
use std::env;
#[derive(Debug, Clone)]
struct Cap {
    subject: String,
    resource: String,
    actions: Vec<String>,
    epoch: u64,
    depth: u32,
}
fn main() -> Result<()> {
    let mut a = env::args().skip(1);
    match a.next().as_deref() {
        Some("demo") => demo(),
        _ => bail!("usage: axiom-capabilities demo"),
    }
}
fn demo() -> Result<()> {
    let root = Cap {
        subject: "module:kv".into(),
        resource: "namespace:alpha".into(),
        actions: vec!["read".into(), "write".into()],
        epoch: 7,
        depth: 0,
    };
    let child = attenuate(&root, "read")?;
    println!("root  = {}", encode(&root));
    println!("child = {}", encode(&child));
    println!(
        "check read  -> {}",
        check(&child, "module:kv", "namespace:alpha", "read", 7)
    );
    println!(
        "check write -> {}",
        check(&child, "module:kv", "namespace:alpha", "write", 7)
    );
    Ok(())
}
fn attenuate(parent: &Cap, action: &str) -> Result<Cap> {
    if !parent.actions.iter().any(|a| a == action) {
        bail!("parent does not grant {action}");
    }
    Ok(Cap {
        subject: parent.subject.clone(),
        resource: parent.resource.clone(),
        actions: vec![action.into()],
        epoch: parent.epoch,
        depth: parent.depth + 1,
    })
}
fn check(c: &Cap, subject: &str, resource: &str, action: &str, epoch: u64) -> bool {
    c.subject == subject
        && c.resource == resource
        && c.epoch == epoch
        && c.actions.iter().any(|a| a == action)
}
fn encode(c: &Cap) -> String {
    format!(
        "subject={};resource={};actions={};epoch={};depth={}",
        c.subject,
        c.resource,
        c.actions.join(","),
        c.epoch,
        c.depth
    )
}
