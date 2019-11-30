fn main() {
    println!("Hello, 03a!");
}
/*
#!/usr/env/bin python

import numpy as np

LENGTH = 1000
WIDTH = 1000

def main(file):
    with open(file) as f:
        fabric = np.zeros([LENGTH,WIDTH])
        for l in f.readlines():
            claim_spec = l.split(' @ ')[1:][0]
            xy, ab = claim_spec.split(': ')
            x, y = xy.split(',')
            a, b = ab.split('x')
            for xi in range(int(x), int(x)+int(a)):
                for yi in range(int(y), int(y)+int(b)):
                    fabric[xi,yi] += 1
    claimed_squares = 0
    for xi in range(LENGTH):
        for yi in range(WIDTH):
            if fabric[xi,yi] > 1:
                claimed_squares += 1
    return claimed_squares

if __name__ == '__main__':
    print(main('../inputs/03'))
*/
