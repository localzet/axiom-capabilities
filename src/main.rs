use anyhow::{bail, Result};
use sha2::{Digest, Sha256};
use std::env;

#[derive(Debug, Clone)]
struct Capability {
    issuer: String,
    subject: String,
    resource_prefix: String,
    actions: Vec<String>,
    not_before: u64,
    expires: u64,
    depth: u32,
    parent: String,
    nonce: u64,
}

fn main() -> Result<()> {
    if env::args().nth(1).as_deref() != Some("demo") {
        bail!("usage: axiom-capabilities demo");
    }
    demo()
}

fn demo() -> Result<()> {
    let root = Capability {
        issuer: "root:owner".into(),
        subject: "module:kv".into(),
        resource_prefix: "namespace:alpha/".into(),
        actions: vec!["read".into(), "write".into()],
        not_before: 10,
        expires: 100,
        depth: 0,
        parent: "GENESIS".into(),
        nonce: 41,
    };
    let child = attenuate(&root, "read", "namespace:alpha/public/", 80, 42)?;
    let child_id = identifier(&child);

    println!("root.id={}", identifier(&root));
    println!("child.id={child_id}");
    println!(
        "read public -> {}",
        permits(
            &child,
            "module:kv",
            "namespace:alpha/public/item",
            "read",
            50
        )
    );
    println!(
        "write public -> {}",
        permits(
            &child,
            "module:kv",
            "namespace:alpha/public/item",
            "write",
            50
        )
    );
    println!(
        "read private -> {}",
        permits(
            &child,
            "module:kv",
            "namespace:alpha/private/item",
            "read",
            50
        )
    );
    Ok(())
}

fn attenuate(
    parent: &Capability,
    action: &str,
    resource_prefix: &str,
    expires: u64,
    nonce: u64,
) -> Result<Capability> {
    if !parent.actions.iter().any(|granted| granted == action) {
        bail!("parent does not grant action {action}");
    }
    if !resource_prefix.starts_with(&parent.resource_prefix) {
        bail!("child resource would widen parent authority");
    }
    if expires > parent.expires {
        bail!("child expiry would outlive parent");
    }
    Ok(Capability {
        issuer: parent.subject.clone(),
        subject: parent.subject.clone(),
        resource_prefix: resource_prefix.to_owned(),
        actions: vec![action.to_owned()],
        not_before: parent.not_before,
        expires,
        depth: parent.depth + 1,
        parent: identifier(parent),
        nonce,
    })
}

fn permits(
    capability: &Capability,
    subject: &str,
    resource: &str,
    action: &str,
    epoch: u64,
) -> bool {
    capability.subject == subject
        && resource.starts_with(&capability.resource_prefix)
        && capability.actions.iter().any(|granted| granted == action)
        && epoch >= capability.not_before
        && epoch <= capability.expires
}

fn identifier(capability: &Capability) -> String {
    let canonical = format!(
        "issuer={};subject={};resource={};actions={};not_before={};expires={};depth={};parent={};nonce={}",
        capability.issuer,
        capability.subject,
        capability.resource_prefix,
        capability.actions.join(","),
        capability.not_before,
        capability.expires,
        capability.depth,
        capability.parent,
        capability.nonce
    );
    Sha256::digest(canonical.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
