# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler
#
# See: https://github.com/willi-kappler/green_moon_2d

"""
This module defines the GMObject base class that all game objects should derive from.
"""

from gm_context import GMContext

class GMObject:
    """
    This base class should be derived from for all objects to work properly.
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
        self.property = {}

    def update(self, context: GMContext) -> None:
        pass

    def draw(self, context: GMContext) -> None:
        pass

    def move(self) -> None:
        self.vel += self.acc
        self.pos += self.vec

    def add_group(self, name: str) -> None:
        self.groups.add(name)

    def remove_group(self, name: str) -> None:
        self.groups.remove(name)

    def in_group(self, name: str) -> bool:
        return name in self.groups

    def clear_groups(self) -> None:
        self.groups.clear()

class GMObjectManager:
    """
    This is a helper class that takes care of handling objects.
    You don't have to use it, but it can make things easier.
    """

    def __init__(self):
        self.objects = []

    def object_not_found(self, method: str, name: str) -> None:
        raise KeyError(f"GMObjectManager.{method}(), object with that name not found: {name}")

    def add_object(self, object: GMObject) -> None:
        assert isinstance(scene, GMObject), "GMObjectManager.add_object(), new object must be a subclass of GMObject"

        index = self.get_index(object.name)

        if index is not None:
            raise KeyError(f"GMObjectManager.add_object(), object with that already exists: {object.name}")

        self.objects.append(object)

    def delete(self, name: str) -> None:
        index = self.get_index(name)

        if index is None:
            self.object_not_found("delete", name)
        else:
            del self.objects[index]

    def sort_update(self) -> None:
        self.objects.sort(key=lambda o: o.update_order)

    def sort_draw(self) -> None:
        self.objects.sort(key=lambda o: o.draw_order)

    def get_index(self, name: str) -> None:
        index = None

        for i, o in enumerate(self.objects):
            if o.name == name:
                index = i
                break

        return index

    def get(self, name: str) -> None:
        index = self.get_index(name)

        if index is None:
            self.object_not_found("get", name)
        else:
            return self.objects[i]

    def update(self, context: GMContext) -> None:
        for o in self.objects:
            if o.active:
                o.update(context)

    def draw(self, context: GMContext) -> None:
        for o in self.objects:
            if o.visible:
                o.draw(context)

    def add_group(self, name: str, group: str) -> None:
        index = self.get_index(name)

        if index is None:
            self.object_not_found("add_group", name)
        else:
            self.objects[index].add_group(group)

    def remove_group(self, name: str, group: str) -> None:
        index = self.get_index(name)

        if index is None:
            self.object_not_found("remove_group", name)
        else:
            self.objects[index].remove_group(group)

    def clear_groups(self, name: str) -> None:
        index = self.get_index(name)

        if index is None:
            self.object_not_found("remove_group", name)
        else:
            self.objects[index].clear_groups()

    def iter_group(self, group: str) -> Iterator[GMObject]:
        for o in self.objects:
            if o.in_group(group):
                yield o

    def apply_group(self, group: str, op) -> None:
        for o in self.iter_group(group):
            op(o)

    def collect_group(self, group: str, op):
        result = []
        for o in self.iter_group(group):
            result.append((o.name, op(o)))

        return result

