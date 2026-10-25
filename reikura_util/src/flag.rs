use crate::collections::BitSet;

pub struct Flag<B: AsRef<[u8]> + AsMut<[u8]> = Vec<u8>> {
    inner: BitSet<B>,
    pub groups: [Option<Vec<FlagGroup>>; 100],
}

pub enum GroupMatch {
    All,
    None,
    NotAll,
    Any,
}

impl Flag {
    pub fn new(count: usize) -> Self {
        Self {
            inner: BitSet::new(count),
            groups: std::array::from_fn(|_| None),
        }
    }

    pub fn check_group(&self, group_index: usize, group_match: GroupMatch) -> bool {
        if group_index >= self.groups.len() {
            return false;
        }

        let Some(groups) = self.groups[group_index].as_ref() else {
            return false;
        };

        let flag_buffer = self.inner.as_slice();

        match group_match {
            GroupMatch::All => {
                for group in groups {
                    for (i, &bitmask) in group.bitmasks.iter().enumerate() {
                        let byte = flag_buffer.get(group.byte_pos + i).copied().unwrap_or(0);

                        if (byte & bitmask) != bitmask {
                            return false;
                        }
                    }
                }

                true
            }
            GroupMatch::None => {
                for group in groups {
                    for (i, &bitmask) in group.bitmasks.iter().enumerate() {
                        let byte = flag_buffer.get(group.byte_pos + i).copied().unwrap_or(0);

                        if (byte & bitmask) != 0 {
                            return false;
                        }
                    }
                }

                true
            }
            GroupMatch::NotAll => {
                for group in groups {
                    for (i, &bitmask) in group.bitmasks.iter().enumerate() {
                        let byte = flag_buffer.get(group.byte_pos + i).copied().unwrap_or(0);

                        if (byte & bitmask) != 0 {
                            return true;
                        }
                    }
                }

                false
            }
            GroupMatch::Any => {
                for group in groups {
                    for (i, &bitmask) in group.bitmasks.iter().enumerate() {
                        let byte = flag_buffer.get(group.byte_pos + i).copied().unwrap_or(0);

                        if (byte & bitmask) != bitmask {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Flag<B> {
    pub fn from_raw(bytes: B, count: usize) -> Self {
        Self {
            inner: BitSet::from_raw(bytes, count),
            groups: [const { None }; 100],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get(&self, index: usize) -> bool {
        self.inner.get(index).unwrap_or(false)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.inner.set(index, value);
    }

    pub fn toggle(&mut self, index: usize) {
        self.inner.toggle(index);
    }
}

pub struct FlagGroup {
    pub byte_pos: usize,
    pub bitmasks: Vec<u8>,
}

impl FlagGroup {
    pub fn new(byte_pos: u16, bitmasks: Vec<u8>) -> Self {
        Self {
            byte_pos: byte_pos as usize,
            bitmasks,
        }
    }
}
