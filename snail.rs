/*local maxx = 4
local maxy = 3
local final = maxx * maxy

local dir = {
	up = function (x,y)
		y = y - 1
		if y < 1 then
			return nil
		end
		return x,y
	end, 
	down = function (x,y)
		y = y + 1
		if y > maxy then
			return nil
		end
		return x,y
	end, 
	left  = function (x,y)
		x = x - 1
		if x < 1 then
			return nil
		end
		return x,y
	end,
	right = function (x,y)
		x = x + 1
		if x > maxx then
			return nil
		end
		return x,y
	end
}*/

fn up(x: i32,y: i32) -> (bool,i32,i32) {
	if y -1 < 0 {
		(false,0,0)
	}else{
		(true,x,y-1)
	}
}

fn down(x: i32,y: i32) -> (bool,i32,i32) {	
	if y + 1 >= 4 {
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


fn main() {
	let (ret,x,y) = up(1,2);
	if ret {
		println!("{},{}",x,y);
	}
}


