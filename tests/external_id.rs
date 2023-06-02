use nom_xml::{parse::Parse, prolog::external_id::ExternalID};

#[test]

fn test_parse_external_id_system() {
    let input = "SYSTEM \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"";
    let (_rest, res) = ExternalID::parse(input).unwrap();

    match res {
        ExternalID::System(sys_id) => assert_eq!(
            sys_id,
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"
        ),
        _ => panic!("Expected a System ID, got a different variant."),
    }
}

#[test]
fn test_parse_external_id_public() {
    let input = "PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"";
    let (_rest, res) = ExternalID::parse(input).unwrap();

    match res {
        ExternalID::Public {
            pubid,
            system_identifier,
        } => {
            assert_eq!(pubid, "-//W3C//DTD XHTML 1.0 Transitional//EN");
            match *system_identifier {
                ExternalID::System(sys_id) => assert_eq!(
                    sys_id,
                    "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"
                ),
                _ => panic!("Expected a System ID, got a different variant."),
            }
        }
        _ => panic!("Expected a Public ID, got a different variant."),
    }
}
