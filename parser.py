with open("input.text", "r") as f:
    lines = f.readlines()
    a = ""
    b = ""
    for line in lines:
        if len(line.split()) == 2:
            c, d = line.split()
            a+= f"{c},"
            b+= f"{d},"
    
    print(a)

    print("********")
    print(b)
