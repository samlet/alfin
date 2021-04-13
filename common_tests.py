from alfin.common import Point
p = Point(13, 42)
print(p.x, p.y)

from alfin.hash_set import HashSet
set=HashSet()
set.insert(1)
set.insert(2)
print(set.contains(2))
print([s for s in set.iter()])

'''
>>> from alfin.hash_set import HashSet
>>> set=HashSet()
>>> set.insert("hello")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: argument 'value': 'str' object cannot be interpreted as an integer
>>> set.insert(1)
>>> set.insert(2)
>>> set.contains(2)
True
>>> [s for s in set]
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: 'builtins.HashSet' object is not iterable
>>> [s for s in set.iter()]
[2, 1]
>>> 
'''
