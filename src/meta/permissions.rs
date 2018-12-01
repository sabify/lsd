use super::Type;
use ansi_term::{ANSIString, Colour};
use color::{Colors, Elem};
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
pub struct Permissions {
    pub file_type: Type,

    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,

    pub sticky: bool,
    pub setgid: bool,
    pub setuid: bool,
}

impl<'a> From<&'a Metadata> for Permissions {
    fn from(meta: &Metadata) -> Self {
        let bits = meta.permissions().mode();
        let has_bit = |bit| bits & bit == bit;

        Permissions {
            file_type: Type::from(meta),

            user_read: has_bit(modes::USER_READ),
            user_write: has_bit(modes::USER_WRITE),
            user_execute: has_bit(modes::USER_EXECUTE),

            group_read: has_bit(modes::GROUP_READ),
            group_write: has_bit(modes::GROUP_WRITE),
            group_execute: has_bit(modes::GROUP_EXECUTE),

            other_read: has_bit(modes::OTHER_READ),
            other_write: has_bit(modes::OTHER_WRITE),
            other_execute: has_bit(modes::OTHER_EXECUTE),

            sticky: has_bit(modes::STICKY),
            setgid: has_bit(modes::SETGID),
            setuid: has_bit(modes::SETUID),
        }
    }
}

impl Permissions {
    pub fn render(&self) -> String {
        let mut res = String::with_capacity(11);

        match self.file_type {
            Type::File => res += &Colors[&Elem::File].paint("."),
            Type::Directory => res += &Colors[&Elem::Dir].paint("d"),
            Type::SymLink(_) => res += &Colors[&Elem::SymLink].paint("l"),
        }

        let bit = |bit, chr: &'static str, color: Colour| {
            if bit {
                color.paint(chr).to_string()
            } else {
                Colors[&Elem::NoAccess].paint("-").to_string()
            }
        };

        res += &bit(self.user_read, "r", Colors[&Elem::Read]);
        res += &bit(self.user_write, "w", Colors[&Elem::Write]);
        res += &self.execute_bit(self.setuid).to_string();
        res += &bit(self.group_read, "r", Colors[&Elem::Read]);
        res += &bit(self.group_write, "w", Colors[&Elem::Write]);
        res += &self.execute_bit(self.setgid).to_string();
        res += &bit(self.other_read, "r", Colors[&Elem::Read]);
        res += &bit(self.other_write, "w", Colors[&Elem::Write]);
        res += &self.other_execute_bit().to_string();

        res
    }

    fn execute_bit(&self, special: bool) -> ANSIString<'static> {
        match (self.user_execute, special) {
            (false, false) => Colors[&Elem::NoAccess].paint("-"),
            (true, false) => Colors[&Elem::Exec].paint("x"),
            (false, true) => Colors[&Elem::ExecSticky].paint("S"),
            (true, true) => Colors[&Elem::ExecSticky].paint("s"),
        }
    }

    fn other_execute_bit(&self) -> ANSIString<'static> {
        match (self.other_execute, self.sticky) {
            (false, false) => Colors[&Elem::NoAccess].paint("-"),
            (true, false) => Colors[&Elem::Exec].paint("x"),
            (false, true) => Colors[&Elem::ExecSticky].paint("T"),
            (true, true) => Colors[&Elem::ExecSticky].paint("t"),
        }
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
mod modes {
    use libc;

    pub type Mode = u32;
    // The `libc::mode_t` type’s actual type varies, but the value returned
    // from `metadata.permissions().mode()` is always `u32`.

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const USER_WRITE: Mode = libc::S_IWUSR as Mode;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;

    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const GROUP_WRITE: Mode = libc::S_IWGRP as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;

    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
    pub const OTHER_WRITE: Mode = libc::S_IWOTH as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const STICKY: Mode = libc::S_ISVTX as Mode;
    pub const SETGID: Mode = libc::S_ISGID as Mode;
    pub const SETUID: Mode = libc::S_ISUID as Mode;
}