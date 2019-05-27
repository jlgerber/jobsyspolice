# Jobsystem Template
A jobsystem template is an (hopefully) ayclic graph whose nodes represent potential directories within the file system. Each node carries information that allows us to match path entries against it for validaiton purposes. The node may store an explicit name, such as `etc` or `SHARED`, or a regex that provides more general matching, as would be the case for a `show`, `sequence`, or `shot`, which one would like to constrain in the template without naming explicitly. (It would not be particularly ergonomic if one had to update the template every time a show was created).

The jobsystem Template may provide other useful metadata in addition to the name; it may provide the intended owner and permissions.

