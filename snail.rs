/*输出一个4X3矩阵,沿上下左右前进,每个格子都走一遍的走法数量,
例如下面是一些合法的走法:

1  2  3  4
10 11 12 5 
9  8  7  6		

1 2 9 10
4 3 8 11 
5 6 7 12
*/

static mut total: u32 = 0;

fn up(x: i32,y: i32) -> (bool,i32,i32) {
	if y -1 < 0 {
		(false,0,0)
	}else{
		(true,x,y-1)
	}
}

fn down(x: i32,y: i32) -> (bool,i32,i32) {	
	if y + 1 >= 3 {
		(false,0,0)
	}else{
		(true,x,y+1)
	}
}

fn left(x: i32,y: i32) -> (bool,i32,i32) {	
	if x - 1 < 0 {
		(false,0,0)
	}else{
		(true,x-1,y)
	}
}

fn right(x: i32,y: i32) -> (bool,i32,i32) {	
	if x + 1 >= 4 {
		(false,0,0)
	}else{
		(true,x+1,y)
	}
}

static DIR: [fn(i32,i32) -> (bool,i32,i32); 4] = [up, down,left,right];

fn is_vaild(matrix: [[i32; 4]; 3],x: i32,y: i32) -> bool {
	if x < 0 || y < 0  || x >= 4 || y >= 3 {
		return false;
	}
	if matrix[y as usize][x as usize] > 0 {
		return false;
	}
	return true;
}
	
fn snail(mut matrix: [[i32; 4]; 3],mut c: i32,x: i32,y: i32) {
	if is_vaild(matrix,x,y) {
		c = c + 1;
		matrix[y as usize][x as usize] = c;
		if c == 12 {
			unsafe { total = total + 1; }
			return;
		}
		for i in 0..4 {
			let (vaild,nextx,nexty) = DIR[i](x,y);
			if vaild {
				snail(matrix,c,nextx,nexty);
			}
		}		
	}
}


fn main() {
	let matrix: [[i32; 4]; 3] = [[0,0,0,0],[0,0,0,0],[0,0,0,0]];
	for j in 0..3 {
		for i in 0..4 {
			snail(matrix,0,i,j);
		}
	}
	unsafe{ println!("{}",total); }
}


