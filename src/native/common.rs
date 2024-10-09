use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::{
    prelude::*,
    types::{PyDateTime, PyDict},
};

use crate::native::deserializers::{
    default_datetime_none, default_string_none, deserialize_empty_string_as_none,
    deserialize_empty_string_as_none_datetime,
};

#[cfg(feature = "python")]
use crate::native::deserializers::{to_py_datetime, to_py_datetime_option};

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(alias = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Value {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,
    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(alias = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Value {
    #[getter]
    fn by(&self) -> PyResult<String> {
        Ok(self.by.clone())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.clone())
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.clone())
    }

    #[getter]
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("by", &self.by)?;
        dict.set_item("by_unique_id", &self.by_unique_id)?;
        dict.set_item("role", &self.role)?;
        dict.set_item("when", to_py_datetime(py, &self.when)?)?;
        dict.set_item("value", &self.value)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reason {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(alias = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Reason {
    pub by: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub by_unique_id: Option<String>,

    pub role: String,
    pub when: DateTime<Utc>,

    #[serde(alias = "$value")]
    pub value: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Reason {
    #[getter]
    fn by(&self) -> PyResult<String> {
        Ok(self.by.clone())
    }

    #[getter]
    fn by_unique_id(&self) -> PyResult<Option<String>> {
        Ok(self.by_unique_id.clone())
    }

    #[getter]
    fn role(&self) -> PyResult<String> {
        Ok(self.role.clone())
    }

    #[getter]
    fn when<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when)
    }

    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("by", &self.by)?;
        dict.set_item("by_unique_id", &self.by_unique_id)?;
        dict.set_item("role", &self.role)?;
        dict.set_item("when", to_py_datetime(py, &self.when)?)?;
        dict.set_item("value", &self.value)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    #[serde(alias = "id")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct Entry {
    #[serde(alias = "id")]
    pub entry_id: String,
    pub value: Option<Value>,
    pub reason: Option<Reason>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Entry {
    #[getter]
    fn entry_id(&self) -> PyResult<String> {
        Ok(self.entry_id.clone())
    }

    #[getter]
    fn value(&self) -> PyResult<Option<Value>> {
        Ok(self.value.clone())
    }

    #[getter]
    fn reason(&self) -> PyResult<Option<Reason>> {
        Ok(self.reason.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("entry_id", &self.entry_id)?;
        if let Some(value) = &self.value {
            dict.set_item("value", value.to_dict(py)?)?;
        } else {
            dict.set_item("value", py.None())?;
        }
        if let Some(reason) = &self.reason {
            dict.set_item("reason", reason.to_dict(py)?)?;
        } else {
            dict.set_item("reason", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,

    #[serde(alias = "type")]
    pub field_type: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub data_type: Option<String>,
    pub error_code: String,
    pub when_created: DateTime<Utc>,
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Vec<Entry>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Field {
    pub name: String,

    #[serde(alias = "type")]
    pub field_type: String,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub data_type: Option<String>,

    pub error_code: String,
    pub when_created: DateTime<Utc>,
    pub keep_history: bool,

    #[serde(alias = "entry")]
    pub entries: Option<Vec<Entry>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Field {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn field_type(&self) -> PyResult<String> {
        Ok(self.field_type.clone())
    }

    #[getter]
    fn data_type(&self) -> PyResult<Option<String>> {
        Ok(self.data_type.clone())
    }

    #[getter]
    fn error_code(&self) -> PyResult<String> {
        Ok(self.error_code.clone())
    }

    #[getter]
    fn when_created<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        to_py_datetime(py, &self.when_created)
    }

    #[getter]
    fn keep_history(&self) -> PyResult<bool> {
        Ok(self.keep_history)
    }

    #[getter]
    fn entries(&self) -> PyResult<Option<Vec<Entry>>> {
        Ok(self.entries.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("field_type", &self.field_type)?;
        dict.set_item("data_type", &self.data_type)?;
        dict.set_item("error_code", &self.error_code)?;
        dict.set_item("when_created", to_py_datetime(py, &self.when_created)?)?;
        dict.set_item("keep_history", self.keep_history)?;

        let mut entry_dicts = Vec::new();
        if let Some(entries) = &self.entries {
            for entry in entries {
                let entry_dict = entry.to_dict(py)?;
                entry_dicts.push(entry_dict.to_object(py));
            }
            dict.set_item("entries", entry_dicts)?;
        } else {
            dict.set_item("entries", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,

    #[serde(alias = "type")]
    pub category_type: String,

    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Vec<Field>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass(get_all)]
pub struct Category {
    pub name: String,

    #[serde(alias = "type")]
    pub category_type: String,

    pub highest_index: usize,

    #[serde(alias = "field")]
    pub fields: Option<Vec<Field>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Category {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn category_type(&self) -> PyResult<String> {
        Ok(self.category_type.clone())
    }

    #[getter]
    fn highest_index(&self) -> PyResult<usize> {
        Ok(self.highest_index)
    }

    #[getter]
    fn fields(&self) -> PyResult<Option<Vec<Field>>> {
        Ok(self.fields.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("category_type", &self.category_type)?;
        dict.set_item("highest_index", self.highest_index)?;

        let mut field_dicts = Vec::new();
        if let Some(fields) = &self.fields {
            for field in fields {
                let field_dict = field.to_dict(py)?;
                field_dicts.push(field_dict.to_object(py));
            }
            dict.set_item("fields", field_dicts)?;
        } else {
            dict.set_item("fields", py.None())?;
        }

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub value: String,
    pub signer: String,
    pub signer_unique_id: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct State {
    pub value: String,
    pub signer: String,
    pub signer_unique_id: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_signed: Option<DateTime<Utc>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl State {
    #[getter]
    fn value(&self) -> PyResult<String> {
        Ok(self.value.clone())
    }

    #[getter]
    fn signer(&self) -> PyResult<String> {
        Ok(self.signer.clone())
    }

    #[getter]
    fn signer_unique_id(&self) -> PyResult<String> {
        Ok(self.signer_unique_id.clone())
    }

    #[getter]
    fn date_signed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_signed)
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("value", &self.value)?;
        dict.set_item("signer", &self.signer)?;
        dict.set_item("signer_unique_id", &self.signer_unique_id)?;
        dict.set_item("date_signed", to_py_datetime_option(py, &self.date_signed)?)?;

        Ok(dict)
    }
}

#[cfg(not(feature = "python"))]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    pub when_created: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub locked: bool,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    pub form_title: String,
    pub form_index: usize,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub form_group: Option<String>,

    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Vec<State>>,

    #[serde(alias = "category")]
    pub categories: Option<Vec<Category>>,
}

#[cfg(feature = "python")]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[pyclass]
pub struct Form {
    pub name: String,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub last_modified: Option<DateTime<Utc>>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_name: Option<String>,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub who_last_modified_role: Option<String>,

    pub when_created: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
    pub locked: bool,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub user: Option<String>,

    #[serde(
        default = "default_datetime_none",
        deserialize_with = "deserialize_empty_string_as_none_datetime"
    )]
    pub date_time_changed: Option<DateTime<Utc>>,

    pub form_title: String,
    pub form_index: usize,

    #[serde(
        default = "default_string_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub form_group: Option<String>,

    pub form_state: String,

    #[serde(alias = "state")]
    pub states: Option<Vec<State>>,

    #[serde(alias = "category")]
    pub categories: Option<Vec<Category>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl Form {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[getter]
    fn last_modified<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.last_modified)
    }

    #[getter]
    fn who_last_modified_name(&self) -> PyResult<Option<String>> {
        Ok(self.who_last_modified_name.clone())
    }

    #[getter]
    fn who_last_modified_role(&self) -> PyResult<Option<String>> {
        Ok(self.who_last_modified_role.clone())
    }

    #[getter]
    fn when_created(&self) -> PyResult<usize> {
        Ok(self.when_created)
    }

    #[getter]
    fn has_errors(&self) -> PyResult<bool> {
        Ok(self.has_errors)
    }

    #[getter]
    fn has_warnings(&self) -> PyResult<bool> {
        Ok(self.has_warnings)
    }

    #[getter]
    fn locked(&self) -> PyResult<bool> {
        Ok(self.locked)
    }

    #[getter]
    fn user(&self) -> PyResult<Option<String>> {
        Ok(self.user.clone())
    }

    #[getter]
    fn date_time_changed<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyDateTime>>> {
        to_py_datetime_option(py, &self.date_time_changed)
    }

    #[getter]
    fn form_title(&self) -> PyResult<String> {
        Ok(self.form_title.clone())
    }

    #[getter]
    fn form_index(&self) -> PyResult<usize> {
        Ok(self.form_index)
    }

    #[getter]
    fn form_group(&self) -> PyResult<Option<String>> {
        Ok(self.form_group.clone())
    }

    #[getter]
    fn form_state(&self) -> PyResult<String> {
        Ok(self.form_state.clone())
    }

    #[getter]
    fn states(&self) -> PyResult<Option<Vec<State>>> {
        Ok(self.states.clone())
    }

    #[getter]
    fn categories(&self) -> PyResult<Option<Vec<Category>>> {
        Ok(self.categories.clone())
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item(
            "last_modified",
            to_py_datetime_option(py, &self.last_modified)?,
        )?;
        dict.set_item("who_last_modified_name", &self.who_last_modified_name)?;
        dict.set_item("who_last_modified_role", &self.who_last_modified_role)?;
        dict.set_item("when_created", self.when_created)?;
        dict.set_item("has_errors", self.has_errors)?;
        dict.set_item("has_warnings", self.has_warnings)?;
        dict.set_item("locked", self.locked)?;
        dict.set_item("user", &self.user)?;
        dict.set_item(
            "date_time_changed",
            to_py_datetime_option(py, &self.date_time_changed)?,
        )?;
        dict.set_item("form_title", &self.form_title)?;
        dict.set_item("form_index", self.form_index)?;
        dict.set_item("form_group", &self.form_group)?;
        dict.set_item("form_state", &self.form_state)?;

        let mut state_dicts = Vec::new();
        if let Some(states) = &self.states {
            for state in states {
                let state_dict = state.to_dict(py)?;
                state_dicts.push(state_dict.to_object(py));
            }
            dict.set_item("states", state_dicts)?;
        } else {
            dict.set_item("states", py.None())?;
        }

        if let Some(categories) = &self.categories {
            let mut category_dicts = Vec::new();
            for category in categories {
                let category_dict = category.to_dict(py)?;
                category_dicts.push(category_dict.to_object(py));
            }
            dict.set_item("categories", category_dicts)?;
        } else {
            dict.set_item("categories", py.None())?;
        }

        Ok(dict)
    }
}
