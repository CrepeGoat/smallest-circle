from itertools import zip_longest


class Point:
	__slots__ = ('axes',)

	def __init__(self, axes):
		self.axes = tuple(axes)

	def __add__(self, vector):
		return Point(pi + vi for pi, vi in zip_longest(self.axes, vector.axes))

	def __sub__(self, point):
		return Vector(p1i - p0i for p1i, p0i in zip_longest(self.axes, point.axes))

class Vector:
	__slots__ = ('axes',)

	def __init__(self, axes):
		self.axes = tuple(axes)

	def __add__(self, vector):
		return Vector(pi + vi for pi, vi in zip_longest(self.axes, vector.axes))

	def __sub__(self, vector):
		return Vector(p1i - p0i for p1i, p0i in zip_longest(self.axes, vector.axes))

