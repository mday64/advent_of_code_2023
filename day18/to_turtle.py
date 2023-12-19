inp = open("input.txt").read().splitlines()
for line in inp:
    (heading, dist, _) = line.split()
    if heading == "U":
        print("t.setheading(90)")
    elif heading == "L":
        print("t.setheading(180)")
    elif heading == "D":
        print("t.setheading(270)")
    elif heading == "R":
        print("t.setheading(0)")
    else:
        print("### invalid heading ###")
    print(f"t.forward({dist})")
