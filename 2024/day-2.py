
if __name__ == "__main__":
    
    with open("./inputs/input-2.txt", "r", encoding="utf8") as file:
        lines = file.readlines()

    reports: list[list[int]] = []

    for line in lines:
        raw_values = line.strip().split(" ")
        values = []
        for val in raw_values:
            values.append(int(val))
        reports.append(values)

    part_one_total = 0

    for report in reports:
        prev = None
        for val in report:
            if prev == None:
                prev = val
                continue
            
            diff = prev - val

            if diff == 0:
                continue
            
            

    print(part_one_total)
    