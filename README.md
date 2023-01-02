# sudoku_solver

Rust로 작성된 스도쿠를 생성하거나, 기존 퍼즐을 풀기 위한 프로그램

## 퍼즐 생성 방법

완성된 스도쿠를 다음과 같이 간단히 생성할 수 있습니다.

```rust
use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::solver::Solver;

fn main() {
    let mut t = Table::new_default_16(); // 16x16 스도쿠 구조. 9x9를 생성하기 위해선 Table::new_default_9()을 사용.
    let mut solver = Solver::new(&mut t);

    solver.set_random_seed(0); // 스도쿠의 랜덤 시드 고정. 이걸 빼면 무작위 스도쿠가 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
}
```

### 9x9 스도쿠 퍼즐 생성 출력 결과 예시

```
1       7       9       3       6       2       8       5       4
5       2       3       4       8       7       1       9       6
6       4       8       1       9       5       3       7       2
9       5       7       6       2       1       4       3       8
4       8       2       5       7       3       9       6       1
3       6       1       9       4       8       7       2       5
7       1       6       2       3       4       5       8       9
2       3       5       8       1       9       6       4       7
8       9       4       7       5       6       2       1       3
```

### 16x16 스도쿠 퍼즐 생성 출력 결과 예시

```
1       7       16      11      5       12      8       9       2       4       14      3       15      6       10      13
6       10      14      13      7       16      15      1       9       8       12      5       11      3       4       2
15      2       8       12      3       6       4       11      7       10      1       13      14      16      5       9
9       5       3       4       14      2       10      13      16      6       15      11      8       12      1       7
10      8       13      5       9       14      11      7       4       3       2       6       12      15      16      1
4       14      15      6       8       1       13      3       12      9       10      16      7       2       11      5
3       16      11      1       2       5       12      6       13      7       8       15      10      14      9       4
2       9       12      7       15      4       16      10      14      11      5       1       13      8       6       3
8       12      6       2       16      10      3       5       11      15      9       7       4       1       13      14
5       15      1       16      12      9       2       4       6       14      13      10      3       11      7       8
7       11      10      14      1       13      6       8       3       12      4       2       9       5       15      16
13      4       9       3       11      15      7       14      1       5       16      8       6       10      2       12
14      3       2       8       13      11      9       15      5       1       6       4       16      7       12      10
11      6       4       9       10      3       1       2       8       16      7       12      5       13      14      15
12      1       7       10      4       8       5       16      15      13      11      14      2       9       3       6
16      13      5       15      6       7       14      12      10      2       3       9       1       4       8       11
```


## 커스텀 스도쿠 생성방법

9x9 스도쿠의 경우 총 81개의 cell이 존재하며, 이러한 cell은 각각 복수개의 Zone에 속할 수 있습니다. 이러한 Zone은 스도쿠 퍼즐이 지켜야만 하는 제약조건을 나타냅니다.

예를들어 9x9 스도쿠에서는 가로, 세로, 3x3 칸 내에서 모두 중복이 없어야하기에 모든 cell은 3개의 Zone에 속하고 있습니다.

만약 여기서 대각선 중복 금지 규칙을 추가할 경우 일부 cell은 4개의 Zone을 가지게 되며, 이렇게 Zone을 수정하는 것으로 커스텀 스도쿠를 만들 수 있습니다.


### 직소 스도쿠 생성 예시

다음은 Zone을 커스터마이징하여 직소 스도쿠를 생성하는 예시입니다.

#### 생성할 직소 스도쿠의 모습
![직소](https://user-images.githubusercontent.com/104359503/208801798-49cba9ef-fd7e-4635-8caf-9a2e0576a614.png)

#### 직소 스도쿠 생성 코드 예시
```rust
use sudoku_solver_lib::model::cell::Cell;
use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::model::table_lock::TableLock;
use sudoku_solver_lib::model::zone::Zone;
use sudoku_solver_lib::solver::Solver;

pub fn new_jigsaw() -> TableLock<9> {
    let zone: [usize; 81] = [
        1, 1, 1, 1, 1, 2, 2, 2, 2, 4, 1, 1, 1, 3, 3, 2, 2, 2, 4, 4, 1, 3, 3, 3, 3, 2, 2, 4, 4, 4,
        5, 5, 3, 3, 3, 6, 4, 4, 5, 5, 5, 5, 5, 6, 6, 4, 7, 7, 7, 5, 5, 6, 6, 6, 8, 8, 7, 7, 7, 7,
        9, 6, 6, 8, 8, 8, 7, 7, 9, 9, 9, 6, 8, 8, 8, 8, 9, 9, 9, 9, 9,
    ]; // 직소 스도쿠의 모양

    let mut cells: Vec<Vec<Cell<9>>> = Vec::with_capacity(9);
    for x in 0..9 {
        let mut row: Vec<Cell<9>> = Vec::with_capacity(9);
        for y in 0..9 {
            let index = zone[x * 9 + y];

            let this_zone = vec![
                Zone::new_unique_from_num(index),  // cell이 속해있는 직소 모양
                Zone::new_unique_from_num(x + 10), // cell이 속한 세로
                Zone::new_unique_from_num(y + 19), // cell이 속한 가로
            ];
            let cell = Cell::new(x, y, this_zone);

            row.push(cell);
        }
        cells.push(row);
    }

    Table::new_with_vec_cells(cells)
}

fn main() {
    let mut t = new_jigsaw();
    let mut solver = Solver::new(&mut t);

    solver.set_random_seed(0); // 스도쿠의 랜덤 시드 고정. 이걸 빼면 무작위 스도쿠가 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
}
```

#### 직소 스도쿠 퍼즐 생성 출력 결과 예시

```
6       3       1       7       2       5       9       8       4
1       5       8       4       9       3       2       7       6
5       7       9       2       6       4       8       3       1
2       6       4       9       8       7       5       1       3
8       9       3       1       7       6       4       5       2
3       4       7       8       5       2       1       6       9
7       1       2       5       3       9       6       4       8
4       2       5       6       1       8       3       9       7
9       8       6       3       4       1       7       2       5
```


## 현재 구현 상태
* Single
* Naked Pair, Naked Triple, Naked Quad 등..
* Box Line Reduction
* 구현된 알고리즘으로 풀 수 없을 경우 무작위 Guess 및 Guess가 잘못되었을 경우 Rollback

## TODO
* 더 많은 Solver 알고리즘 구현
* 출력 결과물을 더 보기좋게 제공
* 더 빠른 속도
