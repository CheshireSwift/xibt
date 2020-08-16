use crate::id;
use exitfailure::ExitFailure;
use std::io::Write;
use sxd_document::{
  dom::{Document, Element},
  parser,
  writer::Writer,
};

pub fn outlets<W: Write>(
  xml: &String,
  collection: &String,
  xpath: &String,
  writer: &mut W,
) -> Result<(), ExitFailure> {
  let package = parser::parse(&xml)?;
  let document = package.as_document();

  let element_ids: Vec<&str> = elements_for_path(&document, xpath)?
    .iter()
    .filter_map(|elem| elem.attribute_value("id"))
    .collect();

  let outlet_collections = element_ids.iter().map(|elem_id| {
    let elem = document.create_element("outletCollection");
    elem.set_attribute_value("property", collection);
    elem.set_attribute_value("destination", elem_id);
    elem.set_attribute_value("collectionClass", "NSMutableArray");
    elem.set_attribute_value("id", &id::gen());
    elem
  });

  let conn_elems = elements_for_path(&document, "./document/objects/placeholder/connections")?;
  if conn_elems.len() > 1 {
    panic!("Multiple elements found for ./document/objects/placeholder/connections");
  }

  let connections = conn_elems.first().unwrap();
  connections.append_children(outlet_collections);

  Writer::new()
    .set_single_quotes(false)
    .format_document(&document, writer)?;

  Ok(())
}

fn elements_for_path<'d>(
  document: &'d Document,
  xpath: &str,
) -> Result<Vec<Element<'d>>, sxd_xpath::Error> {
  let value = sxd_xpath::evaluate_xpath(&document, xpath)?;
  Ok(match value {
    sxd_xpath::Value::Nodeset(n) => n.iter().filter_map(|node| node.element()).collect(),
    _ => vec![],
  })
}

#[test]
fn extract_a_collection_reference() {
  let output = test_util::run_outlets(test_util::XML_SINGLE_LABEL, "//label");

  let package = parser::parse(&output).unwrap();
  let document = package.as_document();
  let collection_tags = elements_for_path(&document, r#"//connections/outletCollection"#).unwrap();

  assert_eq!(collection_tags.len(), 1);
  assert_eq!(
    collection_tags
      .get(0)
      .unwrap()
      .attribute("destination")
      .unwrap()
      .value(),
    "test-id"
  );
}

#[test]
fn extract_specific_collection_references() {
  let output = test_util::run_outlets(test_util::XML_MULTIPLE_LABEL, "//label[fontDescription]");

  let package = parser::parse(&output).unwrap();
  let document = package.as_document();
  let collection_tags = elements_for_path(&document, r#"//connections/outletCollection"#).unwrap();

  let mut destinations: Vec<&str> = collection_tags
    .iter()
    .map(|tag| tag.attribute("destination").unwrap().value())
    .collect();

  destinations.sort();

  assert_eq!(destinations, vec!("test-1", "test-3"));
}

#[cfg(test)]
mod test_util {
  pub fn run_outlets(input: &str, outlets_xpath: &str) -> String {
    let mut writer = vec![];
    super::outlets(
      &input.to_string(),
      &"test_collection".to_string(),
      &outlets_xpath.to_string(),
      &mut writer,
    )
    .unwrap();

    String::from_utf8(writer).unwrap()
  }

  pub const XML_SINGLE_LABEL: &str = r#"
<document>
  <objects>
    <placeholder>
      <connections>
        <outlet destination="kd0-q0-77h" id="vgt-Qs-f9G" property="titleTextField"/>
      </connections>
    </placeholder>
    <foo>
      <bar>
        <label id="test-id">Whee</label>
      </bar>
    </foo>
  </objects>
</document>
"#;

  pub const XML_MULTIPLE_LABEL: &str = r#"
<document>
  <objects>
    <placeholder>
      <connections>
        <outlet destination="kd0-q0-77h" id="vgt-Qs-f9G" property="titleTextField"/>
      </connections>
    </placeholder>
    <foo>
      <bar>
        <label id="test-1"><fontDescription /></label>
        <label id="test-2" />
        <label id="test-3"><fontDescription /></label>
      </bar>
    </foo>
  </objects>
</document>
"#;
}
