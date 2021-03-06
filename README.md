# Jobsystem Template
A jobsystem template describes an ayclic graph whose nodes represent potential directories within the file system. Each node carries information that allows us to match path entries against it for validaiton purposes. The node may store an explicit name, such as `etc` or `SHARED`, or a regex that provides more general matching, as would be the case for a `show`, `sequence`, or `shot`, which one would like to constrain in the template without naming explicitly. (It would not be particularly ergonomic if one had to update the template every time a show was created).

The jobsystem Template may provide other useful metadata in addition to the name; it may provide the intended owner and permissions, environment variable names, and navigation aliases.

# disk::Disk

The ability to create directories and volumes is provided by the Disk trait. Implementations may be found in the `disk` directory in th src, mapping to the `disk` submodule. These implmentations are responsible for handling the reqiurements imposed by particular disk systems (eg Netapp, Isilon, etc). By default, local storage is configured. Local Storage makes no affordance for Volumes, handling them the same as any other directory.

# Local Disk
The local::DiskService implementation relies on setuid to handle creation of directories for specified owners with speicfic permissions.

In order to install appropriately, once must change the owner of `jsp` and `jspmk` to root, and set their setuid bits.
(chmod u+s).

The Makefile provides this facility provided you have appropriate sudo permissions

# Design Iteration

## Adding Template via code
I have added a macro that makes it easier to create nodes in code. Usage looks like this:

```rust
let refdir = graph.add_node(jspnode!("REF").set_volume());
let quicktimes = graph.add_node(jspnode!("quicktimes", "perms"=>"751"));
let qtsubdir = graph.add_node(jspnode!("qtsubdir", r"^[0-9_]+$"));
let clientvault = graph.add_node(jspnode!("CLIENT_VAULT").set_volume());
let clientvaultsd = graph.add_node(jspnode!("clientvault_subdir", r"^(incoming|outgoing)$"));
let clientvaultssd = graph.add_node(jspnode!("clientvault_ssd", r"^[0-9_]+$"));
let slates_n_categories = graph.add_node(jspnode!("slatesNcategories", r"(SLATES|CATGORIES)^$"));
let snc_sd = graph.add_node(jspnode!("snc_sd", r"^[a-z0-9_.-]+$"));
let locations = graph.add_node(jspnode!("LOCATIONS"));
let loc_sd = graph.add_node(jspnode!("loc_sd", r"^[a-z0-9_.-]+$"));
let loc_ssd = graph.add_node(jspnode!("loc_ssd", r"^[a-z0-9_.-]+$"));
let documents = graph.add_node(jspnode!("documents"));
let doc_sd = graph.add_node(jspnode!("doc_sd", r"^(agency|director_treatments|vfx_methodology|schedules|scripts|storyboards)$"));
let audio = graph.add_node(jspnode!("audio"));
let audio_sd = graph.add_node(jspnode!("audio_sd", r"^(mixes|sources)$"));
let threed = graph.add_node(jspnode!("3d"));
let threed_sd = graph.add_node(jspnode!("3d_sd", r"^(3d_assets|mocap)$"));
let chars = graph.add_node(jspnode!("CHARACTERS"));
let chars_sd = graph.add_node(
    jspnode!("chars_sd", r"^[a-z0-9_]+$", r"^(DEVL|SHARED|etc|lib|bin|user)$")
);
```

## Jsp Template Format
Even simpler than using the jspnode macro, the jspt format describes the graph
in a way that is much simpler than a generic template.

Here is an example of the `jspt` format, which the jsp commands now read:

```
[regex]

num_under =   "[0-9_]+"
quicktimes =  "quicktimes"
qtsubdir   =  "[0-9_]+" 
doc_sd     =  "(agency|director_treatments|vfx_methodology|schedules|scripts|storyboards)"
chars_sd   =  "(DEVL|SHARED|etc|lib|bin|user)"
show       = "[A-Z]+[A-Z0-9]*" "(REF|SHARED|OUTSOURCE|LOCATIONS)"

[nodes]

dd  
shows
show            = $show [ owner: jobsys, perms: 751, varname: DD_SHOW ]     
seq             = $seq  [ owner: jobsys, perms: 751, varname: DD_SHOW ]  
shot            = $shot [ owner: jobsys, perms: 751, varname: DD_SHOW ]  
refdir          = REF [ volume ]
shared          = SHARED
img             = IMG
quickimes       = $quicktimes [ perms: 751 ]
qtsubdir        = $qtsubdir
clientvault     = CLIENT_VAULT [ volume ]
clientvault_sd  = "(incoming|outgoing)"
clientvault_ssd = "[0-9_]+"

[graph] 
dd -> refdir -> quicktimes
dd -> shows -> show -> sequence -> shot
// speculative shared -> img | model | anim | fx 
show -> shared
seq -> shared
shot -> shared
```

## Demo
```
# clear out the stuff that is there
sudo rm -rf  /dd/shows/*
# create dev01 show (assuming template is in ~/etc)
jspmk --input ~/etc/template.jspt dev01
sudo cp ~/etc/template.jspt /dd/shows/DEV01/etc/.

# create some dev01 levels
jspmk dev01.rd.0001 work:jgerber
jspmk dev01.aa.0001 work:jgerber
jspmk dev01.rd.9999 work:jgerber

# go to the last creation
jspgo !:*

# create dev02
jspmk --input ~/etc/template.jspt dev01
sudo cp ~/etc/template.jspt /dd/shows/DEV01/etc/.

jspmk dev02.rd.0001 work:jgerber
jspmk dev02.aa.0001 work:jgerber
jspmk dev02.rd.9999 work:jgerber

# demonstrate relative path 
jspgo .aa.
jspgo .rd.
jspgo ..9999

jspgo dev01..
jspgo dev02.

jspgo dev01.rd.9999
jspgo ... work:jgerber
jspmk .. work:jgerber
jspgo .. work:jgerber

jspgo . work:jgerber
jspgo . work:jgerber

jspgo dev01
jspgo /dd/shows/DEV01/RD

# create a date directory 
jspmk dev01 work:jgerber -t
jspgo dev01 work:jgerber 

# set stickybit on creation
jspmk dev01.rd.2000 --sticky 
jspgo dev01.rd
ls -lrt

 