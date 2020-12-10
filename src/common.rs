use std::fmt::Display;

use crate::{error::Result, utils};

/// pod spec group
#[derive(Debug)]
pub struct PodSpecGroup(pub FilePodSpec, pub PodSpec);

// pod version
#[derive(Debug, Eq, PartialEq)]
pub struct PodVer {
    pub nums: [u32; 3], // 仅关注前三位数字(x.x.x)
}

/// pod spec
#[derive(Debug)]
pub struct PodSpec {
    pub name: String,          // pod spec name
    pub hex: String,           // pod spec name hex with prefix three
    vers: Option<Vec<PodVer>>, // all version, order desc(big -> small)
}

/// Podfile.lock的pod spec
#[derive(Debug)]
pub struct FilePodSpec {
    pub name: String,                  // pod spec name
    pub ver: PodVer,                   // version
    pub sub_spec: Option<Vec<String>>, // sub pod spec
}

impl PodVer {
    pub fn new(text: &str) -> PodVer {
        let mut vs = text.split(".").peekable();
        let vs: Vec<_> = (0..3)
            .into_iter()
            .map(|_| {
                if vs.peek().is_some() {
                    vs.next().unwrap_or("0")
                } else {
                    "0"
                }
            })
            .map(|s| str::parse(s).unwrap_or(0))
            .collect();

        PodVer {
            nums: [vs[0], vs[1], vs[2]],
        }
    }
}

impl PartialOrd for PodVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.nums.cmp(&other.nums))
    }
}

impl Ord for PodVer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nums.cmp(&other.nums)
    }
}

impl Display for PodVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}",
            self.nums[0], self.nums[1], self.nums[2]
        ))
    }
}

impl Into<String> for PodVer {
    fn into(self) -> String {
        self.nums
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl PodSpec {
    pub fn new(name: String) -> Result<PodSpec> {
        let hex = utils::name_hash(&name)?;

        Ok(PodSpec {
            name,
            hex,
            vers: None,
        })
    }

    pub fn get_vers(&self) -> &Option<Vec<PodVer>> {
        &self.vers
    }

    pub fn set_vers(&mut self, vers: Option<Vec<PodVer>>) {
        match vers {
            Some(mut vers) => {
                vers.sort();
                vers.reverse();
                self.vers = Some(vers);
            }
            None => self.vers = None,
        }
    }
}

impl FilePodSpec {
    pub fn new(name: String, ver: String, sub_spec: Option<Vec<String>>) -> FilePodSpec {
        FilePodSpec {
            name,
            ver: PodVer::new(&ver),
            sub_spec,
        }
    }
}
