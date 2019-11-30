fn main() {
    println!("Hello, 03b!");
}
/*
#!/usr/env/bin python

import numpy as np

LENGTH = 1000
WIDTH = 1000

def main(file):
    with open(file) as f:
        fabric = np.zeros([LENGTH,WIDTH])
        claims = []
        for l in f.readlines():
            claim, claim_spec = l.split(' @ ')
            xy, ab = claim_spec.split(': ')
            x, y = xy.split(',')
            a, b = ab.split('x')
            claims.append(dict(claim_num = claim, x = int(x), y = int(y), a = int(a), b = int(b)))
            for xi in range(int(x), int(x)+int(a)):
                for yi in range(int(y), int(y)+int(b)):
                    fabric[xi,yi] += 1
        for c in claims:
            conflict = False
            for x in range(c['x'], c['x'] + c['a']):
                for y in range(c['y'], c['y'] + c['b']):
                    if fabric[x,y] != 1:
                        # This should really be followed by the next claim, but Python doesn't have an elegant way to
                        # break/continue an inner to an outer
                        conflict = True
            if conflict is False:
                return c['claim_num']

if __name__ == '__main__':
    print(main('../inputs/03'))
*/
