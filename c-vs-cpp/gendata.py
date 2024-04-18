#!/usr/bin/env python

import random
import string

MIN_LEN=1
MAX_LEN=8
NWORDS=6000000

f = open("test-data.txt", "w")
for i in xrange(NWORDS):
  f.write("".join(random.sample(string.ascii_letters, random.randint(MIN_LEN,MAX_LEN))))
  f.write(' ')
  
