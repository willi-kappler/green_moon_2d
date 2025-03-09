# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMObject base class that all game objects should derive from.
"""

from collections.abc import Iterator, Iterable
from typing import Any, NoReturn

from green_moon_2d.gm_math import GMVec2D
from green_moon_2d.gm_message import GMMessage, GMMessageType

import logging
logger = logging.getLogger(__name__)


class GMObject:
    """
    This base class should be derived from for all objects to work properly.
    """

    def __init__(self, name):
        """
        :param name: The name of this object. It must be unique.
        """

        self.name: str = name
        self.object_manager: GMObjectManager
        self.pos: GMVec2D = GMVec2D(0.0, 0.0)
        self.active: bool = True
        self.visible: bool = True
        self.draw_order: int = 0
        self.update_order: int = 0
        self.groups: set[str] = set()
        self.properties: dict[str, Any] = {}

    def update(self, dt: float, om) -> None:
        """
        This method is called once per frame, you should implement it if needed.
        It is used to update the internal state of the object.
        :param :dt The time in ms since the previous frame.
        """

        pass

    def draw(self) -> None:
        """
        This method is called once per frame, you should implement it if needed.
        It is used to draw the object.
        """

        pass

    def send_message(self, msg: GMMessage) -> Any:
        """
        This method can be implemented to send a custom message to the object.

        :param msg: The message that is send to this object.
        :return: A possible response from the object.
        :rtype: Any
        """

        command = msg.command
        value = msg.value
        msg2 = (command, value)

        match msg2:
            # Properties:
            case ("set_pos", pos):
                self.set_pos(pos)
            case ("get_pos", _):
                return self.pos
            case ("set_x", float(x)):
                self.pos.x = x
            case ("get_x", _):
                return self.pos.x
            case ("set_y", float(y)):
                self.pos.y = y
            case ("get_y", _):
                return self.pos.y
            case ("set_active", bool(active)):
                self.active = active
            case ("toggle_active", _):
                self.active = not self.active
            case ("set_visible", bool(visible)):
                self.visible = visible
            case ("toggle_visible", _):
                self.visible = not self.visible
            case ("set_draw_order", int(order)):
                self.draw_order = order
            case ("set_update_order", int(order)):
                self.update_order = order
            # Methods:
            case ("add_group", str(name)):
                self.add_group(name)
            case ("add_groups", list(groups)):
                self.add_groups(groups)
            case ("remove_group", str(name)):
                self.remove_group(name)
            case ("in_group", str(name)):
                return self.in_group(name)
            case ("clear_groups", _):
                self.clear_groups()
            case ("set_property", (str(name), val)):
                self.set_property(name, val)
            case ("set_properties", properties):
                self.set_properties(properties)
            case ("get_property", str(name)):
                return self.get_property(name)
            case ("remove_property", str(name)):
                return self.remove_property(name)
            case ("get_property_default", (str(name), default)):
                return self.get_property_default(name, default)
            case ("has_property", str(name)):
                return self.has_property(name)
            case ("clear_properties", _):
                self.clear_properties()

    def set_pos(self, pos: tuple[float, float] | GMVec2D) -> None:
        """
        Sets the position of this object.
        :param pos: Can be a tuple of floats or a GMVec2D.
        """

        match pos:
            case GMVec2D():
                self.pos = pos
            case (float(x), float(y)):
                self.pos.x = x
                self.pos.y = y

    def add_group(self, name: str) -> None:
        """
        Add adds this object to the given group.

        :param name: The name of the group.
        """

        self.groups.add(name)

    def add_groups(self, groups: list[str]) -> None:
        """
        Add adds this object to the given groups.

        :param groups: A list of group names.
        """

        for name in groups:
            self.groups.add(name)

    def remove_group(self, name: str) -> None:
        """
        Removes this object from the given group.

        :param name: The name of the group to be removed.
        """

        if name in self.groups:
            self.groups.remove(name)

    def in_group(self, name: str) -> bool:
        """
        Tests if this object is part of the given group.

        :param name: The name of the group.
        :return: True if the object is in the group otherwise False.
        :rtype: bool
        """

        return name in self.groups

    def clear_groups(self) -> None:
        """
        Removed this object from all the groups.
        """

        self.groups.clear()

    def set_property(self, name: str, val: Any) -> None:
        """
        Sets the property with the given name to the given value.

        :param name: The name of the property.
        :param val: The value of the property.
        """

        self.properties[name] = val

    def set_properties(self, properties: Iterable[tuple[str, Any]]) -> None:
        """
        Sets multiple properties at once.

        :param properties: The properties to set.
        """

        for name, val in properties:
            self.properties[name] = val

    def get_property(self, name: str) -> Any:
        """
        Return the property with the given name.

        :param name: The name of the property.
        :return: The value of the property.
        :rtype: Any
        """

        return self.properties[name]

    def remove_property(self, name: str) -> Any:
        """
        Removes the property with the given name.

        :param name: The name of the property.
        :raise KeyError: If the property with that name doesn't exist.
        """

        del self.properties[name]

    def get_property_default(self, name: str, default: Any) -> Any:
        """
        Return the property with the given name, return default value if not found.

        :param name: The name of the property.
        :param default: The default value if the property is not available.
        :return: The value of the given property.
        :rtype: Any
        """

        return self.properties.get(name, default)

    def has_property(self, name: str) -> bool:
        """
        Checks wether the property with the given name exists.

        :param name: The name of the property.
        :return: True if the property with the given name was found, otherwise False.
        :rtype: bool
        """

        return name in self.properties

    def clear_properties(self) -> None:
        """
        Removes all custom properties from this object.
        """

        self.properties.clear()


class GMObjectManager(Iterable):
    """
    This is a helper class that takes care of handling objects.
    You don't have to use it, but it can make things easier.
    """

    def __init__(self):
        logger.debug("Create a new GMObjectManager.")

        self.objects = []

    def _object_not_found(self, method: str, name: str) -> NoReturn:
        """
        Internal error method. It's called when the object with the given
        name was not found.

        :param method: The name of the method where the object was not found.
        :param name: The name of the object that was not found.
        :raise KeyError: when the object was not found.
        """

        raise KeyError(
            f"GMObjectManager.{method}(), object with that name not found: {name}"
        )

    def add(self, obj: GMObject | Iterable[GMObject]) -> None:
        """
        Adds one or more new object to the manager. The name must be unique
        and the object must be derived from GMObject.

        :param obj: The new object to be added.
        :raise KeyError: if the name is already in use.
        """

        match obj:
            case GMObject():
                index = self.get_index(obj.name)

                if index is not None:
                    raise KeyError(
                        "GMObjectManager.add(), object with name "
                        f"that already exists: {obj.name}"
                    )

                obj.object_manager = self
                self.objects.append(obj)
            case Iterable():
                for item in obj:
                    self.add(item)
            case _:
                raise TypeError(
                    "GMObjectManager.add(), new object must be a subclass of GMObject")

    def delete(self, name: str) -> None:
        """
        Remove the object with the given name.

        :param name: The name of the object that should be deleted.
        :raise KeyError: if the object was not found.
        """

        index = self.get_index(name)

        if index is None:
            self._object_not_found("delete", name)
        else:
            del self.objects[index]

    def sort_update(self) -> None:
        """
        Sorts all object according to the update_order.
        """

        self.objects.sort(key=lambda o: o.update_order)

    def sort_draw(self) -> None:
        """
        Sorts all object according to the draw_order.
        """

        self.objects.sort(key=lambda o: o.draw_order)

    def get_index(self, name: str) -> int | None:
        """
        Returns the index of the object with the given name.
        Returns None if the object was not found.

        :param name: The name of the object.
        :return: The index of the object if it was found, otherwise None
        """

        index = None

        for i, o in enumerate(self.objects):
            if o.name == name:
                index = i
                break

        return index

    def __getitem__(self, name: str) -> GMObject:
        """
        Returns the object with the given name.

        :param name: The name of the object.
        :return: The object with the given name.
        :rtype: GMObject
        :raise KeyError: if the object was not found.
        """

        index = self.get_index(name)

        if index is None:
            self._object_not_found("get", name)

        return self.objects[index]

    def update(self, dt: float) -> None:
        """
        Updates all the active objects. Objects can be sorted by update_order.
        See sort_update() method.

        :param context: The current game context.
        """

        for o in self.objects:
            if o.active:
                o.update(dt, self)

    def draw(self) -> None:
        """
        Draws all the visible objects. Objects can be sorted by draw_order.
        See sort_draw() method.

        :param context: The current game context.
        """

        for o in self.objects:
            if o.visible:
                o.draw()

    def send_message(self, msg: GMMessage) -> list[tuple[str, Any]]:
        """
        Sends a message to the given object.

        :param name: The name of the object.
        :param msg: The message that will be sent to the object.
        :return: A possible response from the object.
        :rtype: Any
        :raise KeyError: if the object was not found.
        """

        result: list = []

        match msg.msg_type:
            case GMMessageType.SINGLE | GMMessageType.MULTIPLE:
                for name in msg.targets:
                    result.append((name, self[name].send_message(msg)))

            case GMMessageType.GROUP | GMMessageType.MULTI_GROUP:
                for group in msg.targets:
                    result.extend(self.send_message_group(group, msg))

            case _:
                raise ValueError(f"Unknown message type: {msg}")

        return result

    def send_message_group(self, group: str, msg: GMMessage) -> list[tuple[str, Any]]:
        """
        Sends a message to all objects of the given group.

        :param group: The name of the group.
        :param msg: The message to be sent to the group.
        """

        results = []

        for ob in self.objects:
            if ob.in_group(group):
                results.append((ob.name, ob.send_message(msg)))

        return results

    def add_group(self, name: str, group: str) -> None:
        """
        Add the given object to the given group.

        :param name: The name of the object.
        :param group: The name of the group.
        :raise KeyError: if the object was not found.
        """

        ob = self[name]
        ob.add_group(group)

    def add_groups(self, name: str, groups: list[str]) -> None:
        """
        Add the given object to the given groups.

        :param name: The name of the object.
        :param groups: A list of group names.
        :raise KeyError: if the object was not found.
        """

        ob = self[name]
        ob.add_groups(groups)

    def remove_group(self, name: str, group: str) -> None:
        """
        Remove the given object from the given group.

        :param name: The name of the object.
        :param group: The group from which the object should be removed.
        :raise KeyError: if the objectr was not found.
        """

        self[name].remove_group(group)

    def remove_group_from_all(self, group: str) -> None:
        """
        Remove the given group from all object.

        :param group: The group that is going to be removed from all objects.
        """

        for ob in self.objects:
            ob.remove_group(group)

    def clear_groups(self, name: str) -> None:
        """
        Remove all groups from the given object.

        :param name: The name of the object.
        :raise KeyError: if the object was not found.
        """

        self[name].clear_groups()

    def __iter__(self) -> Iterator[GMObject]:
        """
        Returns an iterator for all objects.

        :return: An iterator over all objects.
        :rtype: Iterator[GMObject]
        """

        return iter(self.objects)

    def iter_group(self, group: str) -> Iterator[GMObject]:
        """
        Returns an iterator for all the objects of the given group.

        :param group: The group to which the objects belong.
        :return: An iterator for all object in the given group.
        :rtype: Iterator[GMObject]
        """

        for o in self.objects:
            if o.in_group(group):
                yield o

    def apply_group(self, group: str, op) -> None:
        """
        Apply the given operation to all the object of the given group.

        :param group: The group to which the objects belong.
        :param op: The operation to apply to the objects individually.
        """

        for o in self.iter_group(group):
            op(o)

    def collect_group(self, group: str, op) -> list[tuple[str, Any]]:
        """
        Collects the return values of all the objects of the given groups
        after the operation is applied to the objects.

        :param group: The group to which the objects belong.
        :param op: The operation to apply to the objects individually.
        :return: A list of tuples with the name of the object and the return value.
        :rtype: list[tuple[str, Any]]
        """

        result = []
        for o in self.iter_group(group):
            result.append((o.name, op(o)))

        return result

    def set_property(self, name: str, property: str, val: Any) -> None:
        """
        Sets the property for the given object.

        :param name: The name of the object.
        :param property: The name of the property.
        :param val: The value of the property.
        :raise KeyError: If the object was not found.
        """

        self[name].set_property(property, val)

    def set_property_group(self, group: str, property: str, val: Any) -> None:
        """
        Sets the property for all objects of the given group.

        :param group: The name of the group.
        :param property: The name of the property.
        :param val: The value of the property.
        :raise KeyError: If the object was not found.
        """

        for o in self.iter_group(group):
            o.set_property(property, val)

    def set_properties(self, name: str, properties: Iterable[tuple[str, Any]]) -> None:
        """
        Sets multiple properties at once.

        :param name: The name of the object.
        :param properties: The properties to set.
        :raise KeyError: If the object was not found.
        """

        self[name].set_properties(properties)

    def set_properties_group(self, group: str, properties: Iterable[tuple[str, Any]]) -> None:
        """
        Sets multiple properties at once for all objects of the given group.

        :param group: The name of the group.
        :param properties: The properties to set.
        :raise KeyError: If the object was not found.
        """

        for o in self.iter_group(group):
            o.set_properties(properties)

    def get_property(self, name: str, property: str) -> Any:
        """
        Returns the property of the given object.

        :param name: The name of the object.
        :param property: The name of the property.
        :return: The value of the given property for the given object.
        :rtype: Any
        :raise KeyError: If the object or the property was not found.
        """

        return self[name].get_property(property)

    def remove_property(self, name: str, property: str) -> None:
        """
        Removes the property from the given object.
        :param name: The name of the object.
        :param property: The name of the property.
        :raise KeyError: If the object or the property is not found.
        """

        self[name].remove_property(property)

    def remove_property_group(self, group: str, property: str) -> None:
        """
        Removes the property from all objects of the given group.
        :param group: The name of the group.
        :param property: The name of the property.
        :raise KeyError: If the property was not found.
        """

        for o in self.objects:
            if o.in_group(group):
                o.remove_property(property)

    def has_property(self, name: str, property: str) -> bool:
        """
        Checks if the given object has the property.

        :param name: The name of the object.
        :param property: The name of the property.
        :return: True if the object has the property otherwise False.
        :rtype: bool
        :raise KeyError: if the object or the property was not found.
        """

        return property in self[name].properties

    def clear_properties(self, name: str) -> None:
        """
        Removes all properties from the given object.

        :param name: The name of the object to remove all the properties from.
        :raise KeyError: if the object was not found.
        """

        self[name].clear_properties()

    def clear_properties_group(self, group: str) -> None:
        """
        Clears all the properties from all the objects in the given group.
        """

        for o in self.objects:
            if o.in_group(group):
                o.clear_properties()

