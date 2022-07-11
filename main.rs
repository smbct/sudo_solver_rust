
use std::fs;

// load a given grid
fn load_grid(grid: &mut [i32; 9*9], grid_index: usize) {

	let contents = fs::read_to_string("p096_sudoku.txt")
				.expect("Error opening the file!");
	
	let v: Vec<&str> = contents.split('\n').collect();
  
  let grid_ascii = &v[(grid_index*10+1)..(grid_index*10+9)];
	
	let mut row_index = 0;
	for x in grid_ascii.iter() {
		let mut col_index = 0;  	
  	for c in x.chars() {
  		grid[row_index*9+col_index] = c.to_digit(10).unwrap() as i32;
  		col_index += 1;
  	}
  	row_index = row_index+1;
  }
  
}


// print a grid
fn print_grid(grid: &[i32; 9*9]) {

	for row_index in 0..9 {
		for col_index in 0..9 {
			print!("{}", grid[row_index*9+col_index]);
			if col_index % 3 == 2 {
				print!(" ");
			}
		}
		println!("");
		if row_index % 3 == 2 {
			println!("");
		}
	}

}

fn create_grid_sol(grid: &[i32; 9*9], grid_sol: &mut [Vec<i32>; 9*9]) {

	// fill in the gridSol
  for row_index in 0..9 {
		for col_index in 0..9 {
			if grid[row_index*9+col_index] == 0 {
				grid_sol[row_index*9+col_index] = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
			}
		  else {
		  	let value = grid[row_index*9+col_index];
				grid_sol[row_index*9+col_index] = vec![value]; 
			}
		}
	}
	
}

fn print_grid_sol(grid_sol: &[Vec<i32>; 9*9]) {

	for i in 0..9 {
		for j in 0..9 {
			print!("({}, {}): ", i, j);
			for val in grid_sol[i*9+j].iter() {
				print!("{} ", val);
			}
			print!("\n");
		}
	}
	
}

// filter the content of the grid based on a modification
fn filter(grid_sol: &mut [Vec<i32>; 9*9], row_ind: i32, col_ind: i32) -> (bool,bool) {

	let i: usize = 0;
	let value = grid_sol[(row_ind*9+col_ind) as usize][i];

	let mut possible = true;
	let mut modification = false;

	// row
	for col_ind2 in 0..9 {
		if col_ind2 != col_ind {
			
			let len_before = grid_sol[(row_ind*9+col_ind2) as usize].len();
			 
			grid_sol[(row_ind*9+col_ind2) as usize].retain(|&x| x != value);
			if grid_sol[(row_ind*9+col_ind2) as usize].len() == 0 {
				possible = false;
			}
			
			if len_before != grid_sol[(row_ind*9+col_ind2) as usize].len() {
				modification = true;
			}
			
		}
	}
	
	// column
	for row_ind2 in 0..9 {
		if row_ind2 != row_ind {
		
			let len_before = grid_sol[(row_ind2*9+col_ind) as usize].len();
		
			grid_sol[(row_ind2*9+col_ind) as usize].retain(|&x| x != value);
			if grid_sol[(row_ind2*9+col_ind) as usize].len() == 0 {
				possible = false;
			}
			
			if len_before != grid_sol[(row_ind2*9+col_ind) as usize].len() {
				modification = true;
			}
		}
	}
	
	// cell
	let cell_row = row_ind/3;
	let cell_col = col_ind/3;
	
	for i in 0..3 {
		for j in 0..3 {
			
			let row_ind2 = cell_row*3+i;
			let col_ind2 = cell_col*3+j;
			
			if row_ind2 != row_ind || col_ind2 != col_ind {
			
				let len_before = grid_sol[(row_ind2*9+col_ind2) as usize].len();
			
				grid_sol[(row_ind2*9+col_ind2) as usize].retain(|&x| x != value);
				if grid_sol[(row_ind2*9+col_ind2) as usize].len() == 0 {
					possible = false;
				}
				
				if len_before != grid_sol[(row_ind2*9+col_ind2) as usize].len() {
					modification = true;
				}
				
			}
		}
	}
	
	(modification, possible)

}




fn solve_rec(grid_sol: &mut [Vec<i32>; 9*9], grid_sol_res: &mut [Vec<i32>; 9*9]) -> bool {
	
	let mut impossible = false;
	let mut modification = true;
	
	// println!("before filtering: ");
	// print_grid_sol(grid_sol);
	// println!("\n\n");
	
	while !impossible && modification {
		
		modification = false;
		
		'outer1: for row_ind in 0..9 {
			for col_ind in 0..9 {
				if grid_sol[(row_ind*9+col_ind) as usize].len() == 1 {
					
					let res = filter(grid_sol, row_ind, col_ind);		
					
					
					if res.0 {
						modification = true;
					}
					
					if !res.1 {
						impossible = true;
						break 'outer1
					}
					
				}
			}
		}
		
	}
	
	if !impossible {
	
		// look for an unasigned cell
		let mut found_row_ind = 0;
		let mut found_col_ind = 0;
		let mut found = false;
		'outer2: for row_ind in 0..9 {
			for col_ind in 0..9 {
				if grid_sol[(row_ind*9+col_ind) as usize].len() > 1 {
					found_row_ind = row_ind;
					found_col_ind = col_ind;
					found = true;
					break 'outer2
				}
			}
		}
	
		if found {
	
			
			for val in grid_sol[(found_row_ind*9+found_col_ind) as usize].iter() {
			
				let mut grid_sol_sub: [Vec<i32>; 9*9] = array_init::array_init(|_| vec![]);
				for ind in 0..9*9 {
					grid_sol_sub[ind] = grid_sol[ind].clone();
				}
				grid_sol_sub[(found_row_ind*9+found_col_ind) as usize] = vec![*val];
				
				let res = solve_rec(&mut grid_sol_sub, grid_sol_res);
				
				if res {
					return true;
				}
				
			}
	
			false
	
		} else {
	
			// println!("solved!!, copy the result to grid_sol_res");
			
			// copy the solution
			for ind in 0..9*9 {
				grid_sol_res[ind] = grid_sol[ind].clone();
			}
			
			true
			
		}
	
	} else {
	
		// println!("Impossible branch");
	
		false
	
	}

	

}


fn grid_sol_to_grid(grid_sol: &[Vec<i32>; 9*9], grid: &mut [i32; 9*9]) {

	for ind in 0..9*9 {
		grid[ind] = grid_sol[ind][0];
	}

}


fn main() {

	for ind_grid in 0..50 {
	
		let mut grid: [i32; 9*9] = [0; 9*9]; 
	
		load_grid(&mut grid, ind_grid);
  
  	println!("Initial grid: #{}\n", ind_grid);
  	print_grid(&grid);
  
  	// build the data structure for solving the grid
  	let mut grid_sol: [Vec<i32>; 9*9] = array_init::array_init(|_| vec![]);
  	let mut grid_sol_res: [Vec<i32>; 9*9] = array_init::array_init(|_| vec![]);
  
		create_grid_sol(&grid, &mut grid_sol);
 
 		let res = solve_rec(&mut grid_sol, &mut grid_sol_res);
	
		if res {
		
			// print_grid_sol(&grid_sol_res);
		
			grid_sol_to_grid(&grid_sol_res, &mut grid);
		
			println!("Final grid: \n");
			print_grid(&grid);
		
		} else {
			println!("no solution found :(");
		}
		
		println!("\n\n");
	
	}


	
}
