

if __name__ == "__main__":
    
    with open("./inputs/input-1.txt", "r", encoding="utf8") as file:
        lines = file.readlines()

    left_list = []
    right_list = []

    occurance_map = {}
        
    for line in lines:
        split = line.strip().split(" ")

        left_val = int(split[0])
        right_val = int(split[3])

        left_list.append(left_val)
        right_list.append(right_val)

        if occurance_map.get(right_val, False):
            occurance_map[right_val] += 1
        else:
            occurance_map[right_val] = 1

    
    left_list.sort()
    right_list.sort()

    part_one_total = 0
    part_two_result = 0

    for i, left_item in enumerate(left_list):
        result = abs(left_item - right_list[i])
        occurances = occurance_map.get(left_item, 0)
        part_two_result += left_item * occurances
        part_one_total += result

    print(part_one_total)
    print(part_two_result)