import sys
import re
from copy import deepcopy
from math import gcd
from collections import defaultdict, Counter, deque
D = open(sys.argv[1]).read().strip()
L = D.split('\n')
G = [[c for c in row] for row in L]

# i == current position within dots
# bi == current position within blocks
# current == length of current block of '#'
# state space is len(dots) * len(blocks) * len(dots)
DP = {}
def f(dots, blocks, i, bi, current):
  key = (i, bi, current)
  if key in DP:
    return DP[key]
  if i==len(dots):
    if bi==len(blocks) and current==0:
      return 1
    elif bi==len(blocks)-1 and blocks[bi]==current:
      return 1
    else:
      return 0
  ans = 0
  for c in ['.', '#']:
    if dots[i]==c or dots[i]=='?':
      if c=='.' and current==0:
        ans += f(dots, blocks, i+1, bi, 0)
      elif c=='.' and current>0 and bi<len(blocks) and blocks[bi]==current:
        ans += f(dots, blocks, i+1, bi+1, 0)
      elif c=='#':
        ans += f(dots, blocks, i+1, bi, current+1)
  DP[key] = ans
  return ans
