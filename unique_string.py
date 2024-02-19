def isUnique(str):
    check = 0
    for ch in str:
        diff = ord(ch) - ord("a")
        if (check >> diff) & 1 == 1:
            return False
        else:
            check = check | (1 << diff)
    return True

def main():
    strs = ["abc", "", "abcc"]
    for str in strs:
        print(isUnique(str))


main()
