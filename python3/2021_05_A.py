from collections import defaultdict
import sys


class Line:

    def __init__(self, x1, y1, x2, y2):
        self._x1 = x1
        self._y1 = y1
        self._x2 = x2
        self._y2 = y2


    def is_horizontal(self):
        return self._y1 == self._y2


    def is_vertical(self):
        return self._x1 == self._x2


    def is_straight(self):
        return self.is_horizontal() or self.is_vertical()


    def points(self):
        results = []
        
        if self.is_horizontal():
            startx = min(self._x1, self._x2)
            endx = max(self._x1, self._x2)

            for i in range(startx, endx + 1):
                results.append((i, self._y1))

        if self.is_vertical():
            starty = min(self._y1, self._y2)
            endy = max(self._y1, self._y2)

            for j in range(starty, endy + 1):
                results.append((self._x1, j))

        return results
        

class Diagram:

    def __init__(self):
        self._grid = defaultdict(int)


    def add_line(self, line):
        for point in line.points():
            self._grid[point] += 1
        

    def count_high_points(self, limit):
        count = 0
        for value in self._grid.values():
            if value >= limit:
                count += 1
        return count
    

def parse(line):
    start, end = line.strip().split("->")
    x1, y1 = start.strip().split(",")
    x2, y2 = end.strip().split(",")

    x1 = int(x1)
    x2 = int(x2)
    y1 = int(y1)
    y2 = int(y2)
    return Line(x1, y1, x2, y2)


def solve(lines, high):
    grid = Diagram()
    for line in lines:
        grid.add_line(parse(line))

    return grid.count_high_points(high)
    

if __name__ == "__main__":
    lines = sys.stdin.readlines()
    print(solve(lines, 2))
    

def test_line_is_straight():
    line1 = Line(3, 7, 7, 7)
    assert line1.is_horizontal()
    assert line1.is_straight()

    line2 = Line(5, 9, 5, -13)
    assert line2.is_vertical()
    assert line2.is_straight()

    line3 = Line(1, 2, 3, 4)
    assert not line3.is_vertical()
    assert not line3.is_horizontal()
    assert not line3.is_straight()


def test_example():
    lines = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2"
    ]

    assert solve(lines, 2) == 5
