use nom::{
    IResult,
    sequence::{tuple, preceded, delimited, separated_pair, terminated},
    bytes::complete::tag,
    branch::alt,
    combinator::{map, recognize},
    character::complete::space0,
    multi::separated_nonempty_list,
};

use crate::jspt::{JsptMetadata, MetadataComponent, helpers::{variable, navalias_str, perm_chars} };

/// Parses metadata from a a str, identifed from a list of identifiers surounded by
/// square brackets. 
/// Note that this parser must be applied before the header parser to be effective. 
pub fn parse_metadata(input: &str) -> IResult<&str, JsptMetadata> {
    map(
        parse_components,
        |item|{
            let mut metadata = JsptMetadata::new();
            for component in item {
                match component {
                    MetadataComponent::Permissions(perm) => metadata = metadata.set_permissions(Some(perm)),
                    MetadataComponent::EnvVarName(name) => metadata = metadata.set_varname(Some(name)),
                    MetadataComponent::Owner(name) => metadata = metadata.set_owner(Some(name)),
                    MetadataComponent::Group(group) => metadata = metadata.set_group(Some(group)),
                    MetadataComponent::Volume => metadata = metadata.set_volume(true),
                    MetadataComponent::Autocreate => metadata = metadata.set_autocreate(true),
                    MetadataComponent::NavAlias(name, None) => metadata = metadata.set_navalias( Some((name, None)) ),
                    MetadataComponent::NavAlias(name, Some(value)) => metadata = metadata.set_navalias(Some((name,Some(value)))),
                    MetadataComponent::Separator => {
                        log::warn!("parse_metadata encountered Separateor");
                    }
                }
            }
            metadata
        }
    )(input)
}

pub fn parse_components(input: &str) -> IResult<&str, Vec<MetadataComponent>> {
    delimited( 
        preceded(space0,tag("[")),
        separated_nonempty_list(
            parse_comma,
            alt((
                parse_navalias,
                parse_autocreate,
                parse_volume,
                parse_permissions,
                parse_owner,
                parse_group,
                parse_varname,
            ))
        ), 
        terminated(tag("]"), space0)
    )
    (input)
}

#[cfg(test)]
mod parse_components_tests {
    use super::*;

    #[test]
    fn can_parse_volume() {
       let owner = parse_components("[ volume ]");
       assert_eq!(owner, Ok(("", vec![MetadataComponent::Volume]))) ;
    }

    #[test]
    fn can_parse_navalias() {
       let owner = parse_components("[ navalias: cs ]");
       assert_eq!(owner, Ok(("", vec![MetadataComponent::NavAlias("cs".to_string(), None)]))) ;
    }

    #[test]
    fn can_parse_navalias_complex() {
       let owner = parse_components("[ navalias: cs work.$USER ]");
       assert_eq!(owner, Ok(("", vec![MetadataComponent::NavAlias("cs".to_string(), Some("work.$USER".to_string()) )]))) ;
    }

    #[test]
    fn can_parse_2_volumes() {
       let owner = parse_components("[ volume ,volume]");
       assert_eq!(owner, Ok(("", vec![MetadataComponent::Volume, MetadataComponent::Volume]))) ;
    }

    #[test]
    fn can_parse_volume_and_owner() {
        let owner = parse_components("[ volume , owner : jgerber] ");
        assert_eq!(
           owner,
            Ok((
                "",
                vec![
                     MetadataComponent::Volume, 
                     MetadataComponent::Owner("jgerber".to_string())
                ]
            ))
        );
    }

    #[test]
    fn can_parse_owner_and_group() {
        let owner = parse_components("[ group: cgi , owner : jgerber] ");
        assert_eq!(
           owner,
            Ok((
                "",
                vec![
                     MetadataComponent::Group("cgi".to_string()), 
                     MetadataComponent::Owner("jgerber".to_string())
                ]
            ))
        );
    }
    #[test]
    fn can_parse_volume_and_owner_and_perms() {
        let cmp = parse_components("  [ volume , owner : jgerber, perms: 751 ]");
        assert_eq!(
           cmp,
            Ok((
                "",
                vec![
                     MetadataComponent::Volume, 
                     MetadataComponent::Owner("jgerber".to_string()),
                    MetadataComponent::Permissions("751".to_string())

                ]
            ))
        );
    }

    #[test]
    fn can_parse_volume_and_owner_and_perms_and_varname() {
        let cmp = parse_components("[ volume , owner : jgerber, perms: 751, varname: JG_SHOW]");
        assert_eq!(
           cmp,
            Ok((
                "",
                vec![
                     MetadataComponent::Volume, 
                     MetadataComponent::Owner("jgerber".to_string()),
                    MetadataComponent::Permissions("751".to_string()),
                    MetadataComponent::EnvVarName("JG_SHOW".to_string())

                ]
            ))
        );
    }

    #[test]
    fn can_parse_volume_and_owner_and_perms_and_varname_and_autocreate() {
        let cmp = parse_components("[ autocreate, volume , owner : jgerber, perms: 751, varname: JG_SHOW]");
        assert_eq!(
           cmp,
            Ok((
                "",
                vec![
                    MetadataComponent::Autocreate, 
                    MetadataComponent::Volume, 
                    MetadataComponent::Owner("jgerber".to_string()),
                    MetadataComponent::Permissions("751".to_string()),
                    MetadataComponent::EnvVarName("JG_SHOW".to_string())
                ]
            ))
        );
    }


    #[test]
    fn can_parse_volume_and_owner_and_perms_and_varname_and_autocreate_navalias() {
        let cmp = parse_components("[ autocreate, volume , owner : jgerber, perms: 751, varname: JG_SHOW, navalias: cs]");
        assert_eq!(
           cmp,
            Ok((
                "",
                vec![
                    MetadataComponent::Autocreate, 
                    MetadataComponent::Volume, 
                    MetadataComponent::Owner("jgerber".to_string()),
                    MetadataComponent::Permissions("751".to_string()),
                    MetadataComponent::EnvVarName("JG_SHOW".to_string()),
                    MetadataComponent::NavAlias("cs".to_string(), None)
                ]
            ))
        );
    }

    #[test]
    fn can_parse_volume_and_owner_and_perms_and_varname_and_autocreate_navalias_complex() {
        let cmp = parse_components("[ autocreate, volume , owner : jgerber, perms: 751, varname: JG_SHOW, navalias: cs work.$USER]");
        assert_eq!(
           cmp,
            Ok((
                "",
                vec![
                    MetadataComponent::Autocreate, 
                    MetadataComponent::Volume, 
                    MetadataComponent::Owner("jgerber".to_string()),
                    MetadataComponent::Permissions("751".to_string()),
                    MetadataComponent::EnvVarName("JG_SHOW".to_string()),
                    MetadataComponent::NavAlias("cs".to_string(), Some("work.$USER".to_string()))
                ]
            ))
        );
    }

}

fn parse_comma(input:  &str) -> IResult<&str, MetadataComponent> {
    map(
    tag(","),
    |_item|{
        MetadataComponent::Separator
    }
    )(input)
}


fn parse_autocreate(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(space0, tag("autocreate"), space0),
        |_item| {
            MetadataComponent::Autocreate
        }
    )(input)
}


#[cfg(test)]
mod autocreate_tests {
    use super::*;

    #[test]
    fn can_parse_autocreate_no_spaces() {
       let owner = parse_autocreate("autocreate");
       assert_eq!(owner, Ok(("", MetadataComponent::Autocreate))) ;
    }

    #[test]
    fn can_parse_autocreate_spaces() {
       let owner = parse_autocreate("  autocreate   ");
       assert_eq!(owner, Ok(("", MetadataComponent::Autocreate))) ;
    }
}

fn parse_volume(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(space0, tag("volume"), space0),
        |_item| {
            MetadataComponent::Volume
        }
    )(input)
}

#[cfg(test)]
mod volume_tests {
    use super::*;

    #[test]
    fn can_parse_volume_no_spaces() {
       let owner = parse_volume("volume");
       assert_eq!(owner, Ok(("", MetadataComponent::Volume))) ;
    }

    #[test]
    fn can_parse_volume_spaces() {
       let owner = parse_volume("  volume   ");
       assert_eq!(owner, Ok(("", MetadataComponent::Volume))) ;
    }
}

// owner : jgerber
fn parse_owner(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            separated_pair(
                tag("owner"),
                 preceded(space0,tag(":")), 
                 preceded(
                    space0,
                    alt((
                        variable,
                        recognize(tuple((tag("$"), variable)))
                    )) 
                 )

            ), 
            space0,
        ),
        |item| {
            let (_, owner_name) = item;
            MetadataComponent::Owner(owner_name.to_string())
        }
    )(input)
}

#[cfg(test)]
mod owner_tests {
    use super::*;

    #[test]
    fn can_parse_owner_no_spaces() {
       let owner = parse_owner("owner:fred");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }

    #[test]
    fn can_parse_owner_spaces() {
       let owner = parse_owner("owner : fred");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }

    #[test]
    fn can_parse_owner_variable() {
       let owner = parse_owner("owner : $fred");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("$fred".to_string())))) ;
    }

    #[test]
    fn can_parse_owner_more_spaces() {
       let owner = parse_owner("  owner : fred  ");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }
}

// convert permissions
fn parse_group(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            separated_pair(
                tag("group"),
                 preceded(space0,tag(":")), 
                 preceded(space0, variable),
            ),
            space0
        ),
        |item| {
            let (_,item) = item;
            MetadataComponent::Group(item.to_string())
        }
    )(input)
}

#[cfg(test)]
mod group_tests {
    use super::*;

    #[test]
    fn can_parse_group_no_spaces() {
        let p = parse_group("group:cgi");
        assert_eq!(p, Ok(("", MetadataComponent::Group("cgi".to_string()))));
    }

    #[test]
    fn can_parse_group_spaces() {
        let p = parse_group(" group :  cgi ");
        assert_eq!(p, Ok(("", MetadataComponent::Group("cgi".to_string()))));
        let p = parse_group(" group:  cgi ");
        assert_eq!(p, Ok(("", MetadataComponent::Group("cgi".to_string()))));
        let p = parse_group(" group :cgi ");
        assert_eq!(p, Ok(("", MetadataComponent::Group("cgi".to_string()))));
    }
}

// convert permissions
fn parse_permissions(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            separated_pair(
                tag("perms"),
                 preceded(space0,tag(":")), 
                 preceded(space0,perm_chars),
            ),
            //perm_chars,
            space0
        ),
        |item| {
            let (_,item) = item;
            MetadataComponent::Permissions(item.to_string())
        }
    )(input)
}


#[cfg(test)]
mod permissions_tests {
    use super::*;

    #[test]
    fn can_parse_perms_no_spaces() {
        let p = parse_permissions("perms:777");
        assert_eq!(p, Ok(("", MetadataComponent::Permissions("777".to_string()))));
    }

    #[test]
    fn can_parse_perms_spaces() {
        let p = parse_permissions(" perms :  777 ");
        assert_eq!(p, Ok(("", MetadataComponent::Permissions("777".to_string()))));
        let p = parse_permissions(" perms:  777 ");
        assert_eq!(p, Ok(("", MetadataComponent::Permissions("777".to_string()))));
        let p = parse_permissions(" perms :777 ");
        assert_eq!(p, Ok(("", MetadataComponent::Permissions("777".to_string()))));
    }
}


// varname : jgerber
fn parse_varname(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            separated_pair(
                tag("varname"),
                 preceded(space0,tag(":")), 
                 preceded(space0, variable)
            ), 
            space0,
        ),
        |item| {
            let (_, var_name) = item;
            MetadataComponent::EnvVarName(var_name.to_string())
        }
    )(input)
}


#[cfg(test)]
mod varname_tests {
    use super::*;

    #[test]
    fn can_parse_varname_no_spaces() {
       let varname = parse_varname("varname:fred");
       assert_eq!(varname, Ok(("", MetadataComponent::EnvVarName("fred".to_string())))) ;
    }

    #[test]
    fn can_parse_varname_spaces() {
       let varname = parse_varname("varname : fred");
       assert_eq!(varname, Ok(("", MetadataComponent::EnvVarName("fred".to_string())))) ;
    }
    #[test]
    fn can_parse_varname_more_spaces() {
       let varname = parse_varname("  varname : fred  ");
       assert_eq!(varname, Ok(("", MetadataComponent::EnvVarName("fred".to_string())))) ;
    }
}

#[allow(dead_code)]
fn parse_navalias(input: &str) -> IResult<&str, MetadataComponent> {
    alt((
      parse_navalias_some, 
      parse_navalias_none 
    ))(input)
}

#[cfg(test)]
mod navalias_tests {
    use super::*;

    #[test]
    fn can_parse_navalias_no_spaces() {
       let varname = parse_navalias("navalias:fred");
       assert_eq!(varname, Ok(("", MetadataComponent::NavAlias("fred".to_string(), None)))) ;
    }

    #[test]
    fn can_parse_navalias_spaces() {
       let varname = parse_navalias("navalias : fred");
       assert_eq!(varname, Ok(("", MetadataComponent::NavAlias("fred".to_string(),None)))) ;
    }

    #[test]
    fn can_parse_navalias_more_spaces() {
       let varname = parse_navalias("  navalias : fred  ");
       assert_eq!(varname, Ok(("", MetadataComponent::NavAlias("fred".to_string(), None)))) ;
    }


    #[test]
    fn can_parse_navalias_with_second_param() {
       let varname = parse_navalias("navalias: cs work.$USER");
       assert_eq!(varname, Ok(("", MetadataComponent::NavAlias("cs".to_string(), Some("work.$USER".to_string()))))) ;
    }

}

// navalias: cs work.$JGERBER,
#[allow(dead_code)]
fn parse_navalias_some(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            tuple((
                tag("navalias"),
                preceded(space0,tag(":")), 
                preceded(space0, variable),
                preceded(space0, navalias_str),
            )), 
            space0,
        ),
        |item| {
            let (_, _, var_name, var_value) = item;
            MetadataComponent::NavAlias(var_name.to_string(), Some(var_value.to_string()))
        }
    )(input)
}

// navalias: cs
#[allow(dead_code)]
fn parse_navalias_none(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,

            tuple((
                tag("navalias"),
                preceded(space0,tag(":")), 
                preceded(space0, variable),
            )), 
            space0,
        ),
        |item| {
            let (_, _, var_name) = item;
            MetadataComponent::NavAlias(var_name.to_string(), None)
        }
    )(input)
}
