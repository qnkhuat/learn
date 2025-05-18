# Chapter 8.4, Local Value Numbering
import pprint
from enum import Enum, auto
from typing import Tuple
from dataclasses import dataclass, replace

class Op(Enum):
  ADD = auto()
  MUL = auto()
  CONST = auto()
  ASSIGN = auto()

@dataclass(frozen=True)
class UOp:
  op: Op
  src: Tuple
  argument: any
  def __repr__(self):
    if self.src is None:
      return f"({id(self)}: {self.op=}, {self.argument=})"
    else:
      return f"({id(self)}: {self.op=}, {[u for u in self.src]}, {self.argument=})"

  def __hash__(self):
    # Include operation type in hash for proper value numbering
    if self.src is None:
      return hash((self.op, self.argument))
    return hash((self.op, self.src))

  def __eq__(self, other):
    if not isinstance(other, UOp):
      return False
    if self.op != other.op:
      return False
    if self.src is None:
      return self.argument == other.argument
    return self.src == other.src

"""
a = 2
b = 3
c = a + b
d = a + b
e = a + d
"""
a = UOp(Op.CONST, None, (2,))
b = UOp(Op.CONST, None, (3,))
c = UOp(Op.ADD, (a, b), None)
d = UOp(Op.ADD, (a, b), None)
e = UOp(Op.ADD, (c, d), None)

def replace_tuple(t, i, new_v):
  l = list(t)
  l[i] = new_v
  return tuple(l)

def inv_dict(d): dict((v, k) for k, v in d.items())

def rewrite(u, current=0, val_to_uop={}, uop_to_val={}):
  # for each op, get value, if the value exists,
  # then replace it with the existing node
  if u.src is None:
    return u, current, val_to_uop, uop_to_val
  else:
    new_src = []
    for i, s in enumerate(u.src):
      [s, current, val_to_uop, uop_to_val] = rewrite(s, current, val_to_uop, uop_to_val)
      val = uop_to_val.get(s, None)
      if val is None:
        val = current
        uop_to_val[s] = val
        val_to_uop[val] = s
        current += 1  # Increment by 1 for clearer value numbers
      else:
        s = val_to_uop[val]
      new_src.append(s)
    new_src = tuple(new_src)
    u = replace(u, src=tuple(new_src))
    return [u, current, val_to_uop, uop_to_val]

pprint.pp(e)
pprint.pp(rewrite(e)[0])
