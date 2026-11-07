# generated from rosidl_generator_py/resource/_idl.py.em
# with input from whycode_interfaces:srv/GetGuiSettings.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetGuiSettings_Request(type):
    """Metaclass of message 'GetGuiSettings_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('whycode_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'whycode_interfaces.srv.GetGuiSettings_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_gui_settings__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_gui_settings__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_gui_settings__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_gui_settings__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_gui_settings__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetGuiSettings_Request(metaclass=Metaclass_GetGuiSettings_Request):
    """Message class 'GetGuiSettings_Request'."""

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

import builtins  # noqa: E402, I100

# already imported above
# import rosidl_parser.definition


class Metaclass_GetGuiSettings_Response(type):
    """Metaclass of message 'GetGuiSettings_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('whycode_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'whycode_interfaces.srv.GetGuiSettings_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_gui_settings__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_gui_settings__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_gui_settings__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_gui_settings__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_gui_settings__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetGuiSettings_Response(metaclass=Metaclass_GetGuiSettings_Response):
    """Message class 'GetGuiSettings_Response'."""

    __slots__ = [
        '_draw_coords',
        '_draw_segments',
        '_coords',
    ]

    _fields_and_field_types = {
        'draw_coords': 'boolean',
        'draw_segments': 'boolean',
        'coords': 'int8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.draw_coords = kwargs.get('draw_coords', bool())
        self.draw_segments = kwargs.get('draw_segments', bool())
        self.coords = kwargs.get('coords', int())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.draw_coords != other.draw_coords:
            return False
        if self.draw_segments != other.draw_segments:
            return False
        if self.coords != other.coords:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def draw_coords(self):
        """Message field 'draw_coords'."""
        return self._draw_coords

    @draw_coords.setter
    def draw_coords(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'draw_coords' field must be of type 'bool'"
        self._draw_coords = value

    @builtins.property
    def draw_segments(self):
        """Message field 'draw_segments'."""
        return self._draw_segments

    @draw_segments.setter
    def draw_segments(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'draw_segments' field must be of type 'bool'"
        self._draw_segments = value

    @builtins.property
    def coords(self):
        """Message field 'coords'."""
        return self._coords

    @coords.setter
    def coords(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'coords' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'coords' field must be an integer in [-128, 127]"
        self._coords = value


class Metaclass_GetGuiSettings(type):
    """Metaclass of service 'GetGuiSettings'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('whycode_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'whycode_interfaces.srv.GetGuiSettings')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_gui_settings

            from whycode_interfaces.srv import _get_gui_settings
            if _get_gui_settings.Metaclass_GetGuiSettings_Request._TYPE_SUPPORT is None:
                _get_gui_settings.Metaclass_GetGuiSettings_Request.__import_type_support__()
            if _get_gui_settings.Metaclass_GetGuiSettings_Response._TYPE_SUPPORT is None:
                _get_gui_settings.Metaclass_GetGuiSettings_Response.__import_type_support__()


class GetGuiSettings(metaclass=Metaclass_GetGuiSettings):
    from whycode_interfaces.srv._get_gui_settings import GetGuiSettings_Request as Request
    from whycode_interfaces.srv._get_gui_settings import GetGuiSettings_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
