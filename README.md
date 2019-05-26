# jstemplate2
a jobsystem template is  an ayclic graph whose nodes represent potential directories within the file system. Each node carries information that allows us to match path entries against it for validaiton purposes. The node may store an explicit name, such as `etc` or `SHARED`, or a regex that provides more general matching, as would be the case for `show`, `sequence`, or `shot`.

