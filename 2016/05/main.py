 #!/usr/bin/env python3

def part12(id):
    from hashlib import md5
    i, res1, res2 = 0, "", [" " for _ in range(8)]
    while " " in res2:
        hash = md5(f"{id}{i}".encode()).hexdigest()
        idx = int(hash[5], 16)
        if hash.startswith("00000"):
            if len(res1) < 8:
                res1 = res1 + hash[5]
            if idx < 8 and res2[idx] == " ":
                res2[idx] = hash[6]
        i += 1
    return res1, "".join(res2)


def main():
    id = "ffykfhsq"
    res1, res2 = part12(id)
    print(res1, res2)


if __name__ == "__main__":
    main()
