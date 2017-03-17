fn main() {
    // testing(5, vec![(3, 1)]);
    // testing(20, vec![(6, 2)]);
    // testing(9001, vec![(4501, 2250)]);
    // testing(9004, vec![(2252, 1125)]);
    // testing(9005, vec![(4503, 2251), (903, 449)]);
    // testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
    // testing(90002, vec![]);

    println!(" {} -- {:?}", 9000, solequa(9000));
    println!(" {} -- {:?}", 9001, solequa(9001));
    println!(" {} -- {:?}", 9002, solequa(9002));
    println!(" {} -- {:?}", 9003, solequa(9003));
    println!(" {} -- {:?}", 9004, solequa(9004));
    println!(" {} -- {:?}", 9005, solequa(9005));
    println!(" {} -- {:?}", 9006, solequa(9006));
    println!(" {} -- {:?}", 9007, solequa(9007));
    println!(" {} -- {:?}", 9008, solequa(9008));
    println!(" {} -- {:?}", 9009, solequa(9009));

}

fn q_sqrt( n:f64 ) -> u64 {
	let mut i: u64;
	let mut x2: f64;
    let mut y: f64;
	let threehalfs: f64 = 1.5;

	x2 = n * 0.5;
	y  = n;
	i  = y as u64;                       // evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );               // what the fuck?
	y  = i as f64;
	y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
//	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed

	y as u64
}

fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut ret = Vec::new();
    let mut x: u64;
    let mut y: u64;

    x = (n as f64).sqrt() as u64;

    println!("?{} -- x{}", 1/q_sqrt(n as f64), x);
    let xlimit = n / 2 + 1;

    if n % 2 == 0 {
        if x % 2 != 0 {
            x += 1;
        }
    } else {
        if x % 2 == 0 {
            x += 1;
        }
    }

    while x <= xlimit {
        let fx = x as f64;

        y = (( fx * fx - n as f64 ).sqrt() as f64 / 2.0) as u64;

        let res = (fx - 2.0 * y as f64) * (fx + 2.0 * y as f64);

        if res as u64 == n {
            ret.insert(0, (x, y));
        }

        x += 2;
    }

    ret
}

fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(solequa(n), exp)
}

#[test]
fn basics_solequa() {
    testing(20, vec![(6, 2)]);
    testing(900000, vec![(112502, 56249), (56254, 28123), (37506, 18747), (22510, 11245), (18762, 9369),
    (12518, 6241), (11270, 5615), (7530, 3735), (6286, 3107), (4550, 2225), (3810, 1845), (2590, 1205),
    (2350, 1075), (1650, 675), (1430, 535), (1150, 325), (1050, 225), (950, 25)]);

    testing(9001, vec![(4501, 2250)]);
    testing(9000001, vec![(4500001, 2250000), (73801, 36870)]);
    testing(90000001, vec![(45000001, 22500000), (6428575, 3214284), (3461545, 1730766), (494551, 247230)]);

    testing(90002, vec![]);

    testing(9004, vec![(2252, 1125)]);
    testing(9000004, vec![(2250002, 1125000), (173090, 86532), (132370, 66168), (10402, 4980)]);
    testing(90000004, vec![(22500002, 11250000), (252898, 126360), (93602, 46560), (22498, 10200)]);

    testing(9005, vec![(4503, 2251), (903, 449)]);
    testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
    testing(5, vec![(3, 1)]);
    testing(9000005, vec![(4500003, 2250001), (900003, 449999), (642861, 321427), (155187, 77579),
    (128589, 64277), (31107, 15481), (22269, 11033), (4941, 1963)]);

    testing(9009, vec![(4505, 2252), (1503, 750), (647, 320), (505, 248), (415, 202), (353, 170), (225, 102),
    (153, 60), (135, 48), (103, 20), (97, 10), (95, 2)]);

    testing(900000009, vec![(450000005, 225000002), (150000003, 75000000), (50000005, 24999998), (26470597, 13235290), (8823555, 4411752), (2941253, 1470550)]);
}

