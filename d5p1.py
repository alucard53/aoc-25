with open('input') as file:
    input = file.read()
    ranges_end = input.find('\n\n')

    ranges_str = input[:ranges_end]
    ranges = []
    for range in ranges_str.split('\n'):
        ranges.append(list(map(int, range.split('-'))))

    ans = 0
    for n in map(int, input[ranges_end + 1:].strip().split('\n')):
        for l, r in ranges:
            if l <= n <= r:
                ans += 1
                break

    print(ans)
