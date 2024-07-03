# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMObject base class that all game objects should
derive from.
"""

from collections.abc import Iterator, Iterable
from typing import Any, NoReturn

from green_moon_2d.gm_context import GMContext
from green_moon_2d.gm_math import GMVec2D


class GMObject:
    """
    This base class should be derived from for all objects to work
    properly.
    """

    def __init__(self, name):
        self.name = name
        self.pos = GMVec2D()
        self.vel = GMVec2D()
        self.acc = GMVec2D()
        self.active = True
        self.visible = True
        self.draw_order = 0
        self.update_order = 0
        self.groups = set()
        self.properties = {}

    def update(self, context: GMContext) -> None:
        """
        This method is called once per frame, you should derive it if needed.
        It is used to update the internal state of the object.
        """
        pass

    def draw(self, context: GMContext) -> None:
        """
        This method is called once per frame, you should derive it if needed.
        It is used to draw the object.
        """
        pass

    def move(self) -> None:
        """
        This method can be called once per frame if needed.
        It moves this object according to the given velocity and acceleration.
        """
        self.vel += self.acc
        self.pos += self.vel

    def add_group(self, name: str) -> None:
        """
        Add adds this object to the given group.
        """
        self.groups.add(name)

    def remove_group(self, name: str) -> None:
        """
        Removes this object from the given group.
        """
        if name in self.groups:
            self.groups.remove(name)

    def in_group(self, name: str) -> bool:
        """
        Tests if this object is part of the given group.
        """
        return name in self.groups

    def clear_groups(self) -> None:
        """
        Removed this object from all the groups.
        """
        self.groups.clear()

    def set_property(self, name: str, val: Any):
        """
        Sets the property with the given name to the given value.
        """
        self.properties[name] = val

    def get_property(self, name: str) -> Any:
        """
        Return the property with the given name.
        Raise KeyError if the property doesn't exist.
        """
        return self.properties[name]

    def get_property_default(self, name: str, default: Any) -> Any:
        """
        Return the property with the given name, return default value
        if not found.
        """
        return self.properties.get(name, default)

    def has_property(self, name: str) -> bool:
        """
        Checks wether the property with the given name exists.
        """
        return name in self.properties

    def clear_properties(self) -> None:
        """
        Removes all custom properties from this object.
        """
        self.properties.clear()


class GMObjectManager:
    """
    This is a helper class that takes care of handling objects.
    You don't have to use it, but it can make things easier.
    """

    def __init__(self):
        self.objects = []

    def _object_not_found(self, method: str, name: str) -> NoReturn:
        """
        Internal error method. It's called when the object with the given
        name was not found. Raise KeyError.
        """
        raise KeyError(
            f"GMObjectManager.{method}(), object with that "
            "name not found: {name}"
        )

    def add(self, obj: GMObject | Iterable[GMObject]) -> None:
        """
        Adds one or more new object to the manager. The name must be unique
        and the object must be derived from GMObject.
        Raise KeyError if the name is already in use.
        """
        if isinstance(obj, GMObject):
            index = self.get_index(obj.name)

            if index is not None:
                raise KeyError(
                    "GMObjectManager.add(), object with name "
                    f"that already exists: {obj.name}"
                )

            self.objects.append(obj)
        elif isinstance(obj, Iterable):
            for item in obj:
                self.add(item)
        else:
            raise TypeError(
                "GMObjectManager.add(), new object must "
                "be a subclass of GMObject")

    def delete(self, name: str) -> None:
        """
        Remove the object with the given name.
        Raise KeyError if the object was not found.
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
        """
        index = None

        for i, o in enumerate(self.objects):
            if o.name == name:
                index = i
                break

        return index

    def get(self, name: str) -> GMObject:
        """
        Returns the object with the given name.
        Raise KeyError if the object was not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("get", name)

        return self.objects[index]

    def update(self, context: GMContext) -> None:
        """
        Updates all the avtive objects. Objects can be sorted by update_order.
        """
        for o in self.objects:
            if o.active:
                o.update(context)

    def draw(self, context: GMContext) -> None:
        """
        Draws all the visible objects. Objects can be sorted by draw_order.
        """
        for o in self.objects:
            if o.visible:
                o.draw(context)

    def add_group(self, name: str, group: str | list[str]) -> None:
        """
        Add the given object to the given group.
        Raise KeyError if the object was not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("add_group", name)
        else:
            if isinstance(group, str):
                self.objects[index].add_group(group)
            elif isinstance(group, list):
                for g in group:
                    self.objects[index].add_group(g)
            else:
                raise ValueError(
                    "GMObjectManager.add_group(), group must be "
                    f"string or list of strings for object {name}")

    def remove_group(self, name: str, group: str) -> None:
        """
        Rmove the given object to the given group.
        Raise KeyError if the object was not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("remove_group", name)
        else:
            self.objects[index].remove_group(group)

    def remove_group_from_all(self, group: str) -> None:
        """
        Remove the given group from all object.
        """
        for ob in self.objects:
            ob.remove_group(group)

    def clear_groups(self, name: str) -> None:
        """
        Rmove all groups from the given object.
        Raise KeyError if the object was not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("clear_groups", name)
        else:
            self.objects[index].clear_groups()

    def iter_group(self, group: str) -> Iterator[GMObject]:
        """
        Iterates over all the objects of the given group.
        """
        for o in self.objects:
            if o.in_group(group):
                yield o

    def apply_group(self, group: str, op) -> None:
        """
        Apply the given operation to all the object of the given group.
        """
        for o in self.iter_group(group):
            op(o)

    def collect_group(self, group: str, op) -> list[Any]:
        """
        Collects the return values of all the objects of the given groups
        after the operation is applied to the objects.
        """
        result = []
        for o in self.iter_group(group):
            result.append((o.name, op(o)))

        return result

    def set_property(self, name: str, property: str, val: Any) -> None:
        """
        Sets the property for the given object.
        Raise KeyError if the object is not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("set_property", name)
        else:
            self.objects[index].set_property(property, val)

    def get_property(self, name: str, property: str) -> Any:
        """
        Returns the property of the given object.
        Raise KeyError if the object in not found or if the property is
        not found.
        """
        index = self.get_index(name)

        if index is None:
            return self._object_not_found("get_property", name)
        else:
            return self.objects[index].get_property(property)

    def has_property(self, name: str, property: str) -> bool:
        """
        Checks wether the given object has the property.
        Raise KeyError if the object was not found.
        """
        index = self.get_index(name)

        if index is None:
            self._object_not_found("has_property", name)
            return False
        else:
            return property in self.objects[index].properties
