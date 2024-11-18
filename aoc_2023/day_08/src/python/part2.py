import sys, re

def main():
    curr_path = '../input_part2.txt'
    # lines = open(curr_path + 'input.txt', 'r').readlines()
    lines = sys.stdin.readlines()
    
    navigation = lines[0].strip()
    network = {}
    starting_nodes = []
    for i in range(2, len(lines)):
        node, left, right = re.findall(r"(\w{3})", lines[i])
        network[node] = (left, right)
        if node.endswith('A'):
            starting_nodes.append(node)
    

    # observe each node reach their target cyclycally (in multiples)
    # I claim the answer would be the MCM between those numbers
    distances = [0] * len(starting_nodes)

    # simulation
    N, steps = len(navigation), 0
    while True:
        end_nodes = []
        for node in starting_nodes:
            direction = navigation[steps % N]
            left, right = network[node]
            node = left if direction == 'L' else right
            end_nodes.append(node)

        steps += 1
        for i in range(len(end_nodes)):
            if end_nodes[i].endswith('Z') and distances[i] == 0:
                distances[i] = steps

        # until we know when each node reach their target
        if all(el != 0 for el in distances): break
        starting_nodes = end_nodes[:]

        
    answer = 1
    for steps in distances:
        answer = mcm(answer, steps)
    print(int(answer))


# ===== HELPERS ===== #
def mcd(a, b):
    tmp = 0
    while b != 0:
        tmp = b
        b = a % b
        a = tmp
    return a

def mcm(a, b):
    return (a * b) / mcd(a, b)


if __name__ == '__main__':
    main()
