use std::process::Command;

struct FindBuilder {
    args: Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////

pub fn list_all_files() -> String {
    let root = "/";

    let name_blacklist = vec![".git", ".hg", ".svn"];

    let path_blacklist = vec![
        "/tmp",
        "/afs",
        "/media",
        "/mnt",
        "/net",
        "/sfs",
        "/udev",
        "/var/cache",
        "/var/lib/pacman/local",
        "/var/lock",
        "/var/run",
        "/var/spool",
        "/var/tmp",
    ];

    let filesystem_type_blacklist = vec![
        "9p",
        "afs",
        "anon_inodefs",
        "auto",
        "autofs",
        "bdev",
        "binfmt_misc",
        "cgroup",
        "cifs",
        "coda",
        "configfs",
        "cpuset",
        "cramfs",
        "debugfs",
        "devpts",
        "devtmpfs",
        "ecryptfs",
        "exofs",
        "ftpfs",
        "fuse",
        "fuse.encfs",
        "fuse.sshfs",
        "fusectl",
        "gfs",
        "gfs2",
        "hugetlbfs",
        "inotifyfs",
        "iso9660",
        "jffs2",
        "lustre",
        "mqueue",
        "ncpfs",
        "nfs",
        "nfs4",
        "nfsd",
        "pipefs",
        "proc",
        "ramfs",
        "rootfs",
        "rpc_pipefs",
        "securityfs",
        "selinuxfs",
        "sfs",
        "shfs",
        "smbfs",
        "sockfs",
        "sshfs",
        "sysfs",
        "tmpfs",
        "ubifs",
        "udf",
        "usbfs",
        "vboxsf",
    ];

    let mut builder = FindBuilder::new(root);

    for n in name_blacklist {
        builder.exclude_name(n);
    }

    for p in path_blacklist {
        builder.exclude_path(p);
    }

    for fs in filesystem_type_blacklist {
        builder.exclude_filesystem_type(fs);
    }

    builder.run()
}


impl FindBuilder {
    pub fn new(starting_directory: &str) -> FindBuilder {
        let mut builder = FindBuilder { args: Vec::new() };

        // builder.extend(&["-D", "tree"]); // for debugging

        builder.extend(&[starting_directory]);

        // don't recurse into other filesystems
        // TODO not sure I want this...
        builder.extend(&["-xdev"]);

        builder
    }

    pub fn run(&mut self) -> String {
        // print only regular files and symlinks
        self.extend(&["(", "-type", "f", "-o", "-type", "l", ")", "-print"]);

        let output = Command::new("find")
            .args(&self.args)
            .output()
            .expect("failed to call `find`");

        // println!("{}", String::from_utf8(output.stderr).expect("not UTF-8"));

        String::from_utf8(output.stdout).expect("not UTF-8")
    }

    fn exclude_path(&mut self, path: &str) {
        self.exclude_helper("-path", path);
    }

    fn exclude_filesystem_type(&mut self, fstype: &str) {
        self.exclude_helper("-fstype", fstype);
    }

    fn exclude_name(&mut self, name: &str) {
        self.exclude_helper("-name", name);
    }

    fn exclude_helper(&mut self, option: &str, value: &str) {
        self.extend(&["(", option, value, ")", "-prune", "-o"]);
    }

    fn push(&mut self, new_arg: &str) {
        self.args.push(new_arg.to_owned());
    }

    fn extend(&mut self, new_args: &[&str]) {
        let owned: Vec<_> =
            new_args.into_iter().map(|s| s.to_string()).collect();
        self.args.extend(owned);
    }
}
