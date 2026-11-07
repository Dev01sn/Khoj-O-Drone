// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from whycode_interfaces:srv/GetGuiSettings.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "whycode_interfaces/srv/detail/get_gui_settings__struct.h"
#include "whycode_interfaces/srv/detail/get_gui_settings__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool whycode_interfaces__srv__get_gui_settings__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[64];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("whycode_interfaces.srv._get_gui_settings.GetGuiSettings_Request", full_classname_dest, 63) == 0);
  }
  whycode_interfaces__srv__GetGuiSettings_Request * ros_message = _ros_message;
  ros_message->structure_needs_at_least_one_member = 0;

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * whycode_interfaces__srv__get_gui_settings__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of GetGuiSettings_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("whycode_interfaces.srv._get_gui_settings");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "GetGuiSettings_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  (void)raw_ros_message;

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "whycode_interfaces/srv/detail/get_gui_settings__struct.h"
// already included above
// #include "whycode_interfaces/srv/detail/get_gui_settings__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool whycode_interfaces__srv__get_gui_settings__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[65];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("whycode_interfaces.srv._get_gui_settings.GetGuiSettings_Response", full_classname_dest, 64) == 0);
  }
  whycode_interfaces__srv__GetGuiSettings_Response * ros_message = _ros_message;
  {  // draw_coords
    PyObject * field = PyObject_GetAttrString(_pymsg, "draw_coords");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->draw_coords = (Py_True == field);
    Py_DECREF(field);
  }
  {  // draw_segments
    PyObject * field = PyObject_GetAttrString(_pymsg, "draw_segments");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->draw_segments = (Py_True == field);
    Py_DECREF(field);
  }
  {  // coords
    PyObject * field = PyObject_GetAttrString(_pymsg, "coords");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->coords = (int8_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * whycode_interfaces__srv__get_gui_settings__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of GetGuiSettings_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("whycode_interfaces.srv._get_gui_settings");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "GetGuiSettings_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  whycode_interfaces__srv__GetGuiSettings_Response * ros_message = (whycode_interfaces__srv__GetGuiSettings_Response *)raw_ros_message;
  {  // draw_coords
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->draw_coords ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "draw_coords", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // draw_segments
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->draw_segments ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "draw_segments", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // coords
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->coords);
    {
      int rc = PyObject_SetAttrString(_pymessage, "coords", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
