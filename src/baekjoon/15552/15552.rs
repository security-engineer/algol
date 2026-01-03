/*
입력
첫 줄에 테스트케이스의 개수 T가 주어진다. T는 최대 1,000,000이다. 다음 T줄에는 각각 두 정수 A와 B가 주어진다. A와 B는 1 이상, 1,000 이하이다.

출력
각 테스트케이스마다 A+B를 한 줄에 하나씩 순서대로 출력한다.

[예제 입력]
5
1 1
12 34
5 500
40 60
1000 1000

[예제 출력]
2
46
505
100
2000
*/

use std::io::{self, Write};

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // 첫 줄 읽기
    reader.read_line(&mut buf).unwrap();
    let count: i32 = buf.trim().parse().unwrap();
    buf.clear();

    // 출력을 위한 BufWriter (속도 핵심)
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    for _ in 0..count {
        reader.read_line(&mut buf).unwrap();
        
        // split_whitespace 사용 (공백 여러 개도 처리 가능)
        let mut nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        
        // 버퍼에 바로 쓰기
        writeln!(writer, "{}", a + b).unwrap();
        buf.clear();
    }
}