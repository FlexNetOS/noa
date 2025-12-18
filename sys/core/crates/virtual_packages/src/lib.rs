use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VirtualPackageInfo {
    pub name: String,
    pub version: String,
    pub build: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VirtualPackageReport {
    pub packages: Vec<VirtualPackageInfo>,
}

pub fn detect_virtual_packages() -> anyhow::Result<VirtualPackageReport> {
    use rattler_virtual_packages::VirtualPackage;

    let pkgs = VirtualPackage::current()?;

    let mut out: Vec<VirtualPackageInfo> = Vec::with_capacity(pkgs.len());

    for p in pkgs.iter() {
        match p {
            VirtualPackage::Win => out.push(VirtualPackageInfo {
                name: "__win".into(),
                version: "0".into(),
                build: "0".into(),
            }),
            VirtualPackage::Unix => out.push(VirtualPackageInfo {
                name: "__unix".into(),
                version: "0".into(),
                build: "0".into(),
            }),
            VirtualPackage::Linux(linux) => out.push(VirtualPackageInfo {
                name: "__linux".into(),
                version: linux.version.to_string(),
                build: "0".into(),
            }),
            VirtualPackage::Osx(osx) => out.push(VirtualPackageInfo {
                name: "__osx".into(),
                version: osx.version.to_string(),
                build: "0".into(),
            }),
            VirtualPackage::LibC(libc) => out.push(VirtualPackageInfo {
                name: "__glibc".into(),
                version: libc.version.to_string(),
                build: libc.family.clone(),
            }),
            VirtualPackage::Cuda(cuda) => out.push(VirtualPackageInfo {
                name: "__cuda".into(),
                version: cuda.version.to_string(),
                build: "0".into(),
            }),
            VirtualPackage::Archspec(spec) => out.push(VirtualPackageInfo {
                name: "__archspec".into(),
                version: "1".into(),
                build: format!("{:?}", spec),
            }),
        }
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(VirtualPackageReport { packages: out })
}

pub fn detect_virtual_packages_json() -> anyhow::Result<String> {
    let report = detect_virtual_packages()?;
    Ok(serde_json::to_string_pretty(&report)?)
}
