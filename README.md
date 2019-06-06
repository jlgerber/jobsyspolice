# Jobsystem Template
A jobsystem template is an (hopefully) ayclic graph whose nodes represent potential directories within the file system. Each node carries information that allows us to match path entries against it for validaiton purposes. The node may store an explicit name, such as `etc` or `SHARED`, or a regex that provides more general matching, as would be the case for a `show`, `sequence`, or `shot`, which one would like to constrain in the template without naming explicitly. (It would not be particularly ergonomic if one had to update the template every time a show was created).

The jobsystem Template may provide other useful metadata in addition to the name; it may provide the intended owner and permissions.

# `disk::DiskType`s

The ability to create directories and volumes is provided by the Disk trait. Implementations may be found in the `disk` directory in th src, mapping to the `disk` submodule. These implmentations are responsible for handling the reqiurements imposed by particular disk systems (eg Netapp, Isilon, etc). By default, local storage is configured. Local Storage makes no affordance for Volumes, handling them the same as any other directory.

# Local Disk
The local::DiskService implementation relies on sudo and the sudoers file to handle passwordless chowning of files from a service account (by default, `jobsys`) to other accounts, excluding root. This implies that one must set up the service account thusly:

```
jobsys ALL=(ALL) NOPASSWRD: /usr/sbin/chown
```