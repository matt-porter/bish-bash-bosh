import os

import string

letters = zip(string.ascii_uppercase, string.ascii_lowercase)

for l in string.ascii_uppercase:
    print('("{}", "alpha-{}.wav"),'.format(l, l.lower()))
    

# for i, pair in enumerate(letters):
#     for j, case in enumerate(pair):
#         idx = i * 2 + j + 1
#         letter = case
#         print(idx, letter)
#         fname = 'sounds/alpha--{:02}.wav'.format(idx)
#         print(fname)
#         if os.path.exists(fname):
#             os.rename(fname, 'sounds/alpha--{}.wav'.format(letter))
