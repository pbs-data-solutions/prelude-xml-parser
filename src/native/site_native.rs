use chrono::{DateTime, Utc};

#[cfg(feature = "python")]
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDateTime, PyDict},
};

use serde::{Deserialize, Serialize};

pub use crate::native::common::{Category, Comment, Entry, Field, Form, Reason, State, Value};

#[cfg(feature = "python")]
use crate::native::deserializers::to_py_datetime;

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,
    #[serde(rename = "uniqueId")]
    #[serde(alias = "@uniqueId")]
    #[serde(alias = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "numberOfPatients")]
    #[serde(alias = "@numberOfPatients")]
    #[serde(alias = "numberOfPatients")]
    pub number_of_patients: usize,
    #[serde(rename = "countOfRandomizedPatients")]
    #[serde(alias = "@countOfRandomizedPatients")]
    #[serde(alias = "countOfRandomizedPatients")]
    pub count_of_randomized_patients: usize,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: DateTime<Utc>,
    #[serde(alias = "@creator")]
    #[serde(alias = "creator")]
    pub creator: String,
    #[serde(rename = "numberOfForms")]
    #[serde(alias = "@numberOfForms")]
    #[serde(alias = "numberOfForms")]
    pub number_of_forms: usize,

    #[serde(rename = "form")]
    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Site {
    #[serde(alias = "@name")]
    #[serde(alias = "name")]
    pub name: String,
    #[serde(rename = "uniqueId")]
    #[serde(alias = "@uniqueId")]
    #[serde(alias = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "numberOfPatients")]
    #[serde(alias = "@numberOfPatients")]
    #[serde(alias = "numberOfPatients")]
    pub number_of_patients: usize,
    #[serde(rename = "countOfRandomizedPatients")]
    #[serde(alias = "@countOfRandomizedPatients")]
    #[serde(alias = "countOfRandomizedPatients")]
    pub count_of_randomized_patients: usize,
    #[serde(rename = "whenCreated")]
    #[serde(alias = "@whenCreated")]
    #[serde(alias = "whenCreated")]
    pub when_created: DateTime<Utc>,
    #[serde(alias = "@creator")]
    #[serde(alias = "creator")]
    pub creator: String,
    #[serde(rename = "numberOfForms")]
    #[serde(alias = "@numberOfForms")]
    #[serde(alias = "numberOfForms")]
    pub number_of_forms: usize,

    #[serde(rename = "form")]
    #[serde(alias = "form")]
    pub forms: Option<Vec<Form>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Site {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn unique_id(&self) -> PyResult<String> {
        Ok(self.unique_id.clone())
    }

    #[getter]
    fn number_of_patients(&self) -> PyResult<usize> {
        Ok(self.number_of_patients)
    }

    #[getter]
    fn count_of_randomized_patients(&self) -> PyResult<usize> {
        Ok(self.count_of_randomized_patients)
    }

    #[getter]
    fn when_created<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when_created)
    }

    #[getter]
    fn creator(&self) -> PyResult<String> {
        Ok(self.creator.clone())
    }

    #[getter]
    fn number_of_forms(&self) -> PyResult<usize> {
        Ok(self.number_of_forms)
    }

    #[getter]
    fn forms(&self) -> PyResult<Option<Vec<Form>>> {
        Ok(self.forms.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("unique_id", &self.unique_id)?;
        dict.set_item("number_of_patients", self.number_of_patients)?;
        dict.set_item(
            "count_of_randomized_patients",
            self.count_of_randomized_patients,
        )?;
        dict.set_item("when_created", to_py_datetime(py, &self.when_created)?)?;
        dict.set_item("creator", &self.creator)?;
        dict.set_item("number_of_forms", self.number_of_forms)?;

        let mut form_dicts = Vec::new();
        if let Some(forms) = &self.forms {
            for form in forms {
                let form_dict = form.to_dict(py)?;
                form_dicts.push(form_dict);
            }
            dict.set_item("forms", form_dicts)?;
        } else {
            dict.set_item("forms", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
/// Contains the information from the Prelude native site XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteNative {
    #[serde(alias = "site")]
    // #[serde(alias = "site")]
    pub sites: Vec<Site>,
}

#[cfg(not(feature = "python"))]
impl SiteNative {
    /// Convert to a JSON string
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use prelude_xml_parser::parse_site_native_file;
    ///
    /// let file_path = Path::new("tests/assets/site_native_small.xml");
    /// let native = parse_site_native_file(&file_path).unwrap();
    /// let json = native.to_json().unwrap();
    /// println!("{json}");
    /// // Verify the JSON contains expected site data
    /// assert!(json.contains("Some Site"));
    /// assert!(json.contains("Paul Sanders"));
    /// assert!(json.contains("Some Company"));
    /// ```
    pub fn to_json(&self) -> serde_json::Result<String> {
        let json = serde_json::to_string(&self)?;

        Ok(json)
    }
}

#[cfg(feature = "python")]
/// Contains the information from the Prelude native site XML.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct SiteNative {
    #[serde(rename = "site")]
    pub sites: Vec<Site>,
}

#[cfg(feature = "python")]
#[pymethods]
impl SiteNative {
    #[getter]
    fn sites(&self) -> PyResult<Vec<Site>> {
        Ok(self.sites.clone())
    }

    /// Convert the class instance to a dictionary
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        let mut site_dicts = Vec::new();
        for site in &self.sites {
            let site_dict = site.to_dict(py)?;
            site_dicts.push(site_dict);
        }
        dict.set_item("sites", site_dicts)?;
        Ok(dict)
    }

    /// Convert the class instance to a JSON string
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self)
            .map_err(|_| PyErr::new::<PyValueError, _>("Error converting to JSON"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn deserialize_site_native_json() {
        let json_str = r#"{
  "sites": [
    {
      "name": "Some Site",
      "uniqueId": "1681574834910",
      "numberOfPatients": 4,
      "countOfRandomizedPatients": 0,
      "whenCreated": "2023-04-15T16:08:19Z",
      "creator": "Paul Sanders",
      "numberOfForms": 1,
      "forms": [
        {
          "name": "demographic.form.name.site.demographics",
          "lastModified": "2023-04-15T16:08:19Z",
          "whoLastModifiedName": "Paul Sanders",
          "whoLastModifiedRole": "Project Manager",
          "whenCreated": 1681574834930,
          "hasErrors": false,
          "hasWarnings": false,
          "locked": false,
          "user": null,
          "dateTimeChanged": null,
          "formTitle": "Site Demographics",
          "formIndex": 1,
          "formGroup": "Demographic",
          "formState": "In-Work",
          "states": [
            {
              "value": "form.state.in.work",
              "signer": "Paul Sanders - Project Manager",
              "signerUniqueId": "1681162687395",
              "dateSigned": "2023-04-15T16:08:19Z"
            }
          ],
          "categories": [
            {
              "name": "Demographics",
              "categoryType": "normal",
              "highestIndex": 0,
              "fields": [
                {
                  "name": "address",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                },
                {
                  "name": "company",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entries": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-04-15T16:08:19Z",
                        "value": "Some Company"
                      },
                      "reason": null
                    }
                  ]
                },
                {
                  "name": "site_code_name",
                  "fieldType": "hidden",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:08:19Z",
                        "value": "ABC-Some Site"
                      },
                      "reason": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:08:19Z",
                        "value": "calculated value"
                      }
                    },
                    {
                      "entryId": "2",
                      "value": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:07:24Z",
                        "value": "Some Site"
                      },
                      "reason": {
                        "by": "set from calculation",
                        "byUniqueId": null,
                        "role": "System",
                        "when": "2023-04-15T16:07:24Z",
                        "value": "calculated value"
                      }
                    }
                  ]
                }
              ]
            },
            {
              "name": "Enrollment",
              "categoryType": "normal",
              "highestIndex": 0,
              "field": [
                {
                  "name": "enrollment_closed_date",
                  "fieldType": "popUpCalendar",
                  "dataType": "date",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                },
                {
                  "name": "enrollment_open",
                  "fieldType": "radio",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-04-15T16:08:19Z",
                        "value": "Yes"
                      },
                      "reason": null
                    }
                  ]
                },
                {
                  "name": "enrollment_open_date",
                  "fieldType": "popUpCalendar",
                  "dataType": "date",
                  "errorCode": "valid",
                  "whenCreated": "2023-04-15T16:07:14Z",
                  "keepHistory": true,
                  "entry": null
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "Artemis",
      "uniqueId": "1691420994591",
      "numberOfPatients": 0,
      "countOfRandomizedPatients": 0,
      "whenCreated": "2023-08-07T15:14:23Z",
      "creator": "Paul Sanders",
      "numberOfForms": 1,
      "forms": [
        {
          "name": "demographic.form.name.site.demographics",
          "lastModified": "2023-08-07T15:14:23Z",
          "whoLastModifiedName": "Paul Sanders",
          "whoLastModifiedRole": "Project Manager",
          "whenCreated": 1691420994611,
          "hasErrors": false,
          "hasWarnings": false,
          "locked": false,
          "user": null,
          "dateTimeChanged": null,
          "formTitle": "Site Demographics",
          "formIndex": 1,
          "formGroup": "Demographic",
          "formState": "In-Work",
          "states": [
            {
              "value": "form.state.in.work",
              "signer": "Paul Sanders - Project Manager",
              "signerUniqueId": "1681162687395",
              "dateSigned": "2023-08-07T15:14:23Z"
            }
          ],
          "categories": [
            {
              "name": "Demographics",
              "categoryType": "normal",
              "highestIndex": 0,
              "fields": [
                {
                  "name": "address",
                  "fieldType": "text",
                  "dataType": "string",
                  "errorCode": "valid",
                  "whenCreated": "2023-08-07T15:09:54Z",
                  "keepHistory": true,
                  "entries": [
                    {
                      "entryId": "1",
                      "value": {
                        "by": "Paul Sanders",
                        "byUniqueId": "1681162687395",
                        "role": "Project Manager",
                        "when": "2023-08-07T15:14:21Z",
                        "value": "1111 Moon Drive"
                      },
                      "reason": null
                    }
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
        "#;

        let result: SiteNative = serde_json::from_str(json_str).unwrap();

        assert_yaml_snapshot!(result);
    }
}
