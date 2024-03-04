
for i in range(256):
    if i % 16 == 0:
        print('// %r..%r' % (chr(i), chr(i + 15)))
    c = chr(i)

    if c >= 'A' and c <= 'Z':
        print('Value(%d),' % (i - ord('A')))
    elif c >= 'a' and c <= 'z': 
        print('Value(%d),' % (i - ord('a') + 26))
    elif c >= '0' and c <= '9': 
        print('Value(%d),' % (i - ord('0') + 52))
    elif c == '+':
        print('Value(%d),' % 62)
    elif c == '/':
        print('Value(%d),' % 63)
    elif c == '=':
        print('Pad,')
    elif c in '\n\t\r':
        print('Skip,')
    else:
        print('Invalid,')
