with open('input') as file:
    input = file.read()
    ranges_end = input.find('\n\n')

    ranges_str = input[:ranges_end]
    ranges = []
    for r in ranges_str.split('\n'):
        ranges.append(list(map(int, r.split('-'))))

    n = len(ranges)
    ranges.sort()

    squashed_ranges = []
    i = 0
    while i < n:
        start = i
        j = i + 1

        while j < n and ranges[j][0] <= ranges[start][1]:
            if ranges[j][1] > ranges[start][1]:
                start = j
            j += 1

        squashed_ranges.append([ranges[i][0], ranges[start][1]])
        i = j


    ans = 0

    for l, r in squashed_ranges:
        ans += (r - l) + 1

    print(ans)
