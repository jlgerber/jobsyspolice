#![allow(dead_code)]

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Perms {
    Unix(i32),
    Windows,
}

impl Perms {
    pub fn user(&self) -> i32 {
        match self {
            Perms::Unix(v) => {
                USR_MODE & v
            },
            _ => 0,
        }
    }

    pub fn group(&self) -> i32 {
        match self {
            Perms::Unix(v) => {
                GRP_MODE & v
            },
            _ => 0,
        }
    }

    pub fn other(&self) -> i32 {
        match self {
            Perms::Unix(v) => {
                OTH_MODE & v
            }
            _ => 0
        }
    }

    /// Return the permissions bits for sticky bit, user,
    /// group and other. So, basically, file permissions
    pub fn sugo(&self) -> i32 {
        match self {
            Perms::Unix(v) => v & !S_IFMT,
            _ => 0,
        }
    }

    pub fn is_file(self) -> bool {
        match self {
            Perms::Unix(v) => S_IFREG & v > 0,
            _ => false
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Perms::Unix(v) => (S_IFDIR & v) > 0,
            _ => false,
        }
    }

    pub fn is_link(&self) -> bool {
        match self {
            Perms::Unix(v) => S_IFLNK & v > 0,
        _ => false
        }
    }

    pub fn is_sticky(&self) -> bool {
        match self {
            Perms::Unix(v) => S_ISVTX & v > 0,
        _ => false
        }
    }

    pub fn is_sgid(&self) -> bool {
        match self {
            Perms::Unix(v) => S_ISGID & v > 0,
        _ => false
        }
    }

    pub fn is_suid(&self) -> bool {
        match self {
            Perms::Unix(v) => S_ISUID & v > 0,
        _ => false
        }
    }
}

use ascii::{AsciiStr, AsciiChar, AsciiString };
/// Enum that wraps permissions for the particular system
const S_IFMT  : i32 =0o0170000; /* type of file */
const S_IFIFO : i32 =0o0010000; /* named pipe (fifo) */
const S_IFCHR : i32 =0o0020000; /* character special */
const S_IFDIR : i32 =0o0040000; /* directory */
const S_IFBLK : i32 =0o0060000; /* block special */
const S_IFREG : i32 =0o0100000; /* regular */
const S_IFLNK : i32 =0o0120000; /* symbolic link */
const S_IFSOCK: i32 =0o0140000; /* socket */
const S_IFWHT : i32 =0o0160000; /* whiteout */
const S_ISUID : i32 =0o0004000; /* set user id on execution */
const S_ISGID : i32 =0o0002000; /* set group id on execution */
const S_ISVTX : i32 =0o0001000; /* save swapped text even after use */
const S_IRUSR : i32 =0o0000400; /* read permission, owner */
const S_IWUSR : i32 =0o0000200; /* write permission, owner */
const S_IXUSR : i32 =0o0000100; /* execute/search permission, owner */
const S_IRWXG : i32 =0o0000070;    //mask for group permissions
const S_IRGRP : i32 =0o0000040;     //group has read permission
const S_IWGRP : i32 =0o0000020;     //group has write permission
const S_IXGRP : i32 =0o0000010;     //group has execute permission
const S_IRWXO : i32 =0o0000007;     //mask for permissions for others (not in group)
const S_IROTH : i32 =0o0000004;    //others have read permission
const S_IWOTH : i32 =0o0000002;     //others have write permission
const S_IXOTH : i32 =0o0000001;     //others have execute permission

const USR_MODE : i32 =  S_IRUSR | S_IWUSR | S_IXUSR;
const GRP_MODE : i32 =  S_IRGRP | S_IWGRP | S_IXGRP;
const OTH_MODE : i32 =  S_IROTH | S_IWOTH | S_IXOTH;


/// Get the bits involved in determining the file permissions.
/// That would be sticky/sgid/suid and owner, group, other bits
pub fn file_perms(input: i32) -> i32 {
    !S_IFMT & input
}
/// Get the bits involved in determining file type
pub fn file_type(input: i32) -> i32 {
    S_IFMT & input
}

//const PERMS: &'static AsciiStr = "rwxrwxrwx";
const IVALS: [i32;9] = [S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP, S_IXGRP, S_IROTH, S_IWOTH, S_IXOTH];

/// Given st_mode, return an ascii representation of the file
/// permissions, as one might get on the command line via, say,
/// `ls -l`
pub fn pretty_perms(val: i32) -> AsciiString {
    // Definition of stick bit, setuid and setgid taken from:
    // https://www.thegeekdiary.com/what-is-suid-sgid-and-sticky-bit/
    let perms = AsciiStr::from_ascii("rwxrwxrwx").unwrap();
    let val = val & !S_IFMT;
    // initialize the fold with an AsciiString with 10 elements. The
    // first element represents the sticky bit. We initialize the last 9
    // chars to their respective values as determined by the perms var.
    let mut out = (0..9).fold( AsciiString::from_ascii("----------").unwrap(),
                           |mut sum, x| { if (val & IVALS[x]) > 0 {
                                                sum[x+1] = perms[x];
                                        }
                                        sum });
    /*
      0    1  2  3  4  5  6  7  8  9
    sticky ur uw ux gr gw gx or ow ox

    now we have to evalute the sticky bit
    Settings:
    T refers to when the owner execute permissions are off.
    t refers to when the owner execute permissions are on.
    */

    if (val & S_ISVTX) > 0 {
        out[0] = if out[3] == AsciiChar::Minus { AsciiChar::T } else { AsciiChar::t };
    }
    /*
    The setgid permission displays as an “s” in the group’s execute field.
    If a lowercase letter “l” appears in the group’s execute field,
    it indicates that the setgid bit is on, and the execute bit for the group is off or denied.
     */
    if (val & S_ISGID) > 0 {
        out[6] = if out[6] == AsciiChar::Minus { AsciiChar::l } else { AsciiChar::s };
    }
    /*
    The setuid permission displayed as an “s” in the owner’s execute field.
    If a capital “S” appears in the owner’s execute field, it indicates that
    the setuid bit is on, and the execute bit “x” for the owner of the
    file is off or denied.
     */
    if (val & S_ISUID) > 0 {
        out[9] = if out[9] == AsciiChar::Minus { AsciiChar::S } else { AsciiChar::s };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::Permissions;
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    //
    // other tests
    //

    #[test]
    fn pretty_perms_passed_0007() {
        assert_eq!(pretty_perms(0o0007), AsciiString::from_ascii("-------rwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0006() {
        assert_eq!(pretty_perms(0o0006), AsciiString::from_ascii("-------rw-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0005() {
        assert_eq!(pretty_perms(0o0005), AsciiString::from_ascii("-------r-x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0004() {
        assert_eq!(pretty_perms(0o0004), AsciiString::from_ascii("-------r--").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0003() {
        assert_eq!(pretty_perms(0o0003), AsciiString::from_ascii("--------wx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0002() {
        assert_eq!(pretty_perms(0o0002), AsciiString::from_ascii("--------w-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0001() {
        assert_eq!(pretty_perms(0o0001), AsciiString::from_ascii("---------x").unwrap());
    }

    //
    // group tests
    //

    #[test]
    fn pretty_perms_passed_0070() {
        assert_eq!(pretty_perms(0o0070), AsciiString::from_ascii("----rwx---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0060() {
        assert_eq!(pretty_perms(0o0060), AsciiString::from_ascii("----rw----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0050() {
        assert_eq!(pretty_perms(0o0050), AsciiString::from_ascii("----r-x---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0040() {
        assert_eq!(pretty_perms(0o0040), AsciiString::from_ascii("----r-----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0030() {
        assert_eq!(pretty_perms(0o0030), AsciiString::from_ascii("-----wx---").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0020() {
        assert_eq!(pretty_perms(0o0020), AsciiString::from_ascii("-----w----").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0010() {
        assert_eq!(pretty_perms(0o0010), AsciiString::from_ascii("------x---").unwrap());
    }

    //
    // Owner Tests
    //

    #[test]
    fn pretty_perms_passed_0700() {
        assert_eq!(pretty_perms(0o0700), AsciiString::from_ascii("-rwx------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0600() {
        assert_eq!(pretty_perms(0o0600), AsciiString::from_ascii("-rw-------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0500() {
        assert_eq!(pretty_perms(0o0500), AsciiString::from_ascii("-r-x------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0400() {
        assert_eq!(pretty_perms(0o0400), AsciiString::from_ascii("-r--------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0300() {
        assert_eq!(pretty_perms(0o0300), AsciiString::from_ascii("--wx------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0200() {
        assert_eq!(pretty_perms(0o0200), AsciiString::from_ascii("--w-------").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0100() {
        assert_eq!(pretty_perms(0o0100), AsciiString::from_ascii("---x------").unwrap());
    }

    // all no sticky bit
    #[test]
    fn pretty_perms_passed_0777() {
        assert_eq!(pretty_perms(0o0777), AsciiString::from_ascii("-rwxrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0776() {
        assert_eq!(pretty_perms(0o0776), AsciiString::from_ascii("-rwxrwxrw-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0775() {
        assert_eq!(pretty_perms(0o0775), AsciiString::from_ascii("-rwxrwxr-x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0774() {
        assert_eq!(pretty_perms(0o0774), AsciiString::from_ascii("-rwxrwxr--").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0772() {
        assert_eq!(pretty_perms(0o0772), AsciiString::from_ascii("-rwxrwx-w-").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0771() {
        assert_eq!(pretty_perms(0o0771), AsciiString::from_ascii("-rwxrwx--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_0751() {
        assert_eq!(pretty_perms(0o0751), AsciiString::from_ascii("-rwxr-x--x").unwrap());
    }

    // sticky bit
    #[test]
    fn pretty_perms_passed_1777_sticky_bit_on() {
        assert_eq!(pretty_perms(0o1777), AsciiString::from_ascii("trwxrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1177_sticky_bit_on_owner_exe() {
        assert_eq!(pretty_perms(0o1177), AsciiString::from_ascii("t--xrwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1477_sticky_bit_on_owner_read() {
        assert_eq!(pretty_perms(0o1477), AsciiString::from_ascii("Tr--rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1277_sticky_bit_on_owner_write() {
        assert_eq!(pretty_perms(0o1277), AsciiString::from_ascii("T-w-rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1677_sticky_bit_on_owner_readwrite() {
        assert_eq!(pretty_perms(0o1677), AsciiString::from_ascii("Trw-rwxrwx").unwrap());
    }

    #[test]
    fn pretty_perms_passed_1077_sticky_bit_on_owner_off() {
        assert_eq!(pretty_perms(0o1077), AsciiString::from_ascii("T---rwxrwx").unwrap());
    }


    #[test]
    fn pretty_perms_passed_2751_sgid_on_group_exe_on() {
        assert_eq!(pretty_perms(0o2751), AsciiString::from_ascii("-rwxr-s--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_2741_sgid_on_group_exe_off() {
        assert_eq!(pretty_perms(0o2741), AsciiString::from_ascii("-rwxr-l--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_3751_sgid_on_group_exe_on() {
        assert_eq!(pretty_perms(0o3751), AsciiString::from_ascii("trwxr-s--x").unwrap());
    }

    #[test]
    fn pretty_perms_passed_3741_sgid_on_group_exe_off() {
        assert_eq!(pretty_perms(0o3741), AsciiString::from_ascii("trwxr-l--x").unwrap());
    }

    #[test]
    fn test_permissions() {
        let fname = "test_permissions.txt";
        let  f = File::create(fname).unwrap();
        let metadata = f.metadata().unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o777);
        let mode = permissions.mode();
        std::fs::remove_file(fname).expect("unable to remove temp file");
        assert_eq!(mode, 0o777);
    }

    #[test]
    fn test_permissions_ugo() {
        let fname = "test_permissions_ugo.txt";
        let  f = File::create(fname).unwrap();
        let metadata = f.metadata().unwrap();
        let  permissions = metadata.permissions();
        //permissions.set_mode(0o777);
        // so is it a u32 or an i32????
        let pmode = permissions.mode() as i32;
        let mode = pretty_perms(pmode);
        std::fs::remove_file(fname).expect("unable to remove temp file");
        assert_eq!(mode, AsciiString::from_ascii("-rw-r--r--").unwrap());
        assert_eq!(pmode & !S_IFMT, 0o644);
    }

    #[test]
    fn test_file_perm_call_with_100644() {
        let perms = 0o100644;
        assert_eq!(file_perms(perms), 0o0644);
    }

    #[test]
    fn test_file_perm_call_with_100000() {
        let perms = 0o100000;
        assert_eq!(file_perms(perms), 0o0000);
    }

    #[test]
    fn test_file_type_call_with_100644() {
        let perms = 0o100644;
        assert_eq!(file_type(perms), 0o100000);
    }

    #[test]
    fn test_file_type_call_with_000777() {
        let perms = 0o000777;
        assert_eq!(file_type(perms), 0o000000);
    }

}