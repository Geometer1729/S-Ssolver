use std::{
    io::{self, Read},
    str::FromStr,
};

type Coord = u16;
type Size = Coord;
#[derive(Copy,Clone,Debug,PartialEq)]
struct Pos(Coord,Coord);
#[derive(Copy,Clone,Debug)]
struct Shape(Coord,Coord);
#[derive(Copy,Clone,Debug)]
struct Rect(Pos,Shape);
#[derive(Copy,Clone,Debug)]
struct Spot(Pos,Shape);

#[derive(Clone,Debug)]
struct Grid{
    rects: Vec<Rect>,
    spots: Vec<Spot>
}

fn fits_in(l : Shape, r : Shape) -> bool {
    return l.0 <= r.0 && l.1 <= r.1;
}

fn gen_shapes(size : Size) -> Vec<Shape> {
    let mut v = vec![];
    for i in 1..32 {
        if size % i == 0 {
            v.push(Shape(i,size/i));
        }
    };
    return v;
}

fn insert (r : & Rect,mut g : Grid) -> Grid {
    let x = r.0.0;
    let y = r.0.1;
    let w = r.1.0;
    let h = r.1.1;
    let mut spots2 = vec![];
    for s in g.spots {
        if s.0 != Pos(x,y) {
            match shrink(r,s) {
                Some(s2) => spots2.push(s2),
                None => {} , // is there a thing for this?
            }
        }
    }
    g.rects.push(*r);
    g.spots = spots2;
    let g1 = add_spot(g,Pos(x+w,y));
    let g2 = add_spot(g1,Pos(x,y+h));
    return g2;
}

fn add_spot (mut g : Grid,p : Pos) -> Grid {
    let x = p.0;
    let y = p.1;
    if x < 32 && y < 32 {
        let spot = Spot(Pos(x,y),Shape(32-x,32-y));
        match shrink_all(& g.rects,spot) {
            Some(spot2) => g.spots.push(spot2),
            None => {} ,
        }
    }
    return g;
}

fn shrink_all(rs : & Vec<Rect> , s : Spot) -> Option<Spot> {
    let mut s2 = s;
    for r in rs {
        match shrink(r,s2) {
            Some(s3) => s2 = s3 ,
            None => return None ,
        }
    }
    return Some(s2);
}

fn shrink(r : & Rect,s : Spot) -> Option<Spot> {
    let rx =r.0.0;
    let ry =r.0.1;
    let rw =r.1.0;
    let rh =r.1.1;
    let sx =s.0.0;
    let sy =s.0.1;
    let sw =s.1.0;
    let sh =s.1.1;

    if rx < sx && rx + rw > sx && ry < sy+sh {
        let sp=s.0; // declaring this once above makes rust sad
        // surely guard is a thing?
        if ry > sy {
            return Some(Spot(sp,Shape(sw,ry-sy)));
        }else{
            return None;
        }
    } else if ry < sy && ry + rh > sy && rx < sx+sw {
        let sp=s.0;
        if rx > sx {
            return Some(Spot(sp,Shape(rx-sx,sh)));
        }else{
            return None;
        }
    } else {
        return Some(s);
    }
}



fn solve(sizes : Vec<Size>) -> Option<Grid> {
    let initial_grid = Grid{rects : vec![] ,spots : vec![Spot(Pos(0,0),Shape(32,32))] };
    return solve_rec(&sizes,&initial_grid);
}

fn solve_rec(sizes : & Vec<Size>,g : & Grid) -> Option<Grid> {
    if g.spots.len() == 0 {
        return None;
    }
    let spot = g.spots[0];
    let pos = spot.0;
    let space = spot.1;
    for (ind,size) in sizes.iter().enumerate() {
        for shape in gen_shapes(*size) {
            if fits_in(shape,space) {
                let rect = Rect(pos,shape);
                let g2 = g.clone();
                let g3 = insert(&rect,g2);
                let mut sizes2 = sizes.clone();
                sizes2.swap_remove(ind);
                if sizes2.len() == 0 {
                    return Some(g3);
                } else {
                    //println!("Trace:\nsizes: {:?}\ngrid:{:?}",sizes2,g3);
                    //println!("trace: {:?}",sizes2.len());
                    match solve_rec(&sizes2,&g3) {
                        Some(solution) => return Some(solution) ,
                        None => {},
                    }
                }
            }
        }
    }
    return None;
}

fn main() {

    let mut string = String::new();
    let _ = io::stdin().read_to_string(&mut string);
    let sizes = string
        .lines()
        .map(Size::from_str)
        .collect::<Result<Vec<Size>, _>>();

    match solve(sizes.unwrap()) {
        Some(g) => println!("{:?}",g),
        None => println!("cringe!"),
    }
}
