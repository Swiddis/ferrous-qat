with open('data/words.txt') as infile:
    m = map(lambda l: l.strip().lower(), infile)
    f = filter(lambda l: l.isalpha(), m)
    m2 = map(lambda l: l + "\n", f)
    with open('data/words_filtered.txt', 'w') as outfile:
        outfile.writelines(sorted(set(m2)))
